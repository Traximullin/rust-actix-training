use actix_web::{delete, get, post, put, web, HttpResponse, Error};
use crate::DbPool;
use super::services;

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