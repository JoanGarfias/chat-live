use std::sync::{Arc, Mutex};
use std::net::TcpStream;

pub struct SocketState {
    pub stream: Arc<Mutex<Option<TcpStream>>>,
}

impl SocketState {
    pub fn new() -> Self {
        Self { stream: None }
    }
}