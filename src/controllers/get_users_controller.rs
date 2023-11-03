use actix_web::{get, Responder, HttpResponse};

use crate::services;

#[get("/users")]
pub async fn handler() -> impl Responder {
    println!("GET /users");

    HttpResponse::Ok().json(services::get_users_service::execute())
}
