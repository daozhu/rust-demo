#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! 002 index"
}


#[get("/login")]
fn login() -> &'static str {
    "Hello, world! 002 login"
}


#[get("/check")]
fn check() -> &'static str {
    "Hello, world! 002 check"
}


#[get("/nice")]
fn nice() -> &'static str {
    "Hello, world! 002 nice"
}



fn main() {
    rocket::ignite().mount("/", routes![index, login, check, nice]).launch();
}
