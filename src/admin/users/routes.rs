use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::admin::users::{User, UserScope};
use crate::s_env::{Env, RockWrapper};

// todo edit users
// todo delete users
// todo implement user access scope

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
    let scope = UserScope::str_to_scope(&data.scope);

    if scope.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let created = User::create(&db.db, &env, &data.login, &data.password, UserScope::All);

    if !created {
        return HttpResponse::Conflict().finish();
    }

    HttpResponse::Ok().finish()
}

#[derive(Serialize)]
struct UserReadOnly {
    login: String,
    scope: String,
}

pub fn list(db: Data<RockWrapper>) -> HttpResponse {
    let data = User::list(&db.db);
    let mut result: Vec<UserReadOnly> = vec![];

    for val in data {
        result.push(UserReadOnly {
            login: val.login,
            scope: val.scope.scope_to_str(),
        })
    }

    HttpResponse::Ok().json(result)
}

#[derive(Deserialize)]
pub struct UserDeleteData {
    login: String,
}

pub fn delete(db: Data<RockWrapper>, data: web::Json<UserDeleteData>) -> HttpResponse {
    if !User::delete(&db.db, &data.login) {
        // TODO change bad request to some other error
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}
