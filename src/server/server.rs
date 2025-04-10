use crate::shared::packet::Packet;
use std::io::Result;
use std::net::SocketAddr;

pub trait Server {
    fn send(&mut self, addr: &str, data: &[u8]) -> Result<()>;
    fn receive(&mut self, buffer: &mut [u8]) -> Result<(usize, SocketAddr)>;

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
