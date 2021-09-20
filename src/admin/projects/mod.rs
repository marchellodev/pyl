use rocksdb::DB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod routes;

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
        let list = rock.get(Project::DB_TABLE_NAME).unwrap().unwrap_or(vec![]);
        let list: Vec<Project> = bincode::deserialize(&list).unwrap_or(vec![]);

        list
    }

    // todo check if the project with this id already exists
    fn create(rock: &DB, data: Project) {
        let mut list = Project::list(&rock);
        list.push(data);

        rock.put(Project::DB_TABLE_NAME, bincode::serialize(&list).unwrap())
            .unwrap();
    }

    fn delete(rock: &DB, data: Uuid) -> bool {
        let mut list = Project::list(&rock);

        let index = list.iter().position(|x| x.key == data);
        if index.is_none() {
            return false;
        }
        list.remove(index.unwrap());

        rock.put(Project::DB_TABLE_NAME, bincode::serialize(&list).unwrap())
            .unwrap();

        true
    }

    fn edit(rock: &DB, data: Project) -> bool {
        let mut list = Project::list(&rock);

        let index = list.iter().position(|x| x.key == data.key);
        if index.is_none() {
            return false;
        }
        list[index.unwrap()] = data;

        rock.put(Project::DB_TABLE_NAME, bincode::serialize(&list).unwrap())
            .unwrap();

        true
    }
}
