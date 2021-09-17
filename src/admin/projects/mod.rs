pub mod routes;

use rocksdb::DB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct Project {
    /// Project title
    name: String,
    /// Unique identifier
    key: Uuid,
}

impl Project {
    const DB_TABLE_NAME: &'static str = "projects";

    fn list(rock: &DB) -> Vec<Project> {
        let data: Vec<Project> = match rock.get(Project::DB_TABLE_NAME) {
            Ok(Some(value)) => bincode::deserialize(&*value).unwrap(),
            Ok(None) => vec![],
            Err(e) => panic!("operational problem encountered: {}", e),
        };
        data
    }

    fn create(rock: &DB, data: Project) {
        if !rock.key_may_exist(Project::DB_TABLE_NAME) {
            rock.put(Project::DB_TABLE_NAME, bincode::serialize(&vec![data]).unwrap()).unwrap();
        } else {
            rock.merge(Project::DB_TABLE_NAME, bincode::serialize(&data).unwrap()).unwrap();
        }
    }

    fn delete(rock: &DB, data: Uuid) -> bool {
        let mut list = Project::list(&rock);

        let index = list.iter().position(|x| x.key == data);
        if index.is_none() {
            return false;
        }
        list.remove(index.unwrap());

        rock.put(Project::DB_TABLE_NAME, bincode::serialize(&list).unwrap()).unwrap();
        true
    }

    fn edit(rock: &DB, data: Project) -> bool {
        let mut list = Project::list(&rock);

        let index = list.iter().position(|x| x.key == data.key);
        if index.is_none() {
            return false;
        }
        list[index.unwrap()] = data;

        rock.put(Project::DB_TABLE_NAME, bincode::serialize(&list).unwrap()).unwrap();
        true
    }
}

