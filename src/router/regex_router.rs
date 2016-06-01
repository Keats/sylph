use std::collections::HashMap;

use hyper::method::Method;
use regex::{Regex, RegexSet};

use super::{Params, HandlerFn, Router, Route, RequestHandler};


// Can't derive debug because of HandlerFn
// TODO: do it manually?
pub struct RegexRouter {
    paths: Vec<String>,
    compiled_paths: Vec<Regex>,
    regexset: Option<RegexSet>,
    routes: HashMap<Route, HandlerFn>,
}


impl Router for RegexRouter {
    fn new() -> RegexRouter {
        RegexRouter {
            paths: Vec::new(),
            compiled_paths: Vec::new(),
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

    fn put(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Put, path, handler);
    }

    fn post(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Post, path, handler);
    }

    fn patch(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Patch, path, handler);
    }

    fn get(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Get, path, handler);
    }

    fn delete(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Delete, path, handler);
    }

    fn options(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Options, path, handler);
    }

    fn head(&mut self, path: &str, handler: HandlerFn) {
        self.add_route(Method::Head, path, handler);
    }

    fn mount(&mut self, root_path: &str, router: &RegexRouter) {
        for (route, handler) in &router.routes {
            self.add_route(
                route.method.clone(),
                &format!("{}{}", root_path, route.path),
                *handler
            );
        }
    }

    fn build(&mut self) {
        for (route, _) in &self.routes {
            // Check the regex is a valid one
            let re = Regex::new(&route.path).expect(&format!("{:?} is not a valid regex", route.path));
            self.paths.push(route.path.clone());
            self.compiled_paths.push(re);
        }

        self.regexset = Some(
            RegexSet::new(self.paths.iter()).expect("Failed to create routes")
        );
    }

    fn find_handler(&self, method: Method, uri: &str) -> RequestHandler {
        let matches = self.regexset.as_ref().unwrap().matches(&uri);
        let index = matches.iter().next();

        match index {
            Some(i) => {
                let path = &self.paths[i];
                let route = Route {path: path.to_string(), method: method};

                match self.routes.get(&route) {
                    Some(h) => {
                        let compiled_path = &self.compiled_paths[i];
                        let caps = compiled_path.captures(uri).unwrap();
                        let mut params = HashMap::new();
                        // TODO: work for non-named captures somehow?
                        for cap in caps.iter_named() {
                            params.insert(cap.0.to_string(), cap.1.unwrap().to_string());
                        }
                        Some((*h, params))
                    },
                    None => {
                        // TODO: NotAllowed
                        None
                    }
                }
            },
            None =>  {
                // TODO: NotFound
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::method::Method;
    use hyper::server::{Request, Response};
    use super::{RegexRouter};
    use router::{Params, Router};

    fn dummy(_: Request, _: Response, _: Params) {
        // Nothing
    }

    #[test]
    fn works_with_one_route() {
        let mut router = RegexRouter::new();
        router.get("/hello", dummy);
        router.build();

        assert!(router.regexset.unwrap().is_match("/hello"));
    }

    #[test]
    fn works_with_several_routes() {
        let mut router = RegexRouter::new();
        router.get("/hello", dummy);
        router.get("/hey", dummy);
        router.build();

        assert!(router.regexset.unwrap().is_match("/hello"));
    }

    #[test]
    fn can_mount_other_router() {
        let mut router = RegexRouter::new();
        router.get("/bla", dummy);
        let mut api_router = RegexRouter::new();
        api_router.get("/users", dummy);
        router.mount("/api", &api_router);
        router.build();

        assert!(router.regexset.unwrap().is_match("/api/users"));
    }

    #[test]
    fn can_find_handler() {
        let mut router = RegexRouter::new();
        router.get("/hello", dummy);
        router.get(r"/(?P<year>\d{4})", dummy);
        router.build();

        let found = router.find_handler(Method::Get, "/2016");
        assert_eq!(found.is_some(), true);
        assert_eq!(found.unwrap().1.get("year").unwrap().to_string(), "2016".to_string());
    }

    #[test]
    #[should_panic]
    fn panics_with_incorrect_regex() {
        let mut router = RegexRouter::new();
        router.get("/[a-", dummy);
        router.build();
    }

    #[test]
    #[should_panic]
    fn panics_with_path_not_starting_with_slash() {
        let mut router = RegexRouter::new();
        router.get("hey", dummy);
        router.build();
    }
}
