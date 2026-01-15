use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: usize, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            completed: false,
        }
    }
}
