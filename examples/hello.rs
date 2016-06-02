extern crate sylph;
extern crate hyper;

use hyper::server::{Request, Response};
use sylph::{Sylph, Router, RegexRouter, HttpRequest, HttpResponse, SylphResult};


fn dummy(req: &mut HttpRequest<RegexRouter>) -> SylphResult<HttpResponse> {
    Ok(HttpResponse::new())
}


fn main() {
    let mut router = RegexRouter::new();
    router.get("/hello", dummy);
    router.build();
    let sylph = Sylph::new(router);
    sylph.listen("127.0.0.1:4000");
}
