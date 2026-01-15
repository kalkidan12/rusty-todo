use crate::todo::Todo;
use crate::error::{AppError, AppResult};
use std::fs;

#[derive(Default)]
pub struct TodoStorage {
    todos: Vec<Todo>,
}

impl TodoStorage {
    pub fn load(path: &str) -> AppResult<Self> {
        let data = fs::read_to_string(path)?;
        let todos = serde_json::from_str(&data)?;
        Ok(Self { todos })
    }

    pub fn save(&self, path: &str) -> AppResult<()> {
        let data = serde_json::to_string_pretty(&self.todos)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn add(&mut self, title: &str) {
        let id = self.todos.len() + 1;
        self.todos.push(Todo::new(id, title));
    }

    pub fn list(&self) {
        for todo in &self.todos {
            let status = if todo.completed { "✓" } else { " " };
            println!("[{}] {}: {}", status, todo.id, todo.title);
        }
    }

    pub fn mark_done(&mut self, id: usize) -> AppResult<()> {
        let todo = self.todos.iter_mut().find(|t| t.id == id)
            .ok_or(AppError::NotFound)?;
        todo.completed = true;
        Ok(())
    }

    pub fn remove(&mut self, id: usize) -> AppResult<()> {
        let index = self.todos.iter().position(|t| t.id == id)
            .ok_or(AppError::NotFound)?;
        self.todos.remove(index);
        Ok(())
    }
}
