// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_chat_app::client::{__cmd__client_connect, client_connect};
use tauri_chat_app::host::{__cmd__host_server, host_server};
use tauri_plugin_log::LogTarget;

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .filter(|metadata| metadata.target().contains("tauri_chat_app"))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            host_server,
            test_cmd,
            client_connect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test_cmd() {
    println!("Test ");
}
