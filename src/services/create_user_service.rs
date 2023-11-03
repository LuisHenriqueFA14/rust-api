use crate::database::database;
use crate::models::user_model::{User, NewUser};
use crate::utils::validate_params;

use diesel::prelude::*;
use uuid::Uuid;
use bcrypt::hash;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserReturn {
    id: String,
    username: String,
    email: String,
}

pub fn execute(username: String, email: String, password: String) -> Result<UserReturn, String> {
    let valid_username = validate_params::is_username(&username);
    let valid_email = validate_params::is_email(&email);
    let valid_password = validate_params::is_password(&password);

    if !valid_username || !valid_email || !valid_password {
        return Err(String::from("Invalid parameters!"));
    }

    use crate::schema::users;

    let conn = &mut database::connect();

    let email_already_exists = users::table
        .filter(users::email.eq(&email))
        .select(users::id)
        .first::<String>(conn)
        .is_ok();

    if email_already_exists {
        return Err(String::from("Email already in use!"));
    }

    let username_already_exists = users::table
        .filter(users::username.eq(&username))
        .select(users::id)
        .first::<String>(conn)
        .is_ok();

    if username_already_exists {
        return Err(String::from("Username already in use!"));
    }

    let hashed_password = hash(password.as_str(), 8).unwrap();

    let user = NewUser {
        id: &Uuid::new_v4().to_string(),
        username: username.as_str(),
        email: email.as_str(),
        password: hashed_password.as_str(),
    };

    let user: User = diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .expect("Error saving new user!");

    Ok(UserReturn {
        id: user.id,
        username: user.username,
        email: user.email,       
    })
}
