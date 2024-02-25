use actix_web::web;

pub mod services;
mod controller;
pub mod models;
pub struct UsersModule;

impl UsersModule {
    pub fn build(cfg: &mut web::ServiceConfig) {
        cfg
            .service(controller::get_all)
            .service(controller::get_one)
            .service(controller::create)
            .service(controller::delete)
            .service(controller::update);
    }
}