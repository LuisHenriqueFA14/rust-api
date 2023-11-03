use crate::database::database;
use crate::models::user_model::User;
use crate::utils::validate_params::*;

use diesel::prelude::*;
use bcrypt;
use serde::Serialize;

use crate::utils::jwt::decode;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub fn execute(username: Option<String>, email: Option<String>, password: Option<String>, jwt_token: String) -> Result<Response, String> {
    let id = match decode(jwt_token) {
        Ok(id) => id,
        Err(_) => return Err(String::from("Invalid token!")),
    };

    let conn = &mut database::connect();

    use crate::schema::users;

    let user = match users::table
        .filter(users::id.eq(&id))
        .select(User::as_select())
        .first::<User>(conn) {
        Ok(user) => user,
        Err(_) => return Err(String::from("Invalid token!")),
    };

    let new_username: String = match username {
        Some(username) => {
            if !is_username(&username) {
                return Err(String::from("Invalid username!"));
            }

            username
        },
        None => user.username,
    };

    let new_email: String = match email {
        Some(email) => {
            if !is_email(&email) {
                return Err(String::from("Invalid email!"));
            }

            email
        },
        None => user.email,
    };

    let new_password: String = match password {
        Some(password) => {
            if !is_password(&password) {
                return Err(String::from("Invalid password!"));
            }

            bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
        },
        None => user.password,
    };

    match diesel::update(users::table.filter(users::id.eq(&id)))
        .set((
            users::username.eq(&new_username),
            users::email.eq(&new_email),
            users::password.eq(&new_password),
        ))
        .execute(conn) {
        Ok(_) => Ok(Response { message: String::from("User updated successfully!") }),
        Err(_) => Err(String::from("Error updating user!")),
    }


}
