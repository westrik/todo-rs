#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate todo_boilerplate;

use dotenv::dotenv;
use todo_boilerplate::db::PgConn;
use todo_boilerplate::*; // import routes

fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(PgConn::fairing())
        .mount("/api/1/", routes![list_tasks, add_task, update_task])
        .launch();
}
