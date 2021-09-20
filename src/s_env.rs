use std::env;
use std::sync::Arc;

use argon2::Config;
use dotenv::dotenv;
use jsonwebtoken::EncodingKey;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use rocksdb::{Options, DB};

#[derive(Clone)]
pub struct RockWrapper {
    pub(crate) db: Arc<DB>,
}

#[derive(Clone)]
pub struct Env<'a> {
    pub(crate) argon2_salt: String,
    pub(crate) argon2_config: Config<'a>,
    pub(crate) jwt_secret: String,
}

impl RockWrapper {
    pub(crate) fn init(file_path: &str) -> Self {
        let mut opts = Options::default();
        // opts.set_merge_operator_associative("test operator", RockWrapper::concat_merge);
        opts.create_if_missing(true);

        RockWrapper {
            db: Arc::new(DB::open(&opts, file_path).unwrap()),
        }
    }

    // fn concat_merge(new_key: &[u8],
    //                 existing_val: Option<&[u8]>,
    //                 operands: &mut MergeOperands)
    //                 -> Option<Vec<u8>> {
    //     println!("MERGING!!!");
    //
    //     if new_key == b"projects" {
    //         let mut result: Vec<admin::projects::Project> = bincode::deserialize(existing_val.unwrap_or(&[])).unwrap();
    //
    //         for op in operands {
    //             // let arr: Vec<admin::projects::Project> = bincode::deserialize(op).unwrap();
    //             //
    //             // for x in arr {
    //             //     result.push(x);
    //             // }
    //
    //             let val: admin::projects::Project = bincode::deserialize(op).unwrap();
    //
    //             result.push(val);
    //         }
    //
    //         return Some(bincode::serialize(&result).unwrap());
    //     }
    //
    //     panic!("UNIMPLEMENTED MERGE KEY:");
    // }
}

pub fn validate_env() -> Env<'static> {
    dotenv().ok();

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

    if argon2_salt.len() < 8 || jwt_secret.len() < 8 {
        panic!(".env VALUES ARE TOO SHORT");
    }

    Env {
        argon2_salt: argon2_salt.clone(),
        argon2_config: Config::default(),
        jwt_secret,
    }
}

pub fn init_logging() {
    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder().build("log/log.log").unwrap();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("log", Box::new(requests)))
        .logger(
            Logger::builder()
                .appender("log")
                .additive(true)
                .build("log", LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("log")
                .appender("stdout")
                .build(LevelFilter::Warn),
        )
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();

    log_panics::init();
}
