use actix_web::web;
pub struct AuthModule;
mod services;
mod controller;

impl AuthModule {
    pub fn build(cfg: &mut web::ServiceConfig) {
        cfg
            .service(controller::create);
    }
}