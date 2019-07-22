extern crate diesel;
extern crate todo_boilerplate;

use self::diesel::prelude::*;
use self::models::*;
use self::todo_boilerplate::*;
use chrono::Utc;
use std::{env, process};

fn list_tasks(conn: PgConnection) {
    use todo_boilerplate::schema::tasks::dsl::*;

    let results = tasks
        .filter(resolved_at.is_null())
        .load::<Task>(&conn)
        .expect("Error loading tasks");

    println!("Listing {} unresolved tasks", results.len());
    for task in results {
        println!("{} (ID: {})", task.description, task.id);
        println!("----------");
    }
}

fn add_task(conn: PgConnection, args: &[String]) {
    use todo_boilerplate::schema::tasks::dsl::*;

    if args.len() != 3 {
        _usage();
    }

    let task = NewTask {
        description: args[2].clone(),
    };
    let created_rows = diesel::insert_into(tasks)
        .values(&task)
        .get_results::<Task>(&conn)
        .expect("Error creating task");

    println!("Added task '{}'", created_rows[0].description);
}

fn complete_task(conn: PgConnection, args: &[String]) {
    use todo_boilerplate::schema::tasks::dsl::*;

    if args.len() != 3 {
        _usage();
    }

    let task_id: i32 = args[2].trim().parse().expect("Invalid task ID");
    println!("Completing task with ID {}", task_id);

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

    let subcommand = &args[1];
    let conn = establish_connection();
    match subcommand.as_ref() {
        "list" => list_tasks(conn),
        "add" => add_task(conn, &args),
        "complete" => complete_task(conn, &args),
        _ => _usage(),
    }
}
