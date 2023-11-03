use actix_web::{put, Responder, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::services;

#[derive(Deserialize, Serialize)]
#[allow(private_in_public)]
struct Request {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[put("/users")]
pub async fn handler(form: web::Json<Request>, credentials: BearerAuth) -> impl Responder {
    println!("PUT /users");

    let req = form.into_inner();

    HttpResponse::Ok().json(services::update_user_service::execute(req.username, req.email, req.password, credentials.token().to_string()))
}
