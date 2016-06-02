use std::sync::Arc;

use hyper::server::{Server};
use hyper::net::HttpListener;
use num_cpus;

use router::Router;
use request::HttpRequest;


#[derive(Debug)]
pub struct Sylph<R: Router + 'static> {
    router: R,
}


impl<R: Router> Sylph<R> {
    pub fn new(router: R) -> Sylph<R> {
        // TODO: check the router is valid (build() called)
        Sylph {
            router: router,
        }
    }

    pub fn listen(self, url: &str) {
        let listener = HttpListener::bind(&url.parse().unwrap()).unwrap();
        let mut handles = Vec::new();
        let arc_router = Arc::new(Box::new(self.router));

        for _ in 0..num_cpus::get() {
            let listener = listener.try_clone().unwrap();
            let router = arc_router.clone();
            handles.push(::std::thread::spawn(move || {
                Server::new(listener)
                    .handle(|_| HttpRequest::new(&router))
                    .unwrap();
            }));
        }
        println!("Listening on {}", url);

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
