use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use crate::admin::projects::Project;
use crate::s_env::RockWrapper;

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

// TODO async?
pub async fn delete(db: Data<RockWrapper>, data: web::Json<ProjectDeleteData>) -> HttpResponse {
    let uuid = Uuid::parse_str(data.key.as_str());

    if uuid.is_err() || !Project::delete(&db.db, uuid.unwrap()) {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}
