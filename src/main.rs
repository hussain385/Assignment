#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::request::Form;
use rocket::response::content::Html;
#[derive(FromForm)]
struct Number{
    number : u32,
}

#[post("/num", data = "<number>")]
fn addition(number : Form<Number>) -> Html<String> {
   let h1=format!("<p>{}</p>", number.number+50);
    Html(h1)
}
#[get("/num")]
fn input_number()-> Html<String>{
    let h1=format!("
        <form action='/num' method='post'>
            <input type='number' placeholder='Place Your Number'
                name='number' id='number'>
            <input type='Submit'>
        </form>
    ");
    Html(h1)
}

fn main() {
    rocket::ignite().mount("/", routes![input_number,addition]).launch();
}