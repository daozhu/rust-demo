#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/test")]
fn index() -> &'static str {
    "Hello, world!  test "
}

#[get("/back")]
fn back() -> &'static str {
    "Hello, world! i`m back "
}


fn main() {
    rocket::ignite().mount("/", routes![index, back]).launch();
}
