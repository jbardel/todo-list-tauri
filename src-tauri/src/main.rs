// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::State;

type MuContext = Mutex<Context>;

struct Context {
    tasks: Vec<String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn add_task(name: &str, state: State<MuContext>) {
    let mut context = state.lock().unwrap();
    context.tasks.push(String::from(name));
}

#[tauri::command]
fn get_tasks(state: State<MuContext>) -> Vec<String> {
    let context = state.lock().unwrap();
    return context.tasks.clone()
}

fn main() {
    let todos = vec!["Tache 1", "Tache 2", "Tache 3", "Tache 4", "Tache 5"]
        .iter()
        .map(|e| e.to_string())
        .collect();

    tauri::Builder::default()
        .manage(Context { tasks: todos })
        .invoke_handler(tauri::generate_handler![add_task, get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
