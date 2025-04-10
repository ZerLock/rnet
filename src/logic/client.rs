use crate::shared::packet::Packet;

pub trait ClientLogic {
    fn print_usage(&self);
    fn build_request(&self) -> Option<Packet>;
    fn handle_response(&self, response: Packet, rtt: i32);
}
