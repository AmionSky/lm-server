#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod error;
mod pages;
mod watcher;

use std::any::Any;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

fn main() {
    let cfg = necessary(config::load(config::path()));
    necessary(config::verify(&cfg));

    let cfg = Arc::new(Mutex::new(cfg));
    let _watcher = watcher::watch(cfg.clone());

    rocket::ignite()
        .manage(cfg)
        .mount(
            "/",
            routes![
                pages::index::index,
                pages::cover::cover,
                pages::group::group,
                pages::video::video,
                pages::sub::sub
            ],
        )
        .launch();
}

fn necessary<T: Any, E: Display>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Fatal Error: {}", error);
            std::process::exit(1);
        }
    }
}
