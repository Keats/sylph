use hyper::server::{Server, Handler, Request, Response};
use hyper::net::HttpStream;
use hyper::{Next, Encoder, Decoder};

use router::{Router};


#[derive(Debug)]
pub struct Sylph<T: Router> {
    router: T,
}


impl<T: Router> Sylph<T> {
    fn new(router: T) -> Sylph<T> {
        Sylph {
            router: router,
        }
    }

    fn listen(&self, url: &str) {
        let server = Server::http(&url.parse().unwrap()).unwrap();
        // server.run();
    }
}


impl<T: Router> Handler<HttpStream> for Sylph<T> {
    fn on_request(&mut self, _: Request) -> Next {
        Next::write()
    }

    fn on_request_readable(&mut self, _: &mut Decoder<HttpStream>) -> Next {
        Next::write()
    }

    fn on_response(&mut self, response: &mut Response) -> Next {
        Next::write()
    }

    fn on_response_writable(&mut self, encoder: &mut Encoder<HttpStream>) -> Next {
        Next::end()
    }
}
