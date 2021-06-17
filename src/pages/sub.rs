use crate::SharedConfig;
use actix_files::NamedFile;
use actix_web::{get, web};

#[get("/sub/{uid}/{file}")]
pub async fn sub(
    config: web::Data<SharedConfig>,
    web::Path((uid, file)): web::Path<(String, String)>,
) -> Option<NamedFile> {
    let cfg = config.read().ok()?;
    let subs = &cfg.shared.get(&uid)?.subs;
    match subs {
        Some(subs) => {
            let mut sub_path = subs.path.join(&file);
            sub_path.set_extension(&subs.ext);
            NamedFile::open(sub_path).ok()
        }
        None => None,
    }
}
