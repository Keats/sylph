use std::collections::HashMap;

use hyper::method::Method;
use hyper::server::{Request, Response};


pub mod regex_router;


pub type Params = HashMap<String, String>;
pub type HandlerFn = fn(Request, Response, Params);
// Remove the option later on, use a Result instead of Option?
pub type RequestHandler = Option<(HandlerFn, Params)>;


#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Route {
    path: String,
    method: Method
}


// trait that a router needs to implement, means we can add a TrieRouter or a routing
// like Flask if we want to
pub trait Router {
    fn new() -> Self;
    // Adding routes, with shortcuts for methods
    fn add_route(&mut self, method: Method, path: &str, handler: HandlerFn);
    fn get(&mut self, path: &str, handler: HandlerFn);
    fn post(&mut self, path: &str, handler: HandlerFn);
    fn put(&mut self, path: &str, handler: HandlerFn);
    fn delete(&mut self, path: &str, handler: HandlerFn);
    fn patch(&mut self, path: &str, handler: HandlerFn);
    fn options(&mut self, path: &str, handler: HandlerFn);
    fn head(&mut self, path: &str, handler: HandlerFn);
    // Mount another router at a prefix
    fn mount(&mut self, root_path: &str, router: &Self);

    // build can be a no-op depending on the router
    fn build(&mut self);
    fn find_handler(&self, method: Method, path: &str) -> RequestHandler;
}
