use uuid::Uuid;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use argon2::Config;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    name: String,
    password: String,
    scope: UserScope,
}

#[derive(Serialize, Deserialize)]
enum UserScope {
    All,
    ReadAll,
    Read(Vec<Uuid>),
}


impl User {
    /// Takes user login and password
    /// Returns the jwt auth token
    /// TODO: figure out the lifespan of the token
    pub fn login(rock: &DB, salt: String, login: String, password: String) -> Option<String> {
        // Hash the password

        let config = Config::default();
        let password = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();

        // Get the db password for the user
        let users = User::get_all_users(&rock);

        for user in users {
            if user.name == login && user.password == password {
                // TODO GENERATE THE USER TOKEN
                return Some(String::from("q23"));
            }
        }
        return None;
    }

    fn get_all_users(rock: &DB) -> Vec<User> {
        let users = rock.get("users").unwrap().unwrap_or(vec![]);
        let users: Vec<User> = bincode::deserialize(&users).unwrap_or(vec![]);

        users
    }
}