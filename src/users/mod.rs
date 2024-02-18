use actix_web::web;

mod services;
mod controller;
mod models;
pub struct UsersModule;

impl UsersModule {
    pub fn build(cfg: &mut web::ServiceConfig) {
        cfg
            .service(controller::get_all);
    }
}