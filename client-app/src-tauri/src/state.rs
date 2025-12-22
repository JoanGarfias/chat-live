use std::sync::{Arc, Mutex};
use std::net::TcpStream;

pub struct SocketState {
    pub read_stream: Option<Arc<Mutex<TcpStream>>>,
    pub write_stream: Option<Arc<Mutex<TcpStream>>>,
}

impl SocketState {
    pub fn new() -> Self {
        Self { 
            read_stream: None,
            write_stream: None,
        }
    }
}