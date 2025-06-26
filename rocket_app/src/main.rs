mod users;
mod tasks;
mod statuses;
mod assignments;

use rocket::{self, launch, routes};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

use users::*;
use tasks::*;
use statuses::*;
use assignments::*;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    rocket::build()
        .manage(pool)
        .mount("/api", routes![  //   /api/users
            get_users, get_user, create_user, update_user, delete_user,
            get_tasks, get_task, create_task, update_task, delete_task,
            get_task_statuses, get_task_status, create_task_status, update_task_status, delete_task_status,
            get_user_tasks, get_user_task, create_user_task, update_user_task, delete_user_task
        ])
}