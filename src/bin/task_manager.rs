extern crate diesel;
extern crate todo_boilerplate;

use self::diesel::prelude::*;

use self::todo_boilerplate::*;
use chrono::Utc;
use std::{env, process};

fn list_tasks(conn: PgConnection) {
    let results = get_tasks(
        &conn,
        &TaskFilter {
            resolution_status: ResolutionStatus::Unresolved,
        },
    )
    .unwrap_or_else(move |err_msg: String| {
        println!("Could not get tasks - error:\n{}", err_msg);
        process::exit(1);
    });
    println!("Listing {} unresolved tasks", results.len());
    for task in results {
        println!("{} (ID: {})", task.description, task.id);
        println!("----------");
    }
}

fn add_task(conn: PgConnection, args: &[String]) {
    if args.len() != 3 {
        _usage();
    }
    let created_rows = create_task(&conn, &args[2]).unwrap_or_else(move |err_msg: String| {
        println!("Could not create task - error:\n{}", err_msg);
        process::exit(1);
    });
    println!("Added task '{}'", created_rows[0].description);
}

fn complete_task(conn: PgConnection, args: &[String]) {
    use todo_boilerplate::schema::tasks::dsl::*;

    if args.len() != 3 {
        _usage();
    }

    let task_id: i32 = args[2].trim().parse().expect("Invalid task ID");
    println!("Completing task with ID {}", task_id);

    // TODO: replace this with a library call with options (task_id)
    diesel::update(tasks.filter(id.eq(task_id)))
        .set(resolved_at.eq(Utc::now()))
        .execute(&conn)
        .expect("Error completing task");
}

fn _usage() {
    println!("Usage: task_manager [list|add \"<task>\"|complete <task_id>]");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        _usage();
    }
    let conn = establish_connection().unwrap_or_else(move |err_msg: String| {
        println!("Failed to connect to database - error:\n{}", err_msg);
        process::exit(1);
    });

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "list" => list_tasks(conn),
        "add" => add_task(conn, &args),
        "complete" => complete_task(conn, &args),
        _ => _usage(),
    }
}
