use actix_web::{delete, get, post, put, web, HttpResponse, Error};
use crate::DbPool;
use crate::users::models::NewUserPayload;
use super::services;

#[post("/registration")]
async fn create(pool: web::Data<DbPool>, form: web::Json<NewUserPayload>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut connection = pool.get()?;

        services::create(&mut connection, &form.username, &form.password)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
} 