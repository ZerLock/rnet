use std::net::TcpStream;
use std::io::{Read, Write, Result};
use crate::client::client::Client;

pub struct TcpClient {
    stream: Option<TcpStream>,
    addr: String,
}

impl TcpClient {
    pub fn new(addr: &str) -> Self {
        Self {
            stream: None,
            addr: addr.to_string(),
        }
    }
}

impl Client for TcpClient {
    fn connect(&mut self) -> Result<()> {
        let stream = TcpStream::connect(&self.addr)?;
        self.stream = Some(stream);
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(data)?;
        }
        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize> {
        if let Some(ref mut stream) = self.stream {
            Ok(stream.read(buffer)?)
        } else {
            Ok(0)
        }
    }

    fn close(&mut self) {
        self.stream = None;
    }
}
