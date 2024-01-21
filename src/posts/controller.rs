use actix_web::{delete, get, post, put, web, HttpResponse, Error};
use crate::DbPool;
use super::services;
use super::models::NewPostPayload;

#[get("/posts")]
async fn get_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let mut conn = pool.get()?;
        services::get_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
  }

#[get("/posts/{id}")]
async fn get_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let post_id: i32 = path.into_inner();

    let post = web::block(move || {
        let mut conn = pool.get()?;
        services::get_one(&mut conn, post_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

#[post("/posts")]
async fn create(pool: web::Data<DbPool>, form: web::Json<NewPostPayload>) -> Result<HttpResponse, Error> {
    let results = web::block(move || {
        let mut conn = pool.get()?;

        match form.published {
            None => services::create(&mut conn, &form.title, &form.body, false),
            Some(i) => services::create(&mut conn, &form.title, &form.body, i),
        }
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))
}

#[delete("/posts/{id}")]
async fn remove(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let post_id: i32 = path.into_inner();

    web::block(move || {
        let mut conn = pool.get()?;
        services::delete(&mut conn, post_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    let temp = format!("Delete succeed post with id {}", post_id);
    Ok(HttpResponse::Ok().body(temp))
}

#[put("/posts/{id}")]
async fn update(pool: web::Data<DbPool>, path: web::Path<i32>, form: web::Json<NewPostPayload>) -> Result<HttpResponse, Error> {
    let post_id: i32 = path.into_inner();

    let results = web::block(move || {
        let mut conn = pool.get()?;

        match form.published {
            None => services::update(&mut conn, post_id, &form.title, &form.body, false),
            Some(i) => services::update(&mut conn, post_id, &form.title, &form.body, i),
        }
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))}
