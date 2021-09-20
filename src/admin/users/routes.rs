use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::admin::users::{User, UserScope};
use crate::s_env::{Env, RockWrapper};

#[derive(Deserialize)]
pub struct LoginData {
    login: String,
    password: String,
}

pub fn login(env: Data<Env>, db: Data<RockWrapper>, data: web::Json<LoginData>) -> HttpResponse {
    let result = User::login(&db.db, &env, &data.login, &data.password);

    match result {
        None => HttpResponse::Unauthorized().finish(),
        Some(data) => HttpResponse::Ok().json(json!({ "token": data })),
    }
}

#[derive(Deserialize)]
pub struct CreateData {
    login: String,
    password: String,
    scope: String,
}

pub fn create(env: Data<Env>, db: Data<RockWrapper>, data: web::Json<CreateData>) -> HttpResponse {
    let scope = UserScope::parse(&data.scope);

    if scope.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let created = User::create(&db.db, &env, &data.login, &data.password, UserScope::All);

    if !created {
        return HttpResponse::Conflict().finish();
    }

    HttpResponse::Ok().finish()
}
