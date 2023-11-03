use actix_web::{get, web, Responder, HttpResponse};

use crate::services;

#[get("/users/{id}")]
pub async fn handler(id: web::Path<String>) -> impl Responder {
    println!("GET /users/{{id}}");

    HttpResponse::Ok().json(services::get_user_service::execute(id.to_string()))
}
