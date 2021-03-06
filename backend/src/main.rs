#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use actix_files as fs;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod handlers;
mod models;
mod schema;

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
            .service(fs::Files::new("/", "./static"))
            .service(handlers::get_user)
            .service(handlers::add_user)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
