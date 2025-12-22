use tauri::State;
use std::sync::{Mutex, Arc};
use std::io::Write;

use crate::SocketState;

#[tauri::command]
pub fn send_message(state: State<Mutex<SocketState>>, msg: String) -> Result<String, String> {
    // Clonar el Arc del stream de escritura para usarlo en otro thread
    let stream_arc = {
        let s = state.lock().unwrap();
        match &s.write_stream {
            Some(stream) => Arc::clone(stream),
            None => return Err("No hay conexión activa".into()),
        }
    };

    // Ejecutar el envío en un thread separado para no bloquear la UI
    std::thread::spawn(move || {
        let bytes = msg.as_bytes();
        
        let mut stream = match stream_arc.lock() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Error al adquirir lock del stream");
                return;
            }
        };
        
        // El socket bloqueante esperará automáticamente hasta poder escribir
        match stream.write_all(bytes) {
            Ok(_) => {
                if let Err(e) = stream.flush() {
                    eprintln!("Error al hacer flush: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error al enviar mensaje: {}", e);
            }
        }
    });

    Ok("Enviando mensaje...".into())
}
