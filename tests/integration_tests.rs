use todo_service::app::db::{InMemoryStore, Store};
use todo_service::app::models::{CreateTodoRequest, Todo};

#[tokio::test]
async fn test_create_and_get_todo() {
    let store = InMemoryStore::new();
    
    // 할 일 생성
    let todo = Todo::new("테스트 할 일".to_string());
    let created_todo = store.create(todo).await;
    
    // 생성된 할 일 조회
    let found_todo = store.get_by_id(&created_todo.id).await;
    assert!(found_todo.is_some());
    assert_eq!(found_todo.unwrap().title, "테스트 할 일");
}

#[tokio::test]
async fn test_get_all_todos() {
    let store = InMemoryStore::new();
    
    // 여러 할 일 생성
    let todo1 = Todo::new("첫 번째 할 일".to_string());
    let todo2 = Todo::new("두 번째 할 일".to_string());
    
    store.create(todo1).await;
    store.create(todo2).await;
    
    let all_todos = store.get_all().await;
    assert_eq!(all_todos.len(), 2);
}

#[tokio::test]
async fn test_update_todo() {
    let store = InMemoryStore::new();
    
    let todo = Todo::new("원래 제목".to_string());
    let created_todo = store.create(todo).await;
    
    let mut updated_todo = created_todo.clone();
    updated_todo.title = "수정된 제목".to_string();
    updated_todo.completed = true;
    
    let result = store.update(&created_todo.id, updated_todo).await;
    assert!(result.is_some());
    
    let found_todo = store.get_by_id(&created_todo.id).await.unwrap();
    assert_eq!(found_todo.title, "수정된 제목");
    assert!(found_todo.completed);
}

#[tokio::test]
async fn test_delete_todo() {
    let store = InMemoryStore::new();
    
    let todo = Todo::new("삭제될 할 일".to_string());
    let created_todo = store.create(todo).await;
    
    let deleted = store.delete(&created_todo.id).await;
    assert!(deleted);
    
    let found_todo = store.get_by_id(&created_todo.id).await;
    assert!(found_todo.is_none());
}

#[tokio::test]
async fn test_count_todos() {
    let store = InMemoryStore::new();
    
    assert_eq!(store.count().await, 0);
    
    let todo = Todo::new("카운트 테스트".to_string());
    store.create(todo).await;
    
    assert_eq!(store.count().await, 1);
}

#[tokio::test]
async fn test_create_todo_request_validation() {
    // 유효한 요청
    let valid_request = CreateTodoRequest {
        title: "유효한 제목".to_string(),
    };
    assert!(valid_request.validate().is_ok());
    
    // 빈 제목
    let empty_request = CreateTodoRequest {
        title: "".to_string(),
    };
    assert!(empty_request.validate().is_err());
    
    // 너무 긴 제목
    let long_request = CreateTodoRequest {
        title: "a".repeat(201),
    };
    assert!(long_request.validate().is_err());
}