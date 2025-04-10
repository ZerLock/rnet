use crate::shared::packet::Packet;

pub trait ServerLogic {
    fn handle_request(&self, packet: Packet) -> Option<Packet>;
}
