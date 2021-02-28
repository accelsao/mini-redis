use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

pub struct Server {
    ip: String,
    port: u32,
}

impl Server {
    pub fn new(ip: &str, port: u32) -> Self {
        Self {
            ip: ip.to_owned(),
            port,
        }
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        let addr = format!("{}:{}", self.ip, self.port);
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&mut self, stream: TcpStream) {
        println!("Client connected!");
        std::thread::spawn(move || {
            let mut client = Client::new(stream);
            client.read()
        });
    }
}

pub struct Client {
    stream: TcpStream
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        return Self {
            stream
        }
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        let mut buffer = [0u8; 512];
        loop {
            let n =  self.stream.read(&mut buffer)?;
            if n == 0 {
                return Ok(())
            }
            println!("{}", from_utf8(&buffer).unwrap());
        }
    }
}