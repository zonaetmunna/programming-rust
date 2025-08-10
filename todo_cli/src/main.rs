use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::Path;

const TODO_FILE: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Self {
            description,
            done: false,
        }
    }
}

fn load_tasks() -> io::Result<Vec<Task>> {
    if !Path::new(TODO_FILE).exists() {
        return Ok(Vec::new());
    }
    let file = File::open(TODO_FILE)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TODO_FILE)?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let theme = ColorfulTheme::default();
    let mut tasks = load_tasks()?;

    loop {
        println!("\n--- To-Do List ---\n");

        let options = &[
            "Add Task",
            "List Tasks",
            "Edit Task",
            "Delete Task",
            "Toggle Task Done/Undone",
            "Exit",
        ];

        let selection = Select::with_theme(&theme)
            .with_prompt("Choose an action")
            .default(0)
            .items(options)
            .interact()
            .unwrap();

        match selection {
            0 => {
                // Add Task
                let input: String = Input::with_theme(&theme)
                    .with_prompt("Enter the task description")
                    .interact_text()
                    .unwrap();
                if !input.trim().is_empty() {
                    tasks.push(Task::new(input));
                    save_tasks(&tasks)?;
                    println!("Task added.");
                } else {
                    println!("Task description cannot be empty.");
                }
            }
            1 => {
                // List Tasks
                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    for (i, task) in tasks.iter().enumerate() {
                        let status = if task.done { "[x]" } else { "[ ]" };
                        println!("{} {}: {}", status, i + 1, task.description);
                    }
                }
            }
            2 => {
                // Edit Task
                if tasks.is_empty() {
                    println!("No tasks to edit.");
                    continue;
                }
                let task_strings: Vec<String> = tasks
                    .iter()
                    .map(|t| {
                        format!(
                            "{}{}",
                            if t.done { "[x] " } else { "[ ] " },
                            t.description.clone()
                        )
                    })
                    .collect();

                let selected = Select::with_theme(&theme)
                    .with_prompt("Select task to edit")
                    .items(&task_strings)
                    .default(0)
                    .interact()
                    .unwrap();

                let new_desc: String = Input::with_theme(&theme)
                    .with_prompt("Enter new description")
                    .with_initial_text(&tasks[selected].description)
                    .interact_text()
                    .unwrap();

                if !new_desc.trim().is_empty() {
                    tasks[selected].description = new_desc;
                    save_tasks(&tasks)?;
                    println!("Task updated.");
                } else {
                    println!("Description cannot be empty.");
                }
            }
            3 => {
                // Delete Task
                if tasks.is_empty() {
                    println!("No tasks to delete.");
                    continue;
                }
                let task_strings: Vec<String> = tasks
                    .iter()
                    .map(|t| {
                        format!(
                            "{}{}",
                            if t.done { "[x] " } else { "[ ] " },
                            t.description.clone()
                        )
                    })
                    .collect();

                let selected = Select::with_theme(&theme)
                    .with_prompt("Select task to delete")
                    .items(&task_strings)
                    .default(0)
                    .interact()
                    .unwrap();

                if Confirm::with_theme(&theme)
                    .with_prompt(format!(
                        "Are you sure you want to delete task: '{}'? ",
                        tasks[selected].description
                    ))
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    tasks.remove(selected);
                    save_tasks(&tasks)?;
                    println!("Task deleted.");
                } else {
                    println!("Deletion cancelled.");
                }
            }
            4 => {
                // Toggle Done/Undone
                if tasks.is_empty() {
                    println!("No tasks to toggle.");
                    continue;
                }
                let task_strings: Vec<String> = tasks
                    .iter()
                    .map(|t| {
                        format!(
                            "{}{}",
                            if t.done { "[x] " } else { "[ ] " },
                            t.description.clone()
                        )
                    })
                    .collect();

                let selected = Select::with_theme(&theme)
                    .with_prompt("Select task to toggle done/undone")
                    .items(&task_strings)
                    .default(0)
                    .interact()
                    .unwrap();

                tasks[selected].done = !tasks[selected].done;
                save_tasks(&tasks)?;
                println!(
                    "Task '{}' marked as {}.",
                    tasks[selected].description,
                    if tasks[selected].done {
                        "done"
                    } else {
                        "not done"
                    }
                );
            }
            5 => {
                // Exit
                println!("Goodbye!");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
