use serde::{Deserialize, Serialize};

// (Debug, Deserialize, Serialize, Clone,

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TodoId {
    pub id: i64
}

// Query parameters for list_todos
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}