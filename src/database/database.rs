use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn connect() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
