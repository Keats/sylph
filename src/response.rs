use hyper::status::StatusCode;
use hyper::header::Headers;


#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            status: StatusCode::Ok,
            headers: Default::default(),
            body: Vec::new()
        }
    }

    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }
}
