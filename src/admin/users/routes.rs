use serde::{Deserialize};
use actix_web::{web, HttpResponse};
use actix_web::web::Data;
use crate::s_env::{Env, RockWrapper};
use crate::admin::users::{UserScope, User};

#[derive(Deserialize)]
pub struct LoginData {
    login: String,
    password: String,
}

// todo review error codes
pub fn login(env: Data<Env>, db: Data<RockWrapper>, data: web::Json<LoginData>) -> HttpResponse {
    let result = User::login(&db.db, &env, &data.login, &data.password);

    match result {
        None => HttpResponse::Unauthorized().finish(),
        Some(data) => HttpResponse::Ok().json(data)
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
