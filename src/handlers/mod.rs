pub mod routes;

use crate::common::error::AppError;
use crate::models::{CreateTodoRequest, Todo, TodoResponse, UpdateTodoRequest};
use crate::db::Store;
use std::convert::Infallible;
use warp::{http::StatusCode, Reply};
use tracing::{info, warn};

pub async fn get_todos<S: Store>(store: S) -> Result<impl Reply, Infallible> {
    info!("모든 할 일 조회 요청");
    let todos = store.get_all().await;
    let response = TodoResponse::new(todos);
    Ok(warp::reply::json(&response))
}

pub async fn get_todo_by_id<S: Store>(
    id: String,
    store: S,
) -> Result<impl Reply, warp::Rejection> {
    info!("할 일 조회 요청: {}", id);
    
    match store.get_by_id(&id).await {
        Some(todo) => {
            info!("할 일 조회 성공: {}", id);
            Ok(warp::reply::json(&todo))
        }
        None => {
            warn!("할 일을 찾을 수 없음: {}", id);
            Err(warp::reject::custom(AppError::NotFound))
        }
    }
}

pub async fn create_todo<S: Store>(
    request: CreateTodoRequest,
    store: S,
) -> Result<impl Reply, warp::Rejection> {
    info!("새 할 일 생성 요청: {}", request.title);
    
    // 유효성 검사
    if let Err(msg) = request.validate() {
        warn!("유효성 검사 실패: {}", msg);
        return Err(warp::reject::custom(AppError::InvalidInput(msg)));
    }

    let todo = Todo::new(request.title.clone());
    let created_todo = store.create(todo).await;

    info!("할 일 생성 완료: {} - {}", created_todo.id, created_todo.title);
    Ok(warp::reply::with_status(
        warp::reply::json(&created_todo),
        StatusCode::CREATED,
    ))
}

pub async fn update_todo<S: Store>(
    id: String,
    request: UpdateTodoRequest,
    store: S,
) -> Result<impl Reply, warp::Rejection> {
    info!("할 일 업데이트 요청: {}", id);
    
    let existing_todo = match store.get_by_id(&id).await {
        Some(todo) => todo,
        None => {
            warn!("업데이트할 할 일을 찾을 수 없음: {}", id);
            return Err(warp::reject::custom(AppError::NotFound));
        }
    };

    // 업데이트할 필드만 변경
    let updated_todo = Todo {
        id: existing_todo.id.clone(),
        title: request.title.unwrap_or(existing_todo.title),
        completed: request.completed.unwrap_or(existing_todo.completed),
        created_at: existing_todo.created_at,
    };

    match store.update(&id, updated_todo).await {
        Some(todo) => {
            info!("할 일 업데이트 완료: {}", id);
            Ok(warp::reply::json(&todo))
        }
        None => {
            warn!("할 일 업데이트 실패: {}", id);
            Err(warp::reject::custom(AppError::NotFound))
        }
    }
}

pub async fn delete_todo<S: Store>(
    id: String,
    store: S,
) -> Result<impl Reply, warp::Rejection> {
    info!("할 일 삭제 요청: {}", id);
    
    if store.delete(&id).await {
        info!("할 일 삭제 완료: {}", id);
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "message": "Todo deleted successfully",
                "id": id
            })),
            StatusCode::OK,
        ))
    } else {
        warn!("삭제할 할 일을 찾을 수 없음: {}", id);
        Err(warp::reject::custom(AppError::NotFound))
    }
}