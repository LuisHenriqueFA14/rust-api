use actix_web::{post, Responder, HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::services;

#[derive(Deserialize, Serialize)]
#[allow(private_in_public)]
struct Request {
    username: String,
    email: String,
    password: String,
}

#[post("/users")]
pub async fn handler(form: web::Json<Request>) -> impl Responder {
    println!("POST /users");

    let req = form.into_inner();

    HttpResponse::Ok().json(services::create_user_service::execute(req.username, req.email, req.password))
}
