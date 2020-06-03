use actix_web::{http, web, Error, HttpResponse};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    name: String,
}

pub async fn index(
    user: web::Query<User>,
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || crate::service::index(&user.name, pool))
        .await
        .map(|_result| HttpResponse::Ok().body("Success"))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn get(
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || crate::service::get(pool))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}
