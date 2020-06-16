#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;

use rocket::request::Form;
use rocket::response::content;

use rocket_contrib::json::Json;
use serde_json::Value;



#[get("/")]
fn index() -> &'static str {
    "Hello, world! 003 index"
}


#[derive(FromForm)]
struct Task {
    user_id: bool,
    user_name: String,
}

#[post("/create", data = "<task>")]
fn create(task: Form<Task>) -> content::Json<&'static str> {

    content::Json("{
        'code': '200',
        'message': 'return__msg',
    }")
}

#[post("/update", data = "<task>")]
fn update(task: Form<Task>) -> Json<Value> {

    Json(json!({
        "status": 200,
        "result": "ret",
    }))
}


fn main() {
    rocket::ignite().mount("/", routes![index, create, update]).launch();
}
