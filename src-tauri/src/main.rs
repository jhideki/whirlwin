// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;

use config::{read_config, set_data};
use std::sync::{Arc, Mutex};
use tauri::api::dialog::FileDialogBuilder;
use tauri::api::process::{Command, CommandChild};
use tauri::State;

struct CoreProcess {
    running: bool,
    child: Option<CommandChild>,
}

impl CoreProcess {
    fn new() -> Self {
        CoreProcess {
            running: false,
            child: None,
        }
    }
    fn start(&mut self) {
        if self.child.is_none() {
            let child = Command::new_sidecar("whirlwincore")
                .expect("failed to spawn sidecare")
                .spawn()
                .expect("Failed to start whirlwin");
            self.child = Some(child.1);
        } else {
            println!("whirlwin is already running");
        }
    }
    fn stop(&mut self) {
        println!("whirlwin stopped");
        if let Some(child) = self.child.take() {
            child.kill().expect("Failed to kill whirlwin");
        }
    }
}

#[tauri::command]
fn manage_core(core: State<'_, Arc<Mutex<CoreProcess>>>) -> String {
    println!("manage core called");
    let mut core = core.lock().unwrap();
    if core.running {
        core.stop();
        core.running = false;
        "Start program".to_string()
    } else {
        core.start();
        core.running = true;
        "End Program".to_string()
    }
}

#[tauri::command]
fn load_shortcut_data() -> String {
    match read_config() {
        Ok(config) => return serde_json::to_string(&config).expect("failed bud"),
        Err(e) => println!("error reading config {}", e),
    }
    "failed to read config".to_string()
}

#[tauri::command]
fn set_shortcut(shortcut_id: String) {
    let shortcut_id = match shortcut_id.parse::<usize>() {
        Ok(id) => id,
        Err(_) => {
            println!("Error parsing shortcut_id");
            return;
        }
    };

    FileDialogBuilder::new().pick_file(move |file_path| {
        if let Some(path) = file_path {
            set_data(path, shortcut_id);
        }
    })
}
fn main() {
    let core = Arc::new(Mutex::new(CoreProcess::new()));
    tauri::Builder::default()
        .manage(core)
        .invoke_handler(tauri::generate_handler![
            manage_core,
            set_shortcut,
            load_shortcut_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
