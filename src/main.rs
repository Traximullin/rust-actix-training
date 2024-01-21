use actix_web:: {
    web,
    App,
    HttpServer,
};
use diesel::{
    r2d2::{self, ConnectionManager, Pool},
    PgConnection,
};
use std::env;
use dotenv::dotenv;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

mod posts;
mod schema;
mod models;
use posts::services;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(services::build)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
