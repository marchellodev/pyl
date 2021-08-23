use actix_web::{Responder, HttpRequest, web, HttpResponse};
use actix_web::web::Data;
use crate::RockWrapper;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rocksdb::DB;

#[derive(Serialize, Deserialize)]
struct Project {
    /// Project title
    name: String,
    /// Unique identifier
    key: Uuid,
}

impl Project {
    pub fn list(rock: &DB) -> Vec<Project> {
        let data: Vec<Project> = match rock.get(b"projects") {
            Ok(Some(value)) => bincode::deserialize(&*value).unwrap(),
            Ok(None) => vec![],
            // todo we should probably not just panic when encountering a problem
            Err(e) => panic!("operational problem encountered: {}", e),
        };
        data
    }

    pub fn add(rock: &DB, data: Project) {
        let mut existing = Project::list(&rock);
        existing.push(data);
        rock.put(b"projects", bincode::serialize(&existing).unwrap()).unwrap();
    }
}


#[derive(Deserialize)]
pub struct ProjectCreateData {
    name: String,
}

pub async fn create(info: web::Json<ProjectCreateData>, db: Data<RockWrapper>) -> impl Responder {
    println!("Creating a project record");
    println!("Project name: {}", info.name);

    let project = Project {
        name: info.name.clone(),
        key: Uuid::new_v4(),
    };
    Project::add(&db.db, project);

    info.name.clone()
}

pub async fn list(_req: HttpRequest, db: Data<RockWrapper>) -> Result<HttpResponse, actix_web::Error> {
    let list = Project::list(&db.db);

    Ok(HttpResponse::Ok().json(list))
}

pub async fn edit() -> impl Responder {
    "1321"
}

pub async fn delete() -> impl Responder {
    "123"
}
