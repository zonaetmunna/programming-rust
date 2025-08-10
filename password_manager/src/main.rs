use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::Path;

const DATA_FILE: &str = "passwords.json";

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    username: String,
    password: String,
}

type PasswordStore = HashMap<String, Entry>;

fn load_store() -> io::Result<PasswordStore> {
    if !Path::new(DATA_FILE).exists() {
        return Ok(HashMap::new());
    }
    let file = File::open(DATA_FILE)?;
    let reader = BufReader::new(file);
    let store = serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new());
    Ok(store)
}

fn save_store(store: &PasswordStore) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DATA_FILE)?;
    serde_json::to_writer_pretty(file, store)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let theme = ColorfulTheme::default();
    let mut store = load_store()?;

    loop {
        println!("\n--- Password Manager ---\n");

        let options = &[
            "Add Entry",
            "List Services",
            "View Password",
            "Delete Entry",
            "Exit",
        ];

        let choice = Select::with_theme(&theme)
            .with_prompt("Choose an action")
            .items(options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                // Add Entry
                let service: String = Input::with_theme(&theme)
                    .with_prompt("Enter service name")
                    .interact_text()
                    .unwrap();

                if service.trim().is_empty() {
                    println!("Service name cannot be empty.");
                    continue;
                }

                let username: String = Input::with_theme(&theme)
                    .with_prompt("Enter username")
                    .interact_text()
                    .unwrap();

                println!("Enter password (input will be hidden):");
                let password = read_password().unwrap_or_else(|_| "".to_string());

                if password.trim().is_empty() {
                    println!("Password cannot be empty.");
                    continue;
                }

                store.insert(service.clone(), Entry { username, password });

                save_store(&store)?;
                println!("Entry for '{}' saved.", service);
            }
            1 => {
                // List Services
                if store.is_empty() {
                    println!("No entries stored.");
                } else {
                    println!("Stored services:");
                    for service in store.keys() {
                        println!("- {}", service);
                    }
                }
            }
            2 => {
                // View Password
                if store.is_empty() {
                    println!("No entries to view.");
                    continue;
                }

                let services: Vec<String> = store.keys().cloned().collect();
                let selected = Select::with_theme(&theme)
                    .with_prompt("Select service")
                    .items(&services)
                    .default(0)
                    .interact()
                    .unwrap();

                let service = &services[selected];

                if Confirm::with_theme(&theme)
                    .with_prompt(format!("Show password for '{}'? ", service))
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    let entry = store.get(service).unwrap();
                    println!("Username: {}", entry.username);
                    println!("Password: {}", entry.password);
                } else {
                    println!("Aborted.");
                }
            }
            3 => {
                // Delete Entry
                if store.is_empty() {
                    println!("No entries to delete.");
                    continue;
                }

                let services: Vec<String> = store.keys().cloned().collect();
                let selected = Select::with_theme(&theme)
                    .with_prompt("Select service to delete")
                    .items(&services)
                    .default(0)
                    .interact()
                    .unwrap();

                let service = &services[selected];

                if Confirm::with_theme(&theme)
                    .with_prompt(format!("Delete entry for '{}'? ", service))
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    store.remove(service);
                    save_store(&store)?;
                    println!("Entry deleted.");
                } else {
                    println!("Deletion cancelled.");
                }
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
