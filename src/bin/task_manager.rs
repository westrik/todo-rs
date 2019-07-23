extern crate diesel;
extern crate todo_api_rs;

use self::diesel::prelude::*;

use std::{env, process};
use todo_api_rs::db::{establish_connection, queries::*};

fn list_tasks(conn: PgConnection) {
    let results = get_tasks(
        &conn,
        &TaskFilter {
            resolution_status: Some(ResolutionStatus::Unresolved),
            task_id: None,
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
    if args.len() != 3 {
        _usage();
    }
    let task_id: i32 = args[2].trim().parse().expect("Task ID must be an integer.");
    let updated_task = resolve_task(&conn, task_id).unwrap_or_else(|err_msg: String| {
        println!("Could not complete task - error:\n{}", err_msg);
        process::exit(1);
    });
    println!(
        "Completed tasks with ID {} ('{}')",
        updated_task.id, updated_task.description
    );
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
