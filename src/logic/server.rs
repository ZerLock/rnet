use crate::shared::packet::Packet;

pub trait ClientLogic {
    fn handle_request(&self, packet: Packet) -> Option<Packet>;
}
