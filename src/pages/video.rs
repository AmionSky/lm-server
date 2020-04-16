use crate::SharedConfig;
use actix_files::NamedFile;
use actix_web::{get, web};
use percent_encoding::percent_decode_str;

#[get("/video/{uid}/{file}")]
pub async fn video(
    config: web::Data<SharedConfig>,
    info: web::Path<(String, String)>,
) -> Option<NamedFile> {
    let file = percent_decode_str(&info.1).decode_utf8().ok()?;
    let cfg = config.read().unwrap();
    let path = &cfg.shared.get(info.0.as_str())?.path;
    NamedFile::open(path.join(file.to_string())).ok()
}
