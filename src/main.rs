use actix_web::{App, HttpServer};
use rocksdb::{DB, Options, MergeOperands};
use std::sync::Arc;

mod router;
mod admin;

#[derive(Clone)]
pub struct RockWrapper {
    db: Arc<DB>,
}

impl RockWrapper {
    fn init(file_path: &str) -> Self {
        let mut opts = Options::default();
        opts.set_merge_operator_associative("test operator", RockWrapper::concat_merge);
        opts.create_if_missing(true);

        RockWrapper { db: Arc::new(DB::open(&opts, file_path).unwrap()) }
    }

    fn concat_merge(new_key: &[u8],
                    existing_val: Option<&[u8]>,
                    operands: &mut MergeOperands)
                    -> Option<Vec<u8>> {
        println!("MERGING!!!");

        if new_key == b"projects" {
            // todo move it to the project struct ?
            let mut result: Vec<admin::projects::Project> = bincode::deserialize(existing_val.unwrap_or(&[])).unwrap();

            for op in operands {
                // let arr: Vec<admin::projects::Project> = bincode::deserialize(op).unwrap();
                //
                // for x in arr {
                //     result.push(x);
                // }

                let val: admin::projects::Project = bincode::deserialize(op).unwrap();

                result.push(val);
            }

            return Some(bincode::serialize(&result).unwrap());
        }

        panic!("UNIMPLEMENTED MERGE KEY:");
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
