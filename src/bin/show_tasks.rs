extern crate todo_boilerplate;
extern crate diesel;

use self::todo_boilerplate::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use todo_boilerplate::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks.limit(5)
        .load::<Task>(&connection)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{}", task.description);
        println!("----------\n");
    }
}