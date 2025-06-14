use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use warp::Filter;

type Db = Arc<Mutex<HashMap<String, Todo>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    // GET /todos
    let get_todos = warp::path("todos")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(get_todos_handler);

    // POST /todos
    let create_todo = warp::path("todos")
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_todo_handler);

    // GET /health
    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    let routes = health
        .or(get_todos)
        .or(create_todo)
        .with(cors);

    println!("🚀 Server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn get_todos_handler(db: Db) -> Result<impl warp::Reply, std::convert::Infallible> {
    let todos = db.lock().unwrap();
    let todos_list: Vec<Todo> = todos.values().cloned().collect();
    Ok(warp::reply::json(&todos_list))
}

async fn create_todo_handler(
    create_todo: CreateTodo,
    db: Db,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let id = Uuid::new_v4().to_string();
    let todo = Todo {
        id: id.clone(),
        title: create_todo.title,
        completed: false,
    };

    let mut todos = db.lock().unwrap();
    todos.insert(id, todo.clone());

    Ok(warp::reply::with_status(
        warp::reply::json(&todo),
        warp::http::StatusCode::CREATED,
    ))
}