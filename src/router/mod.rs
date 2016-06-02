use std::collections::HashMap;

use hyper::method::Method;
use hyper::server::{Request, Response};


pub mod regex_router;

use errors::SylphResult;
use request::HttpRequest;
use response::HttpResponse;


pub type Params = HashMap<String, String>;
pub type HandlerFn<T> = fn(&mut HttpRequest<T>) -> SylphResult<HttpResponse>;
pub type RequestHandler<T> = SylphResult<(HandlerFn<T>, Params)>;


#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Route {
    path: String,
    method: Method
}


// trait that a router needs to implement, means we can add a TrieRouter or a routing
// like Flask if we want to
pub trait Router: Sync + Send + Sized {
    fn new() -> Self;
    // Adding routes, with shortcuts for methods
    fn add_route(&mut self, method: Method, path: &str, handler: HandlerFn<Self>);
    fn get(&mut self, path: &str, handler: HandlerFn<Self>);
    fn post(&mut self, path: &str, handler: HandlerFn<Self>);
    fn put(&mut self, path: &str, handler: HandlerFn<Self>);
    fn delete(&mut self, path: &str, handler: HandlerFn<Self>);
    fn patch(&mut self, path: &str, handler: HandlerFn<Self>);
    fn options(&mut self, path: &str, handler: HandlerFn<Self>);
    fn head(&mut self, path: &str, handler: HandlerFn<Self>);
    // Mount another router at a prefix
    fn mount(&mut self, root_path: &str, router: &Self);

    // build can be a no-op depending on the router
    fn build(&mut self);
    fn find_handler(&self, method: Method, path: &str) -> RequestHandler<Self>;
}
