#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod error;

use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/video.mkv")]
fn video() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new(
        "e:\\Videos\\Darling in the Franxx\\Anime\\Darling in the FranXX 01 Alone and Lonesome.mkv",
    );
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {}", path.display())))
}

fn main() {
    rocket::ignite().mount("/", routes![index, video]).launch();
}
