use crate::SocketState;

#[tauri::command]
pub fn connect_socket(state: tauri::State<SocketState>, ip: String, port: u16) {
    // TODO: Implementar conexión al socket cuando el módulo socket esté disponible
    println!("Conectando a {}:{}", ip, port);
    *state.0.lock().unwrap() = Some(format!("{}:{}", ip, port));
}