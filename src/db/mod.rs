use crate::models::Todo;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::info;
use tracing::warn;

#[async_trait]
pub trait Store: Send + Sync + Clone {
    async fn get_all(&self) -> Vec<Todo>;
    async fn get_by_id(&self, id: &str) -> Option<Todo>;
    async fn create(&self, todo: Todo) -> Todo;
    async fn update(&self, id: &str, todo: Todo) -> Option<Todo>;
    async fn delete(&self, id: &str) -> bool;
    async fn count(&self) -> usize;
}

#[derive(Clone)]
pub struct InMemoryStore {
    todos: Arc<Mutex<HashMap<String, Todo>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        info!("InMemoryStore 초기화됨");
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_initial_data(todos: Vec<Todo>) -> Self {
        let mut map = HashMap::new();
        for todo in todos {
            map.insert(todo.id.clone(), todo);
        }
        
        info!("InMemoryStore 초기 데이터와 함께 생성됨: {} items", map.len());
        Self {
            todos: Arc::new(Mutex::new(map)),
        }
    }
}

#[async_trait]
impl Store for InMemoryStore {
    async fn get_all(&self) -> Vec<Todo> {
        let todos = self.todos.lock().unwrap();
        let mut result: Vec<Todo> = todos.values().cloned().collect();
        // 생성일 기준으로 정렬 (최신순)
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        info!("모든 할 일 조회: {} items", result.len());
        result
    }

    async fn get_by_id(&self, id: &str) -> Option<Todo> {
        let todos = self.todos.lock().unwrap();
        let result = todos.get(id).cloned();
        match &result {
            Some(_) => info!("할 일 조회 성공: {}", id),
            None => warn!("할 일 조회 실패: {}", id),
        }
        result
    }

    async fn create(&self, todo: Todo) -> Todo {
        let mut todos = self.todos.lock().unwrap();
        let id = todo.id.clone();
        todos.insert(id.clone(), todo.clone());
        info!("새 할 일 생성: {} - {}", id, todo.title);
        todo
    }

    async fn update(&self, id: &str, updated_todo: Todo) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        if todos.contains_key(id) {
            todos.insert(id.to_string(), updated_todo.clone());
            info!("할 일 업데이트: {} - {}", id, updated_todo.title);
            Some(updated_todo)
        } else {
            warn!("업데이트할 할 일을 찾을 수 없음: {}", id);
            None
        }
    }

    async fn delete(&self, id: &str) -> bool {
        let mut todos = self.todos.lock().unwrap();
        let result = todos.remove(id).is_some();
        if result {
            info!("할 일 삭제: {}", id);
        } else {
            warn!("삭제할 할 일을 찾을 수 없음: {}", id);
        }
        result
    }

    async fn count(&self) -> usize {
        let todos = self.todos.lock().unwrap();
        todos.len()
    }
}

pub fn create_store(_database_url: String) -> impl Store {
    InMemoryStore::new()
}