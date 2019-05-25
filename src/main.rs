// #![feature(plugin)]
// #![plugin(rocket_codegen)]
// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;
// #[macro_use] extern crate diesel;

// // #[cfg(test)] mod tests;

// use rocket::fairing::AdHoc;

// mod db;
// mod schema;


// #[post("/", data = "<task>")]
// fn create(task: Json<Task>, connection: db::Connection) -> Json<Task> {
//     let insert = Task { id: None, ..task.into_inner() };
//     Json(Task::create(insert, &connection))
// }



// #[put("/<id>", data = "<task>")]
// fn update(id: i32, task: Json<Task>, connection: db::Connection) -> Json<JsonValue> {
//     let update = Task { id: Some(id), ..task.into_inner() };
//     Json(json!({
//         "success": Task::update(id, update, &connection)
//     }))
// }

// #[delete("/<id>")]
// fn delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
//     Json(json!({
//         "success": Task::delete(id, &connection)
//     }))
// }

//         .manage(db::connect())
        
//         .launch();



// fn rocket() -> Rocket {
//     rocket::ignite()
//         .attach(DbConn::fairing())
//         .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
//         .mount("/", routes![index])
//         .mount("/tasks", routes![create, update, delete])
//         .attach(Template::fairing())
// }

// fn main() {
//     rocket().launch();
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

// mod task;
// use task::Task;

use rocket_contrib::databases::diesel;
// use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json::JsonValue;

#[database("mysql_db")]
struct DbConn(diesel::MysqlConnection);

#[get("/")]
fn index(connection: DbConn) -> JsonValue {
    // Json(json!(Task::read(&connection)))
    json!({
        "id": 83,
        "values": [1, 2, 3, 4]
    })
}

// fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
//     let conn = DbConn::get_one(&rocket).expect("database connection");
//     match embedded_migrations::run(&*conn) {
//         Ok(()) => Ok(rocket),
//         Err(e) => {
//             error!("Failed to run database migrations: {:?}", e);
//             Err(rocket)
//         }
//     }
// }
// .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
fn main() {
    rocket::ignite()
       .attach(DbConn::fairing())
       .mount("/", routes![index])
       .launch();
}
