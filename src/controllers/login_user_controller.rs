use actix_web::{post, Responder, HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::services;

#[derive(Deserialize, Serialize)]
#[allow(private_in_public)]
struct Request {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn handler(form: web::Json<Request>) -> impl Responder {
    println!("POST /login");

    let req = form.into_inner();

    HttpResponse::Ok().json(services::login_user_service::execute(req.email, req.password))
}
