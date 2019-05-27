// use diesel::mysql::MysqlConnection;
use schema::tasks;

// #[table_name="tasks"]
// #[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[derive(AsChangeset, Queryable, Insertable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub desc: String,
    pub content: String,
}

// impl Task {
//     pub fn create(task: Task, connection: &MysqlConnection) -> Task {
//         diesel::insert_into(tasks::table)
//             .values(&task)
//             .execute(connection)
//             .expect("Error creating new task");

//         tasks::table.order(tasks::id.desc()).first(connection).unwrap()
//     }

//     pub fn read(connection: &MysqlConnection) -> Vec<Task> {
//         tasks::table.order(tasks::id).load::<Task>(connection).unwrap()
//     }

//     pub fn update(id: i32, task: Task, connection: &MysqlConnection) -> bool {
//         diesel::update(tasks::table.find(id)).set(&task).execute(connection).is_ok()
//     }

//     pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
//         diesel::delete(tasks::table.find(id)).execute(connection).is_ok()
//     }
// }
