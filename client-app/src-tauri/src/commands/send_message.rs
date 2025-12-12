use tauri::State;
use std::sync::Mutex;
use std::io::Write;

use crate::SocketState;

#[tauri::command]
pub fn send_message(state: State<Mutex<SocketState>>, msg: String) -> Result<String, String> {
    let s = state.lock().unwrap();

    if let Some(ref stream_arc) = s.stream {
        let mut stream = stream_arc.lock().unwrap();

        stream.write_all(msg.as_bytes()).map_err(|_| "Error al enviar".to_string())?;

        Ok("Mensaje enviado".into())
    } else {
        Err("No hay conexión activa".into())
    }
}
