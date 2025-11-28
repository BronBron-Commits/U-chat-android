#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;
mod models; // only if you use models; harmless otherwise

use commands::auth::login;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri");
}
