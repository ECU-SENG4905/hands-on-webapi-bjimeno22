# Step-by-Step Guide: Building a Rocket Web API with Diesel

## 1. Prerequisites

- **REST Client**: Install the REST Client VS Code extension to your Rust profile.

@id:humao.rest-client

---

## 2. Create the Project Structure

From inside the rocket_app folder
```bash
cargo init --bin
```

---

## 3. Add Dependencies

Edit `Cargo.toml` and add:

```toml
[dependencies]
rocket = { version = "0.5.0-rc.3", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
diesel = { version = "2", features = ["sqlite"] }
tasks_db_lib = { path = "../tasks_db_lib" } # Our Diesel-based library crate
dotenvy = "0.15"
anyhow = "1"
```

---

## 4. Configure Rocket

Create a `Rocket.toml` file in the project root:

```toml
[default]
address = "127.0.0.1"
port = 8080
log_level = "normal"
workers = 2    # threads
keep_alive = 5    # seconds
limits = { form = 32768, json = 1048576 }  # bytes

[release]
address = "0.0.0.0"
port = 80
log_level = "critical"
```

---

## 5. Set Up the Database URL

Create a `.env` file in the project root:

```
DATABASE_URL=data/tasks.db
```
## 5a. Run migrations from inside the tasks_db_lib project
<span style="color:red;"><b>This step is missing in the video</b></span><br />
The tasks.db database will not exist in rocket_app/data folder until you run migrations
```rust
// from inside the tasks_db_lib project folder
diesel migration run
```
---

## 6. Organize Your Source Code

Create the following files in `src/`:

- `main.rs`
- `users.rs`
- `tasks.rs`
- `statuses.rs`
- `assignments.rs`

---

## 7. Implement the Main Application (`src/main.rs`)

```rust
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
```

---

## Understanding Rocket Endpoints and Launch Code

### Launching and Configuring Rocket
The `#[launch]` function creates and configures the Rocket instance. Here’s what happens:
- Makes the web api run async - non-blocking
- Loads environment variables (e.g., from `.env`)
- Reads configuration from `Rocket.toml`
- Sets up the database connection pool and manages it as application state
- Mounts all API routes
- Starts the web server

**Example:**
```rust
#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    rocket::build()
        .manage(pool)
        .mount("/api", routes![
            get_users, get_user, create_user, update_user, delete_user,
            get_tasks, get_task, create_task, update_task, delete_task,
            get_task_statuses, get_task_status, create_task_status, update_task_status, delete_task_status,
            get_user_tasks, get_user_task, create_user_task, update_user_task, delete_user_task
        ])
}
```
- The server will listen on the address and port specified in `Rocket.toml` (e.g., `127.0.0.1:8080`).
- All `/api/...` endpoints are now available for HTTP requests.
### Rocket Endpoints (Route Handlers)
A Rocket endpoint (route handler) is a function annotated with a route attribute like `#[get]`, `#[post]`, `#[put]`, or `#[delete]`. Each handler defines:
- The HTTP method (GET, POST, etc.)
- The path (e.g., `/users`, `/users/<id>`)
- The parameters (from the path, query, or request body)
- The return type (e.g., JSON, string, status)

**Example:**
```rust
#[get("/users/<id>")]
pub async fn get_user(id: i32, pool: &State<DbPool>) -> Option<Json<User>> {
    let mut conn = pool.get().ok()?;
    User::read(&mut conn, id).ok().flatten().map(Json)
}
```
- This handler responds to GET requests at `/users/<id>`.
- The `id` parameter is extracted from the path.
- The handler returns a JSON response with a user, or `None` if not found.

### The `routes!` Macro
The `routes!` macro takes a list of handler function names and generates a vector of Rocket `Route` structs. Each `Route` contains:
- The HTTP method (e.g., `Method::Get`)
- The URI (e.g., `/users/<id>`)
- A pointer to the handler function
- Other metadata

The routes! macro will use the function names and build actual route structs that look like the code below.  
```rust
pub struct Route {
    pub method: Method,        //  Method::Get, Method::Post, etc...
    pub uri: Origin<'static>,  //  "/users", "/users/<id>", etc...
    pub handler: Handler,      //  get_users, get_user, create_user, etc...
    // ...other fields...
}
```

**Example:**
```rust
.mount("/api", routes![get_users, get_user, create_user, update_user, delete_user])
```
This registers all the listed handlers under the `/api` path.



---

## 7b. Understanding r2d2 Connection Pooling and Rocket's State<T>

### What is r2d2?
- **r2d2** is a generic connection pool library for Rust. It manages a pool of reusable connections to a resource, such as a database.
- In web applications, creating a new database connection for every request is slow and resource-intensive. r2d2 keeps a pool of open connections, so each request can quickly borrow a connection, use it, and return it to the pool.
- Diesel provides integration with r2d2 for database connection pooling.

**How it works in Rocket:**
- You create a connection pool type:
  ```rust
  type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
  ```
- You build the pool in your `#[launch]` function and register it with `.manage(pool)`:
  ```rust
  let manager = ConnectionManager::<SqliteConnection>::new(database_url);
  let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
  rocket::build().manage(pool)
  ```
- In your route handlers, you access the pool using Rocket's `State<T>`:
  ```rust
  pub async fn get_users(pool: &State<DbPool>) -> ...
  ```
- For each request, you call `pool.get()` to borrow a connection from the pool.

### What is State<T> in Rocket?
- `State<T>` is Rocket's way of sharing application-wide state (like a database pool, config, or client) with your route handlers.
- You register a value with `.manage(value)` when building your Rocket instance.
- In any route handler, you can add `&State<T>` as a parameter to access the managed value.
- The value must be `Send + Sync + 'static` (safe to share across threads and live for the program's lifetime).

**Example:**
```rust
// In main.rs
let pool = ...; // create your r2d2 pool
rocket::build().manage(pool);

// In a handler
pub async fn get_users(pool: &State<DbPool>) -> ... {
    let mut conn = pool.get().expect("db connection");
    // use conn for queries
}
```

**Why use r2d2 and State<T>?**
- They make your application scalable and efficient by reusing expensive resources (like DB connections) and sharing them safely across requests.
- They keep your code clean and modular, separating resource management from business logic.

---

## 8. Implement Entity Modules

### Example: `src/users.rs`
Add users.rs imports, input struct, and GET /users endpoint 
```rust
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
```
Add GET /users/`<id>` endpoint
```rust
#[get("/users/<id>")]
pub async fn get_user(id: i32, pool: &State<DbPool>) -> Option<Json<User>> {
    let mut conn = pool.get().ok()?;
    User::read(&mut conn, id).ok().flatten().map(Json)
}

```
Add PUT /users/`<id>` endpoint
```rust
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
```
Add POST /users endpoint
```rust
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
```
Add DELETE /users/`<id>` endpoint
```rust
#[delete("/users/<id>")]
pub async fn delete_user(id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    User::delete(&mut conn, id).ok().map(Json)
}
```

---

## 9. Build and Run the Application

```bash
cargo run
```

Visit [http://localhost:8080/api/users](http://localhost:8080/api/users) (or other endpoints) to test.

---

## 10. Test Your API

- Use tools like [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), Postman, or `curl` to send HTTP requests to your endpoints.
- Example with `curl`:
  ```bash
  curl http://localhost:8080/api/users
  ```

---

## 11. Create Integration Tests using http file.

---

