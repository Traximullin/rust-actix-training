use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn build(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all)
        .service(get_by_id)
        .service(create)
        .service(remove)
        .service(update);
}

#[get("/posts")]
async fn get_all() -> impl Responder {
  HttpResponse::Ok().body("get_all")
}

#[get("/tweets/{id}")]
async fn get_by_id(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body("get_by_id")
}

#[post("/tweets")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("create")
}

#[delete("/tweets/{id}")]
async fn remove(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("remove")
}

#[put("/tweets/{id}")]
async fn update(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("update")
}