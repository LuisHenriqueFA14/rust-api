use crate::database::database;
use crate::models::user_model::User;
use crate::utils::jwt;

use diesel::prelude::*;
use bcrypt;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserReturn {
    token: String,
}

#[derive(Serialize)]
struct Token {
    id: String,
}

pub fn execute(email: String, password: String) -> Result<UserReturn, String> {
    use crate::schema::users;

    let conn = &mut database::connect();

    let user = match users::table
        .filter(users::email.eq(&email))
        .select(User::as_select())
        .first::<User>(conn) {
        Ok(user) => user,
        Err(_) => return Err(String::from("Invalid email or password!")),
    };

    if user.email != email {
        return Err(String::from("Invalid email or password!"));
    }

    if !bcrypt::verify(password, &user.password).unwrap() {
        return Err(String::from("Invalid email or password!"));
    }

    match jwt::encode(user.id) {
        Ok(token) => Ok(UserReturn { token }),
        Err(_) => Err(String::from("Error generating token!")),
    }
}
