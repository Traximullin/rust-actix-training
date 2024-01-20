use crate::models::{Post};
use crate::schema::posts::dsl;
use diesel::prelude::*;
use diesel::PgConnection;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all(connection: &mut PgConnection) -> Result<Vec<Post>, DbError> {
    let posts_item = dsl::posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
  
    Ok(posts_item)
}

pub fn get_one(connection: &mut PgConnection, post_id: i32) -> Result<Post, DbError> {
    let post_item = dsl::posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)?;

    Ok(post_item)
}