TODO:

- add HttpRequest that impl Handler<HttpStream> instead of Sylph
See https://github.com/sappworks/sapper/blob/master/src/sapp.rs#L282 for inspiration
It should have a function to get query params in hashmap and the data from the original
request
- add HttpResponse that has some shortcuts for json (jsonify!() ?) etc
- add benches to see if it's not straying too far from hyper (90k/s)
- add 404/405 with 404 configurable 404 to router
- add names to urls and reverse_url method -> is that needed?
- static files handling
- file upload


To think of:
we need to pass data around between "middlewares" such as a db connection which means we need to have some kind of context passed to the handler function

See https://www.reddit.com/r/rust/comments/4lz92j/tween_a_middleware_library_experiment/ for some interesting thoughts about middlewares

https://github.com/tomaka/rouille/blob/master/examples/hello_world.rs#L8 is very nice

Rather than Params right now, the function could look like:

```
fn profile(context: Context) -> Result<HttpResponse> {

}
```

Should we not parse captures at first but have a method in HttpRequest to do it?
Saves the hashmap creation and avoids parsing capture unless wanted (if possible)

We need a way to get the routes anywwhere to reverse and write the url_for Tera fn and i have no idea how to do that (yet). Pass the router to everything and add a `reverse` fn to the Router trait?
