use crate::SharedConfig;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;

#[get("/video/<uid>/<file>")]
pub fn video(config: State<SharedConfig>, uid: &RawStr, file: &RawStr) -> Option<NamedFile> {
    let file = file.percent_decode().ok()?;
    let cfg = config.read().unwrap();
    let path = &cfg.shared.get(uid.as_str())?.path;
    NamedFile::open(path.join(file.to_string())).ok()
}
