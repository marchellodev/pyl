use actix_web::{App, HttpServer};
use rocksdb::{DB, Options, MergeOperands};
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use argon2::Config;
use jsonwebtoken::EncodingKey;

mod router;
mod admin;

// todo critical salt can be to short
// todo have those structs in a separate file
#[derive(Clone)]
pub struct RockWrapper {
    db: Arc<DB>,
}

pub struct Env<'a> {
    argon2_salt: String,
    argon2_config: Config<'a>,
    jwt_secret: EncodingKey,
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

    dotenv().ok();

    // todo wut is this
    // todo tests pls
    let mut argon2_salt = String::from("");
    let mut jwt_secret = String::from("");

    for (key, value) in env::vars() {
        if key == "ARGON2_SALT" {
            argon2_salt = value;
        } else if key == "JWT_SECRET" {
            jwt_secret = value;
        }
    }

    if argon2_salt == "" || jwt_secret == "" {
        panic!("ARGON2_SALT OR/AND JWT_SECRET CANNOT BE NULL IN the .env file");
    }

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(Env { argon2_salt: argon2_salt.clone(), argon2_config: Config::default(), jwt_secret: EncodingKey::from_secret(jwt_secret.as_bytes()) })
            .configure(router::router)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


#[cfg(test)]
mod tests {
    use argon2::{self, Config};

    #[test]
    fn test_add() {
        let password = b"password";
        let salt = b"randomsalt123213";
        let config = Config::default();
        let hash = argon2::hash_encoded(password, salt, &config).unwrap();
        println!("{}", hash);
        let matches = argon2::verify_encoded(&hash, password).unwrap();
        assert!(matches);
    }
}

// $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdDEyMzIxMw$GfsiiLMx7lmkmU1RTjm2rKZGWS8NTidC6RA7C40kBMU
// $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdDEyMzIxMw$GfsiiLMx7lmkmU1RTjm2rKZGWS8NTidC6RA7C40kBMU
