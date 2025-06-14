use crate::error::handle_rejection;
use crate::handlers;
use crate::store::Store;
use warp::Filter;

pub fn create_routes<S: Store + 'static>(
    store: S,
) -> impl Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    // Health check
    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    // Todos routes
    let get_todos = warp::path("todos")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_store(store.clone()))
        .and_then(handlers::get_todos);

    let create_todo = warp::path("todos")
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_store(store.clone()))
        .and_then(handlers::create_todo);

    let get_todo = warp::path!("todos" / String)
        .and(warp::get())
        .and(with_store(store.clone()))
        .and_then(handlers::get_todo_by_id);

    let update_todo = warp::path!("todos" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_store(store.clone()))
        .and_then(handlers::update_todo);

    let delete_todo = warp::path!("todos" / String)
        .and(warp::delete())
        .and(with_store(store))
        .and_then(handlers::delete_todo);

    health
        .or(get_todos)
        .or(create_todo)
        .or(get_todo)
        .or(update_todo)
        .or(delete_todo)
        .with(cors)
        .recover(handle_rejection)
}

fn with_store<S: Store + 'static>(
    store: S,
) -> impl Filter<Extract = (S,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || store.clone())
}