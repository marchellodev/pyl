use actix_web::{App, HttpServer};

mod router;
mod admin;
mod s_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    s_env::init_logging();

    let db = s_env::RockWrapper::init("rock");
    let env = s_env::validate_env();

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(env.clone())
            .configure(router::router)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}