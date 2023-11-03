use crate::database::database;
use crate::models::user_model::User;

use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct FilteredUser {
    id: String,
    username: String,
    email: String,
}

pub fn execute(id: String) -> Result<FilteredUser, String> {
    use crate::schema::users;

    let conn = &mut database::connect();

    let user = match users::table
        .filter(users::id.eq(&id))
        .select(User::as_select())
        .first::<User>(conn) {
        Ok(user) => user,
        Err(_) => return Err(String::from("User not found!")),
    };

    Ok(FilteredUser {
        id: user.id,
        username: user.username,
        email: user.email,
    })
}
