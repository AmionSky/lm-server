use crate::SharedConfig;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;

#[get("/cover/<uid>")]
pub fn cover(config: State<SharedConfig>, uid: &RawStr) -> Option<NamedFile> {
    let cfg = config.read().unwrap();
    let cover = &cfg.shared.get(uid.as_str())?.cover;
    match cover {
        Some(c) => NamedFile::open(c).ok(),
        None => None,
    }
}
