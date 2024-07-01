// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, sync::Mutex};

use tauri::State;

type MuContext = Mutex<Context>;

struct Context {
    tasks: HashMap<String, String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_tasks(state: State<MuContext>) -> HashMap<String, String> {
    let context = state.lock().unwrap();
    return context.tasks.clone();
}

#[tauri::command]
fn add_task(name: &str, state: State<MuContext>) {
    let mut context = state.lock().unwrap();
    context.tasks.insert(create_uuid(), String::from(name));
}

#[tauri::command]
fn delete_task(id: String, state: State<MuContext>) {
    let mut context = state.lock().unwrap();
    context.tasks.remove(&id);
}

fn main() {
    let todos = vec!["Tache 1", "Tache 2", "Tache 3", "Tache 4", "Tache 5"]
        .iter()
        .map(|e| (create_uuid(), e.to_string()))
        .collect();

    tauri::Builder::default()
        .manage(Mutex::new(Context { tasks: todos }))
        .invoke_handler(tauri::generate_handler![add_task, get_tasks, delete_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
