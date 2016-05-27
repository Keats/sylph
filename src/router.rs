use std::collections::HashMap;

use hyper::method::Method;
use hyper::server::{Handler, Request, Response};

use regex::{Regex, RegexSet};


pub type Params = HashMap<String, String>;
pub type HandlerFn = fn(Request, Response, Params);


#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Route {
    path: String,
    method: Method
}

// Can't derive debug because of HandlerFn
pub struct Router {
    paths: Vec<String>,
    regexset: Option<RegexSet>,
    routes: HashMap<Route, HandlerFn>,
}

impl Router {
    fn new() -> Router {
        Router {
            paths: Vec::new(),
            regexset: None,
            routes: HashMap::new()
        }
    }

    fn add_route(&mut self, method: Method, path: &str, handler: HandlerFn) {
        // Some sanity checks
        // Ideally we would check that each regex has a named capture but it's hard
        if path.len() == 0 || path.chars().nth(0).unwrap() != '/' {
            panic!("Path cannot be empty and must start with a '/': {:?}", path);
        }
        let route = Route {path: path.to_string(), method: method};
        self.routes.insert(route, handler);
    }

    pub fn put(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Put, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Post, path, handler);
    }

    pub fn patch(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Patch, path, handler);
    }

    pub fn get(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Get, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Delete, path, handler);
    }

    pub fn options(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Options, path, handler);
    }

    pub fn head(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Head, path, handler);
    }

    pub fn mount(&mut self, root_path: &str, router: &Router) {
        for (route, handler) in &router.routes {
            self.add_route(
                route.method.clone(),
                &format!("{}{}", root_path, route.path),
                *handler
            );
        }
    }

    // Debug fn, can be deleted later
    fn dump_routes(&self) {
        for (route, _) in &self.routes {
            println!("{:?} - {:?}", route.method, route.path);
        }
    }

    fn build(&mut self) {
        for (route, _) in &self.routes {
            // Check the regex is a valid one
            Regex::new(&route.path).expect(&format!("{:?} is not a valid regex", route.path));
            self.paths.push(route.path.clone());
        }

        self.regexset = Some(
            RegexSet::new(self.paths.iter()).expect("Failed to create routes")
        );
    }
}

#[cfg(test)]
mod tests {
    use hyper::server::{Request, Response};
    use super::*;

    fn dummy(_: Request, _: Response, _: Params) {
        // Nothing
    }

    #[test]
    fn works_with_one_route() {
        let mut router = Router::new();
        router.get("/hello", dummy);
        router.build();

        assert!(router.regexset.unwrap().is_match("/hello"));
    }

    #[test]
    fn works_with_several_routes() {
        let mut router = Router::new();
        router.get("/hello", dummy);
        router.get("/hey", dummy);
        router.build();

        assert!(router.regexset.unwrap().is_match("/hello"));
    }

    #[test]
    fn can_mount_other_router() {
        let mut router = Router::new();
        router.get("/bla", dummy);
        let mut api_router = Router::new();
        api_router.get("/users", dummy);
        router.mount("/api", &api_router);
        router.build();

        assert!(router.regexset.unwrap().is_match("/api/users"));
    }

    #[test]
    #[should_panic]
    fn panics_with_incorrect_regex() {
        let mut router = Router::new();
        router.get("/[a-", dummy);
        router.build();
    }

    #[test]
    #[should_panic]
    fn panics_with_path_not_starting_with_slash() {
        let mut router = Router::new();
        router.get("hey", dummy);
        router.build();
    }
}
