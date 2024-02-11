use actix_web::{delete, get, post, put, web, HttpResponse, Error};
use crate::DbPool;
use super::services;

#[get("/posts")]
async fn get_all() -> Result<HttpResponse, Error> {
    services::get_all();
}