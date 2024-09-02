use std::error::Error;
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use dialoguer::{theme::ColorfulTheme, Input, Select};

#[derive(Clone, Debug, PartialEq)]
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

    pub async fn get(&self, id: usize) -> Option<Todo> {
        sleep(Duration::from_millis(100)).await; // Simulate some work
        let todos = self.todos.lock().unwrap();
        todos.get(id).cloned()
    }

    pub async fn update(&self, id:usize, title: Option<String>, completed: Option<bool>) -> Option<Todo> {
        sleep(Duration::from_millis(100)).await; // Simulate some work
        let mut todos = self.todos.lock().unwrap();
        if let Some(todo) = todos.get_mut(id) {
            if let Some(title) = title {
                todo.title = title;
            }
            if let Some(completed) = completed {
                todo.completed = completed;
            }
            Some(todo.clone())
        } else {
            None
        }
    }

    pub async fn delete(&self, id: usize) -> bool {
        sleep(Duration::from_millis(100)).await; // Simulate some work
        let mut todos = self.todos.lock().unwrap();
        
        if id < todos.len() {
            todos.remove(id);
            true
        } else {
            false
        }
    }

    pub async fn list_todos(&self) -> Vec<Todo> {
        sleep(Duration::from_millis(100)).await; // Simulate some work
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let todo_list = TodoList::new();
    
    loop {
        let choices = ["View todos","Add a todo", "Update a todo", "Delete a todo", "Quit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => {
                let todos = todo_list.list_todos().await;
                if todos.is_empty() {
                    println!("No todos found.");
                } else {
                    println!("Your todos:");
                    for todo in todos {
                        println!("ID: {}, Title: {}, Completed: {}", todo.id, todo.title, todo.completed);
                    }
                }
            },
            1 => println!("Not implemented yet!"),
            2 => println!("Not implemented yet!"),
            3 => println!("Not implemented yet!"),
            4 => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}
