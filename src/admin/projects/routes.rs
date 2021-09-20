use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::admin::projects::Project;
use crate::s_env::RockWrapper;

#[derive(Deserialize)]
pub struct ProjectCreateData {
    name: String,
}
pub fn create(db: Data<RockWrapper>, data: web::Json<ProjectCreateData>) -> HttpResponse {
    let project = Project {
        name: data.name.clone(),
        key: Uuid::new_v4(),
    };
    Project::create(&db.db, project);

    HttpResponse::Ok().finish()
}

pub fn list(db: Data<RockWrapper>) -> HttpResponse {
    let list = Project::list(&db.db);

    HttpResponse::Ok().json(list)
}

pub fn edit(db: Data<RockWrapper>, data: web::Json<Project>) -> HttpResponse {
    if !Project::edit(&db.db, data.into_inner()) {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct ProjectDeleteData {
    key: Uuid,
}
pub fn delete(db: Data<RockWrapper>, data: web::Json<ProjectDeleteData>) -> HttpResponse {
    if !Project::delete(&db.db, data.key) {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}
