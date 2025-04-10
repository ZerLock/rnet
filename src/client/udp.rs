use std::net::UdpSocket;
use std::io::Result;
use crate::client::client::Client;

pub struct UdpClient {
    socket: UdpSocket,
}

impl UdpClient {
    pub fn new(bind_addr: &str, target: &str) -> Self {
        let socket = UdpSocket::bind(bind_addr).unwrap();
        socket.connect(target).unwrap();
        Self { socket }
    }
}

impl Client for UdpClient {
    fn connect(&mut self) -> Result<()> {
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        self.socket.send(data)?;
        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.socket.recv(buffer)
    }

    fn close(&mut self) {}
}
