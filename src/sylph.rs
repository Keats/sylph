use std::sync::Arc;

use hyper::server::{Server};
use hyper::net::HttpListener;
use num_cpus;

use router::Router;
use request::HttpRequest;


#[derive(Debug)]
pub struct Sylph<T: Router> {
    router: T,
}


impl<T: Router> Sylph<T> {
    pub fn new(router: T) -> Sylph<T> {
        Sylph {
            router: router,
        }
    }

    pub fn listen(self, url: &str) {
        let listener = HttpListener::bind(&url.parse().unwrap()).unwrap();
        let mut handles = Vec::new();
        let arc_router = Arc::new(Box::new(self.router.clone()));

        for _ in 0..num_cpus::get() {
            let listener = listener.try_clone().unwrap();
            handles.push(::std::thread::spawn(move || {
                Server::new(listener)
                    .handle(|_| HttpRequest::new(arc_router.clone()))
                    .unwrap();
            }));
        }
        println!("Listening on {}", url);

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
