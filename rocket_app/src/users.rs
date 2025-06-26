use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{User, NewUser};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub active: bool,
}

#[get("/users")]
pub async fn get_users(pool: &State<DbPool>) -> Json<Vec<User>> {
    let mut conn = pool.get().expect("db connection");
    let users = User::read_all(&mut conn).unwrap_or_default();
    Json(users)
}

#[get("/users/<id>")]
pub async fn get_user(id: i32, pool: &State<DbPool>) -> Option<Json<User>> {
    let mut conn = pool.get().ok()?;
    User::read(&mut conn, id).ok().flatten().map(Json)
}

#[put("/users/<id>", data = "<user>")]
pub async fn update_user(id: i32, pool: &State<DbPool>, user: Json<UserInput>) -> Option<Json<User>> {
    let mut conn = pool.get().ok()?;
    let updated_user = NewUser {
        name: &user.name,
        email: &user.email,
        active: user.active,
    };
    User::update(&mut conn, id, updated_user).ok().map(Json)
}

#[post("/users", data = "<user>")]
pub async fn create_user(pool: &State<DbPool>, user: Json<UserInput>) -> Option<Json<User>> {
    let mut conn = pool.get().ok()?;
    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
        active: user.active,
    };
    User::create(&mut conn, new_user).ok().map(Json)
}

#[delete("/users/<id>")]
pub async fn delete_user(id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    User::delete(&mut conn, id).ok().map(Json)
}