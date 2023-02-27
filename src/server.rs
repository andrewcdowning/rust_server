use std::{net::TcpListener, io::Read};
use crate::http::{Request, StatusCode, Response, ParseError};
use std::convert::TryFrom;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, err: ParseError) -> Response {
        println!("failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                           
                           let response = match  Request::try_from(&buffer[..]) {
                                Ok(request) => {handler.handle_request(&request)},
                                Err(err) => {handler.handle_bad_request(err)},
                           };

                           if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
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
