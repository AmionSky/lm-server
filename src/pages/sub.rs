use crate::SharedConfig;
use actix_files::NamedFile;
use actix_web::{get, web};
use percent_encoding::percent_decode_str;

#[get("/sub/{uid}/{file}")]
pub async fn sub(
    config: web::Data<SharedConfig>,
    info: web::Path<(String, String)>,
) -> Option<NamedFile> {
    let file = percent_decode_str(&info.1).decode_utf8().ok()?;
    let cfg = config.read().unwrap();
    let subs = &cfg.shared.get(info.0.as_str())?.subs;
    match subs {
        Some(subs) => {
            let mut sub_path = subs.path.join(file.to_string());
            sub_path.set_extension(&subs.ext);
            NamedFile::open(sub_path).ok()
        }
        None => None,
    }
}
