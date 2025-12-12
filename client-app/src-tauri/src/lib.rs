// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

use std::sync::{Arc, Mutex};

// Estado temporal del socket (sin implementación de SocketClient por ahora)
pub struct SocketState(pub Arc<Mutex<Option<String>>>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SocketState(Arc::new(Mutex::new(None))))
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::connect_socket::connect_socket,
            commands::send_message::send_message,
            commands::disconnect_socket::disconnect_socket
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
