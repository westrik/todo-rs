#![deny(
clippy::all,
clippy::pedantic,
// TODO: turn on `clippy::cargo` before publishing
)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

use db::queries::*;
use db::PgConn;
use models::*;
use rocket_contrib::json::Json;
use std::process;

#[get("/task", format = "json")]
#[allow(clippy::needless_pass_by_value)]
pub fn list_tasks(conn: PgConn) -> Json<Vec<Task>> {
    let results = get_tasks(
        &*conn,
        &TaskFilter {
            resolution_status: Some(ResolutionStatus::Unresolved),
            task_id: None,
        },
    )
    .unwrap_or_else(move |err_msg: String| {
        // TODO: better error handling
        println!("Could not get tasks - error:\n{}", err_msg);
        process::exit(1);
    });
    Json(results)
}

#[post("/task")]
pub fn create_task() -> &'static str {
    // TODO: accept input + hook up to create_task
    "heh"
}

#[patch("/task")]
pub fn update_task() -> &'static str {
    // TODO: accept input + hook up to complete_task
    "heh"
}

#[cfg(test)]
mod test_routing_logic {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 1, 2);
    }
}
