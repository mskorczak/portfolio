#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_contrib::serve::StaticFiles;

mod blog;

#[get("/")]
fn index() -> &'static str {
    "Hello, Bozo!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![blog::api, blog::api_id, blog::blog, blog::home, blog::article, blog::resume])
        //.mount("/", routes![index])
        .mount("/", FileServer::from("static/"))
        //.mount("/", FileServer::from("blog/"))
        //.register("/", catchers![blog::not_found])
        .attach(Template::fairing())
}

