use actix_web::{delete, Responder, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::services;

#[derive(Deserialize, Serialize)]
#[allow(private_in_public)]
struct Request {
    password: String,
}

#[delete("/users")]
pub async fn handler(form: web::Json<Request>, credentials: BearerAuth) -> impl Responder {
    println!("DELETE /users");

    let req = form.into_inner();

    HttpResponse::Ok().json(services::delete_user_service::execute(req.password, credentials.token().to_string()))
}
