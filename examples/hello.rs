extern crate sylph;
extern crate hyper;

use hyper::server::{Request, Response};
use sylph::{Sylph, Router, RegexRouter, Params};


fn dummy(_: Request, _: Response, _: Params) {
    // Nothing
}


fn main() {
    let mut router = RegexRouter::new();
    router.get("/hello", dummy);

    let sylph = Sylph::new(router);
    sylph.listen("127.0.0.1:4000");
}
