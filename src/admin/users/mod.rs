use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::s_env::Env;

pub mod routes;

#[derive(Serialize, Deserialize)]
struct User {
    login: String,
    password: String,
    scope: UserScope,
}

#[derive(Serialize, Deserialize, PartialEq)]
enum UserScope {
    /// The user can do everything: read/edit/add analytics data & read/change settings
    /// The default user `admin` is created with this value
    All,

    /// The user can only read analytics data from every project
    /// They can't add/edit projects nor see settings
    ReadAll,

    /// The user can read analytics data only from specific projects
    /// They can't read the full list of projects nor add/edit projects nor read/edit settings
    Read(Vec<Uuid>),
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAuthToken {
    login: String,
    exp: u64,
}

pub fn verify_token(rock: &DB, env: &Env, token: &str) -> bool {
    User::verify_token(&rock, &env, &token)
}

impl User {
    const DB_TABLE_NAME: &'static str = "users";

    /// Takes user login and password
    /// Returns the jwt auth token
    fn login(rock: &DB, env: &Env, login: &str, password: &str) -> Option<String> {
        let password = argon2::hash_encoded(
            password.as_bytes(),
            env.argon2_salt.as_bytes(),
            &env.argon2_config,
        )
        .unwrap();

        let users = User::list(&rock);

        for user in users {
            if user.login == login && user.password == password {
                let token_data = UserAuthToken {
                    login: String::from(login),
                    // The token works for 7 days
                    exp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        + 3600 * 24 * 7,
                };

                let jwt = encode(
                    &Header::default(),
                    &token_data,
                    &EncodingKey::from_secret(env.jwt_secret.as_bytes()),
                )
                .unwrap();
                return Some(jwt);
            }
        }
        return None;
    }

    fn verify_token(_rock: &DB, env: &Env, token: &str) -> bool {
        let token = decode::<UserAuthToken>(
            &token,
            &DecodingKey::from_secret(env.jwt_secret.as_bytes()),
            &Validation::default(),
        );
        if token.is_ok() {
            return true;
        }

        return false;
    }

    fn list(rock: &DB) -> Vec<User> {
        let list = rock.get(User::DB_TABLE_NAME).unwrap().unwrap_or(vec![]);
        let list: Vec<User> = bincode::deserialize(&list).unwrap_or(vec![]);

        list
    }

    /// If returns true, the user was created
    /// If returns false, the user was not created
    fn create(rock: &DB, env: &Env, login: &str, password: &str, scope: UserScope) -> bool {
        let mut users = User::list(&rock);

        for user in &users {
            if login == user.login {
                return false;
            }
        }

        let password = argon2::hash_encoded(
            password.as_bytes(),
            env.argon2_salt.as_bytes(),
            &env.argon2_config,
        )
        .unwrap();

        users.push(User {
            login: String::from(login),
            password,
            scope,
        });

        rock.put(User::DB_TABLE_NAME, bincode::serialize(&users).unwrap())
            .unwrap();

        return true;
    }

    fn delete(rock: &DB, login: &str) -> bool {
        let mut list = User::list(&rock);

        let index = list.iter().position(|x| x.login == login);
        if index.is_none() {
            return false;
        }
        list.remove(index.unwrap());

        rock.put(User::DB_TABLE_NAME, bincode::serialize(&list).unwrap())
            .unwrap();

        true
    }
}

impl UserScope {
    fn str_to_scope(data: &str) -> Option<UserScope> {
        if data == "all" {
            return Some(UserScope::All);
        }

        if data == "read_all" {
            return Some(UserScope::ReadAll);
        }

        if data.starts_with("read:") {
            let data = data.replacen("read:", "", 1);

            let mut list = vec![];
            for el in data.split_terminator(",") {
                let el = Uuid::parse_str(el);
                if el.is_err() {
                    return None;
                }
                list.push(el.unwrap());
            }

            return Some(UserScope::Read(list));
        }

        None
    }

    fn scope_to_str(&self) -> String {
        if *self == UserScope::All {
            return "all".to_string();
        }

        if *self == UserScope::ReadAll {
            return "read_all".to_string();
        }
        if let UserScope::Read(list) = self {
            return "read:".to_string()
                + &list
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
        }
        // TODO return and handle none instead
        return "".to_string();
    }
}
