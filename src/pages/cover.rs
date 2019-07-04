use crate::config::Config;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;
use std::sync::{Arc, Mutex};

#[get("/cover/<uid>")]
pub fn cover(config: State<Arc<Mutex<Config>>>, uid: &RawStr) -> Option<NamedFile> {
    let cfg = config.lock().ok()?;
    let cover = &cfg.shared.get(uid.as_str())?.cover;
    match cover {
        Some(c) => NamedFile::open(c).ok(),
        None => None,
    }
}
