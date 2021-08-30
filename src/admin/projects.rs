use actix_web::{Responder, web, HttpResponse};
use actix_web::web::Data;
use crate::RockWrapper;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rocksdb::DB;

#[derive(Serialize, Deserialize)]
pub struct Project {
    /// Project title
    name: String,
    /// Unique identifier
    key: Uuid,
}

impl Project {
    pub fn list(rock: &DB) -> Vec<Project> {
        // TODO store the rocksdb key as a constant
        let data: Vec<Project> = match rock.get(b"projects") {
            Ok(Some(value)) => bincode::deserialize(&*value).unwrap(),
            Ok(None) => vec![],
            Err(e) => panic!("operational problem encountered: {}", e),
        };
        data
    }

    pub fn create(rock: &DB, data: Project) {
        if !rock.key_may_exist(b"projects") {
            rock.put(b"projects", bincode::serialize(&vec![data]).unwrap()).unwrap();
        } else {
            rock.merge(b"projects", bincode::serialize(&data).unwrap()).unwrap();
        }
    }

    pub fn delete(rock: &DB, data: Uuid) -> bool {
        let mut list = Project::list(&rock);

        let index = list.iter().position(|x| x.key == data);
        if index.is_none() {
            return false;
        }
        list.remove(index.unwrap());

        rock.put(b"projects", bincode::serialize(&list).unwrap()).unwrap();
        true
    }

    pub fn edit(rock: &DB, data: Project) -> bool {
        let mut list = Project::list(&rock);

        let index = list.iter().position(|x| x.key == data.key);
        if index.is_none() {
            return false;
        }
        list[index.unwrap()] = data;

        rock.put(b"projects", bincode::serialize(&list).unwrap()).unwrap();
        true
    }
}


#[derive(Deserialize)]
pub struct ProjectCreateData {
    name: String,
}

pub async fn create(db: Data<RockWrapper>, data: web::Json<ProjectCreateData>) -> HttpResponse {
    let project = Project {
        name: data.name.clone(),
        key: Uuid::new_v4(),
    };
    Project::create(&db.db, project);

    HttpResponse::Ok().finish()
}

pub async fn list(db: Data<RockWrapper>) -> Result<HttpResponse, actix_web::Error> {
    let list = Project::list(&db.db);

    Ok(HttpResponse::Ok().json(list))
}

pub async fn edit(db: Data<RockWrapper>, data: web::Json<Project>) -> impl Responder {
    if !Project::edit(&db.db, data.into_inner()) {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct ProjectDeleteData {
    // todo maybe UUID?
    key: String,
}

pub async fn delete(db: Data<RockWrapper>, data: web::Json<ProjectDeleteData>) -> HttpResponse {
    let uuid = Uuid::parse_str(data.key.as_str());

    if uuid.is_err() || !Project::delete(&db.db, uuid.unwrap()) {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}
