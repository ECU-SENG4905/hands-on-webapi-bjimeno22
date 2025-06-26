use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{Task, NewTask};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct TaskInput {
    pub task_name: String,
}

#[get("/tasks")]
pub async fn get_tasks(pool: &State<DbPool>) -> Json<Vec<Task>> {
    let mut conn = pool.get().expect("db connection");
    let tasks = Task::read_all(&mut conn).unwrap_or_default();
    Json(tasks)
}

#[get("/tasks/<id>")]
pub async fn get_task(id: i32, pool: &State<DbPool>) -> Option<Json<Task>> {
    let mut conn = pool.get().ok()?;
    Task::read(&mut conn, id).ok().flatten().map(Json)
}

#[put("/tasks/<id>", data = "<task>")]
pub async fn update_task(id: i32, pool: &State<DbPool>, task: Json<TaskInput>) -> Option<Json<Task>> {
    let mut conn = pool.get().ok()?;
    let updated_task = NewTask {
        task_name: &task.task_name,
    };
    Task::update(&mut conn, id, updated_task).ok().map(Json)
}

#[post("/tasks", data = "<task>")]
pub async fn create_task(pool: &State<DbPool>, task: Json<TaskInput>) -> Option<Json<Task>> {
    let mut conn = pool.get().ok()?;
    let new_task = NewTask {
        task_name: &task.task_name,
    };
    Task::create(&mut conn, new_task).ok().map(Json)
}

#[delete("/tasks/<id>")]
pub async fn delete_task(id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    Task::delete(&mut conn, id).ok().map(Json)
}