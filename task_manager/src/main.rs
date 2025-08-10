use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    priority: Priority,
}

impl Task {
    fn new(id: u32, title: String, description: String, priority: Priority) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
            priority,
        }
    }

    fn mark_complete(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        let status = if self.completed { "✓" } else { "✗" };
        println!(
            "ID: {} | {} | [{}] {} - {}",
            self.id, status, self.priority, self.title, self.description
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, description: String, priority: Priority) {
        let task = Task::new(self.next_id, title, description, priority);
        self.tasks.insert(self.next_id, task);
        println!("Task added with ID: {}", self.next_id);
        self.next_id += 1;
    }

    fn list_tasks(&self, show_completed: bool) {
        if self.tasks.is_empty() {
            println!("No tasks found!");
            return;
        }

        println!("\n--- Your Tasks ---");
        for task in self.tasks.values() {
            if show_completed || !task.completed {
                task.display();
            }
        }
        println!();
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.mark_complete();
                println!("Task {} marked as complete!", id);
                Ok(())
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.remove(&id) {
            Some(_) => {
                println!("Task {} deleted!", id);
                Ok(())
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        println!("Tasks saved to {}", filename);
        Ok(())
    }

    fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(filename)?;
        let manager: TaskManager = serde_json::from_str(&data)?;
        Ok(manager)
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_priority() -> Priority {
    loop {
        println!("Priority levels: 1=Low, 2=Medium, 3=High");
        let input = get_input("Enter priority (1-3): ");
        match input.as_str() {
            "1" => return Priority::Low,
            "2" => return Priority::Medium,
            "3" => return Priority::High,
            _ => println!("Invalid input! Please enter 1, 2, or 3."),
        }
    }
}

fn show_menu() {
    println!("\n=== Personal Task Manager ===");
    println!("1. Add new task");
    println!("2. List all tasks");
    println!("3. List pending tasks only");
    println!("4. Complete a task");
    println!("5. Delete a task");
    println!("6. Save tasks to file");
    println!("7. Load tasks from file");
    println!("8. Exit");
    println!("===============================");
}

fn main() {
    let mut manager = TaskManager::new();
    let save_file = "tasks.json";

    // Try to load existing tasks
    if let Ok(loaded_manager) = TaskManager::load_from_file(save_file) {
        manager = loaded_manager;
        println!("Loaded existing tasks from {}", save_file);
    }

    loop {
        show_menu();
        let choice = get_input("Enter your choice (1-8): ");

        match choice.as_str() {
            "1" => {
                let title = get_input("Task title: ");
                let description = get_input("Task description: ");
                let priority = get_priority();
                manager.add_task(title, description, priority);
            }
            "2" => {
                manager.list_tasks(true);
            }
            "3" => {
                manager.list_tasks(false);
            }
            "4" => {
                manager.list_tasks(false);
                let id_str = get_input("Enter task ID to complete: ");
                if let Ok(id) = id_str.parse::<u32>() {
                    if let Err(e) = manager.complete_task(id) {
                        println!("Error: {}", e);
                    }
                } else {
                    println!("Invalid ID format!");
                }
            }
            "5" => {
                manager.list_tasks(true);
                let id_str = get_input("Enter task ID to delete: ");
                if let Ok(id) = id_str.parse::<u32>() {
                    if let Err(e) = manager.delete_task(id) {
                        println!("Error: {}", e);
                    }
                } else {
                    println!("Invalid ID format!");
                }
            }
            "6" => {
                if let Err(e) = manager.save_to_file(save_file) {
                    println!("Error saving file: {}", e);
                }
            }
            "7" => match TaskManager::load_from_file(save_file) {
                Ok(loaded_manager) => {
                    manager = loaded_manager;
                    println!("Tasks loaded successfully!");
                }
                Err(e) => println!("Error loading file: {}", e),
            },
            "8" => {
                println!("Saving tasks and exiting...");
                let _ = manager.save_to_file(save_file);
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter a number from 1-8.");
            }
        }
    }
}

// Add this to your Cargo.toml file:
/*
[package]
name = "task_manager"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
*/
