use tokio::test;

#[path = "../src/main.rs"]
mod main;
use main::{Todo, TodoList};

#[test]
async fn test_new_todo() {
    let todo = Todo::new(1, "Learn Rust".to_string());
    assert_eq!(todo.title, "Learn Rust");
    assert_eq!(todo.completed, false);
}

#[test]
async fn test_todo_creation() {
    let title = "Learn Rust".to_string();
    let todo = Todo::new(1, title.clone());
    assert_eq!(todo.title, title);
    assert!(!todo.completed);
}

#[test]
async fn test_add_todo() {
    let todo_list = TodoList::new();
    let todo = todo_list.add("Learn Rust".to_string()).await;
    assert_eq!(todo.title, "Learn Rust");
    assert_eq!(todo.id, 0);
    assert!(!todo.completed);
}