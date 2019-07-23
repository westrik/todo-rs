#![deny(
clippy::all,
clippy::pedantic,
// TODO: turn on `clippy::cargo` before publishing
)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

use db::queries::*;
use db::PgConn;
use models::*;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct Error {
    description: String,
}
#[derive(Serialize)]
pub enum TaskOrError {
    Task(Task),
    Err(Error),
}
#[derive(Serialize)]
pub enum TasksOrError {
    Tasks(Vec<Task>),
    Err(Error),
}

#[get("/task", format = "json")]
#[allow(clippy::needless_pass_by_value)] // no &PgConn
pub fn list_tasks(conn: PgConn) -> Json<TasksOrError> {
    let results = get_tasks(
        &*conn,
        &TaskFilter {
            resolution_status: Some(ResolutionStatus::Unresolved),
            task_id: None,
        },
    );
    let results = match results {
        Ok(tasks) => TasksOrError::Tasks(tasks),
        Err(e) => TasksOrError::Err(Error { description: e }),
    };
    Json(results)
}

#[post("/task", format = "json", data = "<task>")]
#[allow(clippy::needless_pass_by_value)] // no &PgConn
pub fn add_task(conn: PgConn, task: Json<NewTask>) -> Json<TaskOrError> {
    let result = create_task(&*conn, &task.description);
    let result = match result {
        Ok(tasks) => match tasks.get(0) {
            None => TaskOrError::Err(Error {
                description: "Could not create task".to_string(),
            }),
            Some(task) => TaskOrError::Task(task.to_owned()),
        },
        Err(e) => TaskOrError::Err(Error { description: e }),
    };
    Json(result)
}

#[patch("/task/<task_id>")]
#[allow(clippy::needless_pass_by_value)] // no &PgConn
pub fn update_task(conn: PgConn, task_id: i32) -> Json<TaskOrError> {
    // TODO: accept a TaskPatch body and make updates instead of resolving on PATCH.
    let result = resolve_task(&*conn, task_id);
    let result = match result {
        Ok(task) => TaskOrError::Task(task),
        Err(e) => TaskOrError::Err(Error { description: e }),
    };
    Json(result)
}

#[cfg(test)]
mod test_routing_logic {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 1, 2);
    }
}
