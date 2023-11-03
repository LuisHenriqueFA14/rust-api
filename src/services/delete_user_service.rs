use crate::database::database;
use crate::models::user_model::User;

use diesel::prelude::*;
use bcrypt;
use serde::Serialize;

use crate::utils::jwt::decode;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub fn execute(password: String, jwt_token: String) -> Result<Response, String> {
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

    if !bcrypt::verify(password, &user.password).unwrap() {
        return Err(String::from("Invalid password!"));
    }

    match diesel::delete(users::table.filter(users::id.eq(&id)))
        .execute(conn) {
        Ok(_) => Ok(Response {
            message: String::from("User deleted successfully!"),
        }),
        Err(_) => Err(String::from("Error deleting user!")),
    }
}
