use std::sync::Arc;
use std::io::{self, Read, Write};

use hyper::{Next, Encoder, Decoder, RequestUri, Headers, HttpVersion};
use hyper::server::{Handler, Request, Response};
use hyper::method::Method;
use hyper::net::HttpStream;
use hyper::header::ContentLength;
use hyper::status::StatusCode;

use response::HttpResponse;
use errors::{SylphResult, SylphError};
use router::Router;
use sylph::Sylph;


pub struct HttpRequest<'a, R: Router + 'a> {
    router: &'a Arc<Box<R>>,
    path: String,
    method: Method,
    version: HttpVersion,
    headers: Headers,
    // our response
    response: SylphResult<HttpResponse>,
    // hyper details
    buf: Vec<u8>,
    body_length: u64,
    read_pos: usize,
    write_pos: usize,
}

impl<'a, R: Router> HttpRequest<'a, R> {
    pub fn new(router: &'a Arc<Box<R>>) -> HttpRequest<R> {
        HttpRequest {
            router: router,
            path: String::new(),
            method: Default::default(),
            version: Default::default(),
            headers: Default::default(),
            response: Err(SylphError::NotFound),
            body_length: 0,
            buf: vec![0; 4096],
            read_pos: 0,
            write_pos: 0,
        }
    }

    fn call_handler(&mut self) {
        match self.router.find_handler(self.method.clone(), &self.path) {
            Ok((handler_fn, params)) => self.response = handler_fn(self),
            Err(e) => self.response = Err(e),
        };
    }
}

impl<'a, R: Router> Handler<HttpStream> for HttpRequest<'a, R> {
    // First point of entry, we need to check whether we have a body to parse
    // or not. If we don't just send it to the router directly
    fn on_request(&mut self, req: Request) -> Next {
        match *req.uri() {
            RequestUri::AbsolutePath(ref path) => {
                self.headers = req.headers().clone();
                self.version = req.version().clone();
                self.method = req.method().clone();
                self.path = path.clone();

                let mut has_body = true;
                if let Some(len) = req.headers().get::<ContentLength>() {
                    has_body = **len > 0;
                    self.body_length = **len;
                }
                if has_body {
                    Next::read_and_write()
                } else {
                    self.call_handler();
                    Next::write()
                }
            },
            _ => Next::write()
        }
    }

    // We have a body to parse!
    fn on_request_readable(&mut self, transport: &mut Decoder<HttpStream>) -> Next {
        match transport.read(&mut self.buf) {
            // EOF
            Ok(0) => {
                self.call_handler();
                Next::write()
            },
            // more to read
            Ok(n) => {
                self.read_pos += n;
                if self.body_length <= self.read_pos as u64 {
                    self.call_handler();
                    Next::write()
                } else {
                    Next::read_and_write()
                }
            },
            Err(e) => match e.kind() {
                io::ErrorKind::WouldBlock => Next::read_and_write(),
                _ => {
                    println!("read error {:?}", e);
                    Next::end()
                }
            }
        }
    }

    fn on_response(&mut self, resp: &mut Response) -> Next {
        match self.response {
            Ok(ref response) => {
                resp.set_status(response.status);
                // TODO: add headers from response to res
                let body_length = response.body.len();
                if body_length > 0 {
                    resp.headers_mut().set(ContentLength(body_length as u64));
                }
                Next::write()
            },
            Err(ref e) => {
                match e {
                    &SylphError::NotFound => {
                        resp.set_status(StatusCode::NotFound);
                    },
                    &SylphError::NotAllowed => {
                        resp.set_status(StatusCode::MethodNotAllowed);
                    },
                    &SylphError::InternalServerError => {
                        resp.set_status(StatusCode::InternalServerError);
                    }
                };

                Next::write()
            }
        }
    }

    // Writing the response body if we have one
    fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
        match self.response {
            Ok(ref response) => {
                if response.body.len() == 0 {
                    return Next::end();
                }

                match transport.write(&response.body[self.write_pos..]) {
                    Ok(0) => {
                        Next::end()
                    },
                    Ok(n) => {
                        self.write_pos += n;
                        Next::write()
                    },
                    Err(e) => match e.kind() {
                        io::ErrorKind::WouldBlock => Next::write(),
                        _ => {
                            println!("write error {:?}", e);
                            Next::end()
                        }
                    }
                }
            },
            _ => Next::end()
        }
    }
}
