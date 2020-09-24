#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod schema;
mod models;
mod actions;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::get_user)
            .service(handlers::add_user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
