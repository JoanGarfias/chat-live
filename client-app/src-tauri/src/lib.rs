// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod state;

use std::sync::Mutex;
use state::SocketState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicializar el logger
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SocketState::new()))
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::connect_socket::connect_socket,
            commands::send_message::send_message,
            commands::disconnect_socket::disconnect_socket
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
