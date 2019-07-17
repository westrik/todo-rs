#![warn(
clippy::all,
clippy::pedantic,
// TODO: turn on `clippy::cargo` before publishing
)]
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod test_business_logic {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 1, 2);
    }
}
