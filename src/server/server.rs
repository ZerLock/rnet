use crate::logic::server::ServerLogic;

pub trait Server {
    fn run(&mut self, logic: Box<dyn ServerLogic>);
}
