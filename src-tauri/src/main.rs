// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_chat_app::client::{__cmd__test_client_connect, test_client_connect};
use tauri_chat_app::host::{__cmd__host_server, host_server};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            host_server,
            test_cmd,
            test_client_connect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test_cmd() {
    println!("Test ");
}
