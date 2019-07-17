extern crate diesel;
extern crate todo_boilerplate;

use self::diesel::prelude::*;
use self::models::*;
use self::todo_boilerplate::*;

fn main() {
    use todo_boilerplate::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
        .limit(5)
        .load::<Task>(&connection)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{}", task.description);
        println!("----------\n");
    }
}
