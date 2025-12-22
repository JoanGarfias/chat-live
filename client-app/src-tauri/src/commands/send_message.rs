use tauri::State;
use std::sync::{Mutex, Arc};
use std::io::{Write, ErrorKind};
use std::time::Duration;

use crate::SocketState;

#[tauri::command]
pub fn send_message(state: State<Mutex<SocketState>>, msg: String) -> Result<String, String> {
    // Clonar el Arc del stream para usarlo en otro thread
    let stream_arc = {
        let s = state.lock().unwrap();
        match &s.stream {
            Some(stream) => Arc::clone(stream),
            None => return Err("No hay conexión activa".into()),
        }
    };

    // Ejecutar el envío en un thread separado para no bloquear la UI
    std::thread::spawn(move || {
        let bytes = msg.as_bytes();
        let mut written = 0;
        let max_retries = 50;
        let mut retries = 0;

        while written < bytes.len() && retries < max_retries {
            let mut stream = match stream_arc.lock() {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Error al adquirir lock del stream");
                    return;
                }
            };
            
            match stream.write(&bytes[written..]) {
                Ok(n) => {
                    written += n;
                    retries = 0;
                    
                    if written == bytes.len() {
                        stream.flush().ok();
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    drop(stream);
                    std::thread::sleep(Duration::from_millis(5));
                    retries += 1;
                }
                Err(e) => {
                    eprintln!("Error al enviar mensaje: {}", e);
                    return;
                }
            }
        }

        if written < bytes.len() {
            eprintln!("Timeout al enviar mensaje");
        }
    });

    Ok("Enviando mensaje...".into())
}
