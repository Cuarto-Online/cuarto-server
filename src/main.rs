// Visit `http://127.0.0.1:3000/?user[name]=Marie` to be greeted with a welcome message. Any other
// request will return a 404 error.

extern crate iron;
extern crate params;
extern crate mongodb;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use iron::prelude::*;

#[derive(Serialize, Deserialize)]
struct BotRegistrationInfo {
    name: String,
    author: String
}

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
    
    let reg_info = BotRegistrationInfo {
        name: "Test Bot".to_owned(),
        author: "Dario".to_owned()
    };

    let j = serde_json::to_string(&reg_info).unwrap();

    println!("Test {}", j);

    Iron::new(handle_user).http("127.0.0.1:3000").unwrap();
}