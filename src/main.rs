// Visit `http://127.0.0.1:3000/?user[name]=Marie` to be greeted with a welcome message. Any other
// request will return a 404 error.

extern crate iron;
extern crate params;

use iron::prelude::*;

fn handle_user(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    match map.find(&["user", "name"]) {
        Some(&Value::String(ref name)) if name == "Jasper" => {
            Ok(Response::with((iron::status::Ok, "Welcome back, Jasper!")))
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}

fn main() {
    Iron::new(handle_user).http("127.0.0.1:3000").unwrap();
}