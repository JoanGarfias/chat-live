use crate::SocketState;

#[tauri::command]
pub fn disconnect_socket(state: tauri::State<SocketState>, ip: String, port: u16) {
    println!("Desconectando de {}:{}", ip, port);
    *state.0.lock().unwrap() = None;
}