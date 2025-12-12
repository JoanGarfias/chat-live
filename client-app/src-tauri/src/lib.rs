// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

use std::sync::{Arc, Mutex};
use std::net::TcpStream;

// Estado del socket con TcpStream
pub struct SocketState {
    pub stream: Option<Arc<Mutex<TcpStream>>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SocketState { stream: None }))
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::connect_socket::connect_socket,
            commands::send_message::send_message,
            commands::disconnect_socket::disconnect_socket
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
