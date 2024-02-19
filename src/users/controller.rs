use actix_web::{delete, get, post, put, web, HttpResponse, Error};
use crate::DbPool;
use super::services;
use super::models::NewUserPayload;

#[get("/users")]
async fn get_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let mut conn = pool.get()?;
        
        services::get_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn get_one(pool: web::Data<DbPool>, path: web::Path<i32>)  -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let users = web::block(move || {
        let mut conn = pool.get()?;

        services::get_one(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[post("/users")]
async fn create(pool: web::Data<DbPool>, form: web::Json<NewUserPayload>) -> Result<HttpResponse, Error> {
    let results = web::block(move || {
        let mut conn = pool.get()?;

        services::create(&mut conn, &form.username, &form.password)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))
}