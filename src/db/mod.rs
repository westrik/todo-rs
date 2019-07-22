use diesel;

pub mod queries;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection, String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_result = PgConnection::establish(&database_url);
    match connection_result {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e.to_string()),
    }
}
