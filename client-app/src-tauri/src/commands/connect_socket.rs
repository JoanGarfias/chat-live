use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use tauri::{AppHandle, Emitter};
use serde::Deserialize;

use crate::SocketState;

#[derive(Deserialize)]
pub struct ServerInfo {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Clone, serde::Serialize)]
pub struct Message {
    pub autor: String,
    pub mensaje: String,
}


#[tauri::command]
pub fn connect_socket(app: AppHandle, state: tauri::State<Mutex<SocketState>>, server_info: ServerInfo) -> Result<String, String> {
    let address = format!("{}:{}", server_info.ip, server_info.port);
    
    // Crear stream para lectura (no bloqueante)
    let read_stream = match TcpStream::connect(&address) {
        Ok(mut stream) => {
            // Enviar identificador de tipo de conexión
            stream.write_all(b"{\"type\":\"read\"}").ok();
            stream.set_nonblocking(true).unwrap();
            Arc::new(Mutex::new(stream))
        },
        Err(e) => return Err(format!("Error al conectar stream de lectura: {}", e))
    };
    
    // Crear stream para escritura (bloqueante)
    let write_stream = match TcpStream::connect(&address) {
        Ok(mut stream) => {
            // Enviar identificador de tipo de conexión
            stream.write_all(b"{\"type\":\"write\"}").ok();
            // Mantener bloqueante por defecto
            Arc::new(Mutex::new(stream))
        },
        Err(e) => return Err(format!("Error al conectar stream de escritura: {}", e))
    };

    // Guardar en el estado global
    {
        let mut s = state.lock().unwrap();
        s.read_stream = Some(read_stream.clone());
        s.write_stream = Some(write_stream.clone());
    }

    // Hilo de escucha (usa el stream no bloqueante)
    let app_clone = app.clone();
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];

        loop {
            let mut stream_lock = read_stream.lock().unwrap();

            match stream_lock.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let msg: Message = match serde_json::from_slice(&buffer[..size]) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Error al deserializar mensaje: {}", e);
                            continue;
                        }
                    };

                    if msg.autor == "" {
                        // Ignorar mensajes de ping del servidor
                        continue;
                    }

                    // Enviar evento al frontend
                    app_clone.emit("socket_message", msg).ok();
                },
                _ => {
                    // Evita bloquear al 100% la CPU
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
        }
    });

    Ok("Conectado al servidor Python".into())
}