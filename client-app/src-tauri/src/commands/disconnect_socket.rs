use tauri::State;
use std::sync::Mutex;

use crate::SocketState;

#[tauri::command]
pub fn disconnect_socket(state: State<Mutex<SocketState>>, ip: String, port: u16) -> Result<String, String> {
    println!("Desconectando de {}:{}", ip, port);
    let mut s = state.lock().unwrap();

    if let Some(stream) = &s.stream {
        drop(stream.lock().unwrap());
        s.stream = None;
        Ok("Socket desconectado".into())
    } else {
        Err("No había conexión activa".into())
    }
}
