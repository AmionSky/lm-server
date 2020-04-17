use crate::SharedConfig;
use actix_files::NamedFile;
use actix_web::{get, web};

#[get("/video/{uid}/{file}")]
pub async fn video(
    config: web::Data<SharedConfig>,
    info: web::Path<(String, String)>,
) -> Option<NamedFile> {
    let cfg = config.read().ok()?;
    let path = &cfg.shared.get(&info.0)?.path;
    NamedFile::open(path.join(&info.1)).ok()
}
