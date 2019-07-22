#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate todo_boilerplate;

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use std::process;
use todo_boilerplate::db::establish_connection;
use todo_boilerplate::db::queries::{get_tasks, ResolutionStatus, TaskFilter};
use todo_boilerplate::models::*;

#[database("todo_db")]
struct PgConn(diesel::PgConnection);

#[get("/task", format = "json")]
fn list_tasks(conn: PgConn) -> Json<Vec<Task>> {
    let results = get_tasks(
        &*conn,
        &TaskFilter {
            resolution_status: Some(ResolutionStatus::Unresolved),
            task_id: None,
        },
    )
    .unwrap_or_else(move |err_msg: String| {
        println!("Could not get tasks - error:\n{}", err_msg);
        process::exit(1);
    });
    Json(results)
}

#[post("/task")]
fn create_task() -> &'static str {
    "created task"
}

#[patch("/task")]
fn update_task() -> &'static str {
    "completed task"
}

fn main() {
    let _conn = establish_connection().unwrap_or_else(move |err_msg: String| {
        println!("Failed to connect to database - error:\n{}", err_msg);
        process::exit(1);
    });

    rocket::ignite()
        .attach(PgConn::fairing())
        .mount("/api/1/", routes![list_tasks, create_task, update_task])
        .launch();
}
