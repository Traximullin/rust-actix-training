use crate::models::{Post, NewPost};
use crate::schema::posts::{dsl, self};
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

pub fn create(connection: &mut PgConnection, title: &str, body: &str, published: bool) -> Result<Post, DbError> {
    let new_post = NewPost {
        title,
        body,
        published
    };

    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new post");

    Ok(post)
}

pub fn delete(connection: &mut PgConnection, post_id: i32) -> Result<bool, DbError> {
    diesel::delete(
        dsl::posts.filter(
            dsl::id.eq(post_id)
        )
    ).execute(connection)?;

    Ok(true)
}

pub fn update(
    connection: &mut PgConnection,
    post_id: i32,
    title: &str,
    body: &str,
    published: bool
) -> Result<Post, DbError> {
    let updated_post = diesel::update(dsl::posts.find(post_id))
    .set((
        dsl::title.eq(title),
        dsl::body.eq(body),
        dsl::published.eq(published),
    ))
    .get_result(connection)?;

    Ok(updated_post)
}