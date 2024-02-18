use super::models::User;
use crate::schema::users::dsl;
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