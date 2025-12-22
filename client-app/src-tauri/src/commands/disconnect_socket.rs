use tauri::State;
use std::sync::Mutex;

use crate::SocketState;

#[tauri::command]
pub fn disconnect_socket(state: State<Mutex<SocketState>>, ip: String, port: u16) -> Result<String, String> {
    println!("Desconectando de {}:{}", ip, port);
    let mut s = state.lock().unwrap();

    let read_exists = s.read_stream.is_some();
    let write_exists = s.write_stream.is_some();

    if read_exists || write_exists {
        if let Some(stream) = &s.read_stream {
            drop(stream.lock().unwrap());
        }
        if let Some(stream) = &s.write_stream {
            drop(stream.lock().unwrap());
        }
        
        s.read_stream = None;
        s.write_stream = None;
        Ok("Socket desconectado".into())
    } else {
        Err("No había conexión activa".into())
    }
}
