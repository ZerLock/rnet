use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use std::io::Result;
use crate::server::server::Server;
use crate::logic::server::ServerLogic;
use crate::shared::packet::Packet;

pub struct UdpServer {
    socket: UdpSocket,
    requests_handled: u32,
    start_time: Instant,
}

impl UdpServer {
    pub fn new(bind_addr: &str) -> Self {
        let socket = UdpSocket::bind(bind_addr).unwrap();
        Self { socket, requests_handled: 0, start_time: Instant::now() }
    }
}

impl UdpServer {
    fn send(&mut self, addr: &str, data: &[u8]) -> Result<()> {
        self.socket.send_to(data, addr)?;
        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.socket.recv_from(buffer)
    }

    fn send_packet(&mut self, addr: &str, packet: Packet) -> Result<()> {
        let data = packet.marshall();
        self.send(addr, data.as_bytes())
    }

    fn receive_packet(&mut self) -> Result<(String, Packet)> {
        let mut buffer = [0; 512];
        let (size, addr) = self.receive(&mut buffer)?;
        let serialized = String::from_utf8(buffer[..size].to_owned()).unwrap();
        Ok((addr.to_string(), Packet::unmarshall(serialized)))
    }
}

impl Server for UdpServer {
    fn run(&mut self, logic: Box<dyn ServerLogic>) {
        loop {
            let (addr, request) = match self.receive_packet() {
                Ok((addr, packet)) => (addr, packet),
                Err(_) => continue,
            };

            println!("Connection requested from {}", addr);

            self.requests_handled += 1;

            let response = match logic.handle_request(request) {
                Some(response) => response,
                None => continue,
            };

            println!("Command {}", response.header);
            match self.send_packet(&response.header.clone(), response) {
                Ok(_) => (),
                Err(_) => continue,
            };
        }
    }
}
