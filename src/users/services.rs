use super::models::{User, NewUser};
use crate::schema::users::{dsl, self};
use diesel::prelude::*;
use diesel::PgConnection;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all(connection: &mut PgConnection) -> Result<Vec<User>, DbError> {
    let users = dsl::users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");
  
    Ok(users)
}

pub fn get_one(connection: &mut PgConnection, id: i32) ->  Result<User, DbError> {
    let user = dsl::users
        .find(id)
        .select(User::as_select())
        .first(connection)?;

    Ok(user)
}

pub fn create(connection: &mut PgConnection, username: &str, password: &str) -> Result<User, DbError> {
    let new_user = NewUser {
        username,
        password
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user");

    Ok(user)
}

pub fn delete(connection: &mut PgConnection, id: i32) -> Result<bool, DbError> {
    diesel::delete(
        dsl::users.filter(
            dsl::id.eq(id)
        )
    ).execute(connection)?;

    Ok(true)
}

pub fn update(connection: &mut PgConnection, id: i32, username: &str, password: &str) -> Result<User, DbError>{
    let updated_user = diesel::update(dsl::users.find(id))
        .set((
            dsl::username.eq(username),
            dsl::password.eq(password)
        ))
        .get_result(connection)?;

    Ok(updated_user)
}