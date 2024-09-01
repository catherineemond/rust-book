use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Todo {
        Todo {
            id,
            title,
            completed: false,
        }
    }
}

pub struct TodoList {
    todos: Arc<Mutex<Vec<Todo>>>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add(&self, title: String) -> Todo {
        sleep(Duration::from_millis(100)).await; // Simulate some work
        let mut todos = self.todos.lock().unwrap();
        let id = todos.len();
        let todo = Todo::new(id, title);
        todos.push(todo.clone());
        todo
    }
    // pub async fn get(&self, id: usize) -> Option<Todo> {}
    // pub async fn update(&self, id:usize, title: Option<String>, completed: Option<bool>) -> Option<Todo> {}
    // pub async fn delete(&self, id: usize) -> bool {}
}

#[tokio::main]
async fn main() {
    println!("Welcome to the todo list!");
    
    let todo_list = TodoList::new();
    let todo = todo_list.add("Learn Rust".to_string()).await;
    println!("Todo added: {} (ID: {})", todo.title, todo.id);
}
