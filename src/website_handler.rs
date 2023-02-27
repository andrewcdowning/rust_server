use super::server::Handler;
use super::http::{Request, Response, StatusCode, Methods};
use::std::fs;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(p) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(p).ok()
                } else {
                    println!("Directory Path Traversal Attempted {}", p);
                    None
                }
            },
            Err(_) => None
        }
        
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Methods::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                "/ping" => Response::new(StatusCode::Ok, Some("pong".to_string())),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => {
                Response::new(StatusCode::NotFound, None)
            }
        }
    }
}
