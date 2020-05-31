#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/<random_num>")]
fn assigment(random_num: u16) -> & String {
    format!("{}", random_num + 50)
}

fn main() {
    rocket::ignite().mount("/", routes![assigment]).launch();
}