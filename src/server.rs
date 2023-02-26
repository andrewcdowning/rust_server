use std::{net::TcpListener, io::Read};
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        print!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                           match  Request::try_from(&buffer[..]) {
                            Ok(request) => {},
                            Err(err) => println!("Failed to parse request: {}", err)
                           }
                            // let result: &Result<Request, _> = &buffer[..].try_into();
                        },
                        Err(_) => println!("Failed to read from socket buffer")
                    }
                },
                Err(e) => print!("Err: {}", e)
            }
        }

    }
}
