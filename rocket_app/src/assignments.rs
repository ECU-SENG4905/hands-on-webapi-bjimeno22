use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{UserTask, NewUserTask};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct UserTaskInput {
    pub user_id: i32,
    pub task_id: i32,
    pub task_status_id: i32,
}



#[get("/assignments")]
pub async fn get_user_tasks(pool: &State<DbPool>) -> Json<Vec<UserTask>> {
    let mut conn = pool.get().expect("db connection");
    let user_tasks = UserTask::read_all(&mut conn).unwrap_or_default();
    Json(user_tasks)
}

#[get("/assignments/<user_id>/<task_id>")]
pub async fn get_user_task(user_id: i32, task_id: i32, pool: &State<DbPool>) -> Option<Json<UserTask>> {
    let mut conn = pool.get().ok()?;
    UserTask::read(&mut conn, (user_id, task_id)).ok().flatten().map(Json)
}

#[put("/assignments/<user_id>/<task_id>", data = "<user_task>")]
pub async fn update_user_task(user_id: i32, task_id: i32, pool: &State<DbPool>, user_task: Json<UserTaskInput>) -> Option<Json<UserTask>> {
    let mut conn = pool.get().ok()?;
    let updated_user_task = NewUserTask {
        user_id: user_task.user_id,
        task_id: user_task.task_id,
        task_status_id: user_task.task_status_id
    };
    UserTask::update(&mut conn, (user_id, task_id), updated_user_task).ok().map(Json)
}

#[post("/assignments", data = "<user_task>")]
pub async fn create_user_task(pool: &State<DbPool>, user_task: Json<UserTaskInput>) -> Option<Json<UserTask>> {
    let mut conn = pool.get().ok()?;
    let new_user_task = NewUserTask {
        user_id: user_task.user_id,
        task_id: user_task.task_id,
        task_status_id: user_task.task_status_id
    };
    UserTask::create(&mut conn, new_user_task).ok().map(Json)
}

#[delete("/assignments/<user_id>/<task_id>")]
pub async fn delete_user_task(user_id: i32, task_id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    UserTask::delete(&mut conn, (user_id, task_id)).ok().map(Json)
}