use crate::SharedConfig;
use actix_files::NamedFile;
use actix_web::{get, web};

#[get("/video/{uid}/{file}")]
pub async fn video(
    config: web::Data<SharedConfig>,
    web::Path((uid, file)): web::Path<(String, String)>,
) -> Option<NamedFile> {
    let cfg = config.read().ok()?;
    let path = &cfg.shared.get(&uid)?.path;
    NamedFile::open(path.join(&file)).ok()
}
