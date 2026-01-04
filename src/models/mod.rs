use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: i64,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            completed: false,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

impl CreateTodoRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if self.title.len() > 200 {
            return Err("Title too long".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub data: Vec<Todo>,
    pub count: usize,
}

impl TodoResponse {
    pub fn new(todos: Vec<Todo>) -> Self {
        let count = todos.len();
        Self {
            data: todos,
            count,
        }
    }
}