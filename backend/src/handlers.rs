use actix_web::{get, post, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// Use super here better than crate in case of restructure the mod
// e.g. move user to a service layer
use super::actions;
use super::models;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/users/{user_id}")]
pub(crate) async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || actions::find_user_by_id(user_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_id));
        Ok(res)
    }
}

#[post("/users")]
pub(crate) async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || actions::insert_new_user(form.0, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}
