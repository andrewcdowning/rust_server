use super::methods::Methods;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Methods
    //header: String,
    //body: String,
}