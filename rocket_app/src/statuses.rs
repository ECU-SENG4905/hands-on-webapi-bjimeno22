use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{TaskStatus, NewTaskStatus};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct TaskStatusInput {
    pub status_name: String,
}

#[get("/tasks_statuses")]
pub async fn get_task_statuses(pool: &State<DbPool>) -> Json<Vec<TaskStatus>> {
    let mut conn = pool.get().expect("db connection");
    let task_statuses = TaskStatus::read_all(&mut conn).unwrap_or_default();
    Json(task_statuses)
}

#[get("/tasks_statuses/<id>")]
pub async fn get_task_status(id: i32, pool: &State<DbPool>) -> Option<Json<TaskStatus>> {
    let mut conn = pool.get().ok()?;
    TaskStatus::read(&mut conn, id).ok().flatten().map(Json)
}

#[put("/tasks_statuses/<id>", data = "<task_status>")]
pub async fn update_task_status(id: i32, pool: &State<DbPool>, task_status: Json<TaskStatusInput> ) -> Option<Json<TaskStatus>> {
    let mut conn = pool.get().ok()?;
    let updated_task_status = NewTaskStatus {
        status_name: &task_status.status_name,
    };
    TaskStatus::update(&mut conn, id, updated_task_status).ok().map(Json)
}

#[post("/tasks_statuses", data = "<task_status>")]
pub async fn create_task_status( pool: &State<DbPool>, task_status: Json<TaskStatusInput>) -> Option<Json<TaskStatus>> {
    let mut conn = pool.get().ok()?;
    let new_task_status = NewTaskStatus {
        status_name: &task_status.status_name,
    };
    TaskStatus::create(&mut conn, new_task_status).ok().map(Json)
}

#[delete("/tasks_statuses/<id>")]
pub async fn delete_task_status(id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    TaskStatus::delete(&mut conn, id).ok().map(Json)
}