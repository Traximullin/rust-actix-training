use diesel::PgConnection;
use crate::users::models::User;
use crate::users::services;
use bcrypt::{hash, verify};


type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create(connection: &mut PgConnection, username: &str, password: &str) -> Result<User, DbError>{
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");

    println!("hash: {}", hash_secret);
    let hashed_password = hash_password(password);

    services::create(connection, username, &hashed_password)
}

fn hash_password(password: &str) -> String {
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");

    hashed_password
}