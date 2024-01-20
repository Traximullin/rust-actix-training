use actix_web::{delete, get, post, put, web, HttpResponse, Error, Responder};
use diesel::dsl;
use crate::DbPool;
use super::controller;

pub fn build(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all)
        .service(get_by_id)
        .service(create)
        .service(remove)
        .service(update);
}

#[get("/posts")]
async fn get_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let mut conn = pool.get()?;
        controller::get_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
  }

#[get("/tweets/{id}")]
async fn get_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let post_id: i32 = path.into_inner();

    let post = web::block(move || {
        let mut conn = pool.get()?;
        controller::get_one(&mut conn, post_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
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