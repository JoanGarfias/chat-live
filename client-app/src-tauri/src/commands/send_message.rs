use crate::SocketState;

#[tauri::command]
pub fn send_message(state: tauri::State<SocketState>, message: String) {
    if let Some(connection) = state.0.lock().unwrap().as_ref() {
        // TODO: Implementar envío real al socket
        println!("Enviando mensaje a {}: {}", connection, message);
    } else {
        println!("No hay conexión establecida");
    }
}