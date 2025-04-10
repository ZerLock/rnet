//
// Example
// TCP Client
//
// Between UDP Client and TCP Client, only socket initialization will change
//

//
// Import library
// (As an example, it uses directly library's source code)
//
use std::io::Write;
use std::net::TcpListener;
use std::time::Instant;
use crate::client::client::Client;
use crate::client::tcp::TcpClient;
use crate::client::udp::UdpClient;
use crate::logic::client::ClientLogic;
use crate::shared::packet::Packet;

pub mod client {
    pub mod client;
    pub mod udp;
    pub mod tcp;
}

pub mod server {
    pub mod server;
    pub mod udp;
    pub mod tcp;
}

pub mod logic {
    pub mod client;
    pub mod server;
}

pub mod shared {
    pub mod packet;
}

//
// Implement ClientLogic using your own struct
//
pub struct TestClientLogic;

impl TestClientLogic {
    pub fn new() -> Self { Self }

    fn print_flush(&self, value: &str) {
        print!("{}", value);
        std::io::stdout().flush().unwrap();
    }

    fn user_input(&self) -> std::io::Result<String> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn get_command(&self) -> std::io::Result<i8> {
        let input = self.user_input()?;
        Ok(input.parse::<i8>().unwrap_or(-1))
    }

    fn format_ms_to_hh_mm_ss(&self, ms: i32) -> String {
        let secs = ms / 1000;
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn format_client_ip_port(&self, data: String) -> String {
        let parts: Vec<&str> = data.split(":").collect();
        if parts.len() != 2 {
            return String::from("");
        }

        format!("client IP = {}, port = {}", parts[0], parts[1])
    }

    fn wrong_command(&self) -> Option<Packet> {
        println!("\nPlease try again\n");
        None
    }
}

impl ClientLogic for TestClientLogic {
    fn print_usage(&self) {
        println!("<Menu>");
        println!("1) convert text to UPPER-case");
        println!("2) get server running time");
        println!("3) get my IP address and port number");
        println!("4) get server request count");
        println!("5) exit");
    }

    fn build_request(&self) -> Option<Packet> {
        self.print_flush("Input option: ");

        match self.get_command() {
            Ok(command) => match command {
                1..=4 => {
                    let mut data: Option<String> = None;
                    if command == 1 {
                        self.print_flush("Input sentence: ");
                        data = Some(self.user_input().ok()?);
                    }
                    Some(Packet::new(command.to_string(), data))
                }
                5 => {
                    println!("Bye bye~");
                    std::process::exit(0)
                }
                _ => self.wrong_command(),
            }
            Err(_) => self.wrong_command(),
        }
    }

    fn handle_response(&self, response: Packet, rtt: i32) {
        let command = response.header.parse::<i8>().unwrap_or(-1);

        let display_result = match command {
            2 => self.format_ms_to_hh_mm_ss(response.unwrap_data().parse::<i32>().unwrap_or(0)),
            3 => self.format_client_ip_port(response.unwrap_data()),
            4 => format!("requests served = {}", response.unwrap_data()),
            _ => response.unwrap_data(),
        };

        println!("\nReply from server: {}", display_result);
        println!("RTT = {:.3}\n", rtt as f64 / 1_000_000.0);
    }
}

//
// Using implemented ClientLogic and given TcpSocket
// Implement client loop logic (send, receive, errors...)
//
fn main() {
    let logic: Box<dyn ClientLogic> = Box::new(TestClientLogic::new());
    let mut socket: Box<dyn Client> = Box::new(TcpClient::new("0.0.0.0:39999"));

    socket.connect().expect("Could not connect to server");

    loop {
        logic.print_usage();

        let request = match logic.build_request() {
            Some(request) => request,
            None => continue,
        };

        match socket.send_packet(request) {
            Ok(_) => {}
            Err(_) => continue,
        }
        let sent_at = Instant::now();

        let response = match socket.receive_packet() {
            Ok(response) => response,
            Err(_) => continue,
        };

        logic.handle_response(response, sent_at.elapsed().as_nanos() as i32);
    }
}
