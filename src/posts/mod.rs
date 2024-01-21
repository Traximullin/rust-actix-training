use actix_web::web;

mod services;
mod controller;
mod models;

pub struct PostModule;
// Точка входа в посты, все инкапсулировано
impl PostModule{
    pub fn build(cfg: &mut web::ServiceConfig) {
        cfg
            .service(controller::get_all)
            .service(controller::get_by_id)
            .service(controller::create)
            .service(controller::remove)
            .service(controller::update);
    }
}