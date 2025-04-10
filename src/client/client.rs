use crate::shared::packet::Packet;
use std::io::Result;

pub trait Client {
    fn connect(&mut self) -> Result<()>;
    fn send(&mut self, data: &[u8]) -> Result<()>;
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize>;
    fn close(&mut self);

    fn send_packet(&mut self, packet: Packet) -> Result<()> {
        let data = packet.marshall();
        self.send(data.as_bytes())
    }

    fn receive_packet(&mut self) -> Result<Packet> {
        let mut buffer = [0; 1024];
        let size = self.receive(&mut buffer)?;
        let serialized = String::from_utf8(buffer[..size].to_owned()).unwrap();
        Ok(Packet::unmarshall(serialized))
    }
}
