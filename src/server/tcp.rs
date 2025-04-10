use std::io::{Read, Write};
use std::net::{TcpListener};
use std::time::Instant;
use crate::logic::server::ServerLogic;
use crate::server::server::Server;
use crate::shared::packet::Packet;

pub struct TcpServer {
    listener: TcpListener,
    requests_handled: u32,
    start_time: Instant,
}

impl TcpServer {
    pub fn new(bind_addr: &str) -> Self {
        let listener = TcpListener::bind(bind_addr).unwrap();
        println!("Server listening on {}", bind_addr);
        Self { listener, requests_handled: 0, start_time: Instant::now() }
    }
}

impl Server for TcpServer {
    fn run(&mut self, logic: Box<dyn ServerLogic>) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 512];

                    loop {
                        match stream.read(&mut buffer) {
                            Ok(0) => break,
                            Ok(size) => {
                                self.requests_handled += 1;

                                let data = String::from_utf8(buffer[..size].to_owned()).unwrap();
                                let request = Packet::unmarshall(data);

                                let response = match logic.handle_request(request) {
                                    Some(response) => response,
                                    None => continue,
                                };
                                println!("Command {}", response.header);

                                match stream.write_all(response.marshall().as_bytes()) {
                                    Ok(_) => {}
                                    Err(_) => break,
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
                Err(_) => continue,
            }
        }
    }
}
