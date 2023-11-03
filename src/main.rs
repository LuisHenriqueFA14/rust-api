pub mod controllers;
pub mod services;
pub mod database;
pub mod models;
pub mod utils;
pub mod schema;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(controllers::create_user_controller::handler)
            .service(controllers::get_users_controller::handler)
            .service(controllers::get_user_controller::handler)
            .service(controllers::login_user_controller::handler)
            .service(controllers::update_user_controller::handler)
            .service(controllers::delete_user_controller::handler)
    })
        .bind((env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
             env::var("PORT").unwrap_or_else(|_| String::from("3000")).parse().unwrap()))?
        .run()
        .await
}

