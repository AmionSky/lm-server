use crate::SharedConfig;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;

#[get("/sub/<uid>/<file>")]
pub fn sub(config: State<SharedConfig>, uid: &RawStr, file: &RawStr) -> Option<NamedFile> {
    let file = file.percent_decode().ok()?;
    let cfg = config.read().unwrap();
    let subs = &cfg.shared.get(uid.as_str())?.subs;
    match subs {
        Some(subs) => {
            let mut sub_path = subs.path.join(file.to_string());
            sub_path.set_extension(&subs.ext);
            NamedFile::open(sub_path).ok()
        }
        None => None,
    }
}
