use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Header};
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

#[derive(Serialize, Deserialize)]
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

                let jwt = encode(&Header::default(), &token_data, &env.jwt_secret).unwrap();
                return Some(jwt);
            }
        }
        return None;
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
}

impl UserScope {
    fn parse(data: &str) -> Option<UserScope> {
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
}
