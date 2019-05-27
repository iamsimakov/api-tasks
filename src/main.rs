#![feature(custom_attribute)]
#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;

#[database("mysql_db")]
struct DbConn(diesel::MysqlConnection);

pub mod schema;
pub mod task;

use task::Task;
use rocket_contrib::json::{Json, JsonValue};
use diesel::prelude::*;

#[get("/")]
fn index(conn: DbConn) -> JsonValue {
    let tasks = schema::tasks::table.order(schema::tasks::id)
                                    .load::<Task>(&*conn)
                                    .unwrap();
    json!(tasks)
}

fn main() {
    rocket::ignite()
       .attach(DbConn::fairing())
       .mount("/", routes![index])
       .launch();
}
