use diesel::prelude::*;
use serde::Serialize;

use crate::database::database;
use crate::models::user_model::User;

#[derive(Serialize)]
pub struct FilteredUser {
    id: String,
    username: String,
}

pub fn execute() -> Vec<FilteredUser> {
    let conn = &mut database::connect();

    use crate::schema::users::dsl::*;

    let results: Vec<User> = users
        .limit(20)
        .load(conn)
        .expect("Error loading users!");

    let mut filtered_users: Vec<FilteredUser> = Vec::new();

    for user in results {
        let filtered_user = FilteredUser {
            id: user.id,
            username: user.username,
        };

        filtered_users.push(filtered_user);
    }

    filtered_users
}
