extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;

fn get_page(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Got page")))
}


fn main() {

    let mut router = Router::new();

    router.get("/:page", get_page, "page");

    Iron::new(router).http("localhost:3000").unwrap();
}