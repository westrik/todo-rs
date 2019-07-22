#![deny(
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

use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use schema::tasks::dsl::*;
use std::env;

pub enum ResolutionStatus {
    Any,
    Unresolved,
    Resolved,
}
pub struct TaskFilter {
    pub resolution_status: ResolutionStatus,
}

pub fn get_tasks(conn: &PgConnection, filter: &TaskFilter) -> Result<Vec<Task>, String> {
    let query = tasks.into_boxed();
    let query = match filter.resolution_status {
        ResolutionStatus::Any => query,
        ResolutionStatus::Unresolved => query.filter(resolved_at.is_not_null()),
        ResolutionStatus::Resolved => query.filter(resolved_at.is_null()),
    };
    let result = query.load::<Task>(conn);

    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}

pub fn establish_connection() -> Result<PgConnection, String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_result = PgConnection::establish(&database_url);
    match connection_result {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod test_database_logic {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 1, 2);
    }
}
