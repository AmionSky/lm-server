use crate::config::Config;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;
use std::sync::{Arc, Mutex};

#[get("/video/<uid>/<file>")]
pub fn video(config: State<Arc<Mutex<Config>>>, uid: &RawStr, file: &RawStr) -> Option<NamedFile> {
    let file = file.percent_decode().ok()?;
    let cfg = config.lock().ok()?;
    let path = &cfg.shared.get(uid.as_str())?.path;
    NamedFile::open(path.join(file.to_string())).ok()
}
