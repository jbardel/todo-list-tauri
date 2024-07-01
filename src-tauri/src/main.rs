// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Manager, State};

type MuContext = Mutex<Context>;

struct Context {
    tasks: Vec<String>,
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn add_task(name: &str, state: State<MuContext>) {
    let mut context = state.lock().unwrap();
    context.tasks.push(String::from(name));
}

#[tauri::command]
fn get_tasks(state: State<MuContext>) -> Vec<String> {
    let context = state.lock().unwrap();
    return context.tasks.clone();
}

fn main() {
    let todos = vec!["Tache 1", "Tache 2", "Tache 3", "Tache 4", "Tache 5"]
        .iter()
        .map(|e| e.to_string())
        .collect();

    tauri::Builder::default()
        .manage(Context { tasks: todos })
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // initialize your app here instead of sleeping :)
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(5));
                println!("Done initializing.");

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_task,
            get_tasks,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
