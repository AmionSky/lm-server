use crate::covers;
use crate::SharedConfig;
use actix_files::NamedFile;
use actix_web::{get, web};

#[get("/cover/{uid}")]
pub async fn cover(config: web::Data<SharedConfig>, uid: web::Path<String>) -> Option<NamedFile> {
    let cfg = config.read().ok()?;
    let cover = &cfg.shared.get(uid.as_str())?.cover;
    match cover {
        Some(c) => NamedFile::open(c).ok(),
        None => {
            let path = covers::get_path(uid.as_str());
            if path.exists() {
                NamedFile::open(path).ok()
            } else {
                None
            }
        }
    }
}
