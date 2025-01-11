use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct Receiver {
    listener: TcpListener,
}

impl Receiver {
    pub fn new(port: u16) -> Self {
        let address = format!("0.0.0.0:{port}");
        Self {
            listener: TcpListener::bind(address).expect("Couldn't create a listener"),
        }
    }

    pub fn next_request(&self) -> Option<(TcpStream, SocketAddr)> {
        self.listener.accept().ok()
    }
}
