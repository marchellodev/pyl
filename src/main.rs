use actix_web::{App, HttpServer};
use rocksdb::{DB};
use std::sync::Arc;

mod router;
mod admin;

#[derive(Clone)]
pub struct RockWrapper {
    db: Arc<DB>,
}

impl RockWrapper {
    fn init(file_path: &str) -> Self {
        RockWrapper { db: Arc::new(DB::open_default(file_path).unwrap()) }
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = RockWrapper::init("rock");

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .configure(router::router)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
