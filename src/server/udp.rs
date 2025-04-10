use std::net::{SocketAddr, UdpSocket};
use crate::server::server::Server;

pub struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    pub fn new(bind_addr: &str) -> Self {
        let socket = UdpSocket::bind(bind_addr).unwrap();
        Self { socket }
    }
}

impl Server for UdpServer {
    fn send(&mut self, addr: &str, data: &[u8]) -> std::io::Result<()> {
        self.socket.send_to(data, addr)?;
        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        self.socket.recv_from(buffer)
    }
}
