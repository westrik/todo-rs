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
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use schema::tasks::dsl::*;
use std::env;

pub enum ResolutionStatus {
    Unresolved,
    Resolved,
}
// TODO: come up with a non-awkward way to build TaskFilters.
pub struct TaskFilter {
    pub resolution_status: Option<ResolutionStatus>,
    pub task_id: Option<i32>,
}

pub fn get_tasks(conn: &PgConnection, filter: &TaskFilter) -> Result<Vec<Task>, String> {
    let query = tasks.into_boxed();
    let query = match filter.resolution_status {
        Some(ResolutionStatus::Unresolved) => query.filter(resolved_at.is_not_null()),
        Some(ResolutionStatus::Resolved) => query.filter(resolved_at.is_null()),
        None => query,
    };
    let result = query.load::<Task>(conn);
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}

pub fn create_task(conn: &PgConnection, desc: &str) -> Result<Vec<Task>, String> {
    let task = NewTask {
        description: desc.to_string(),
    };
    let result = diesel::insert_into(tasks)
        .values(&task)
        .get_results::<Task>(conn);
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}

pub fn resolve_task(conn: &PgConnection, task_id: i32) -> Result<Task, String> {
    // TODO: filter out resolved tasks & fail if task is already resolved
    let result = diesel::update(tasks.filter(id.eq(task_id)))
        .set(resolved_at.eq(Utc::now()))
        .get_result(conn);
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
