use actix_web::{delete, get, post, put, web, HttpResponse, Error, Responder};
use diesel::PgConnection;
use crate::DbPool;
use crate::models::{Post};
use crate::schema::posts::dsl;
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn build(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all)
        .service(get_by_id)
        .service(create)
        .service(remove)
        .service(update);
}

pub fn get_all_published_post(connection: &mut PgConnection) -> Result<Vec<Post>, DbError> {
  let posts_item = dsl::posts
      .select(Post::as_select())
      .load(connection)
      .expect("Error loading posts");

  Ok(posts_item)
}

#[get("/posts")]
async fn get_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let mut conn = pool.get()?;
        get_all_published_post(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
  }

#[get("/tweets/{id}")]
async fn get_by_id(_id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body("get_by_id")
}

#[post("/tweets")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("create")
}

#[delete("/tweets/{id}")]
async fn remove(_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("remove")
}

#[put("/tweets/{id}")]
async fn update(_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("update")
}