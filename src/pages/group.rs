use crate::SharedConfig;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use std::path::Path;

const VIDEO_EXT: [&str; 3] = [".mkv", ".mp4", ".avi"];

#[derive(Serialize)]
pub struct GroupResponse {
    pub videos: Vec<VideoDetails>,
}

impl Responder for GroupResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Serialize)]
pub struct VideoDetails {
    pub name: String,
}

#[get("/group/{uid}")]
pub async fn group(
    config: web::Data<SharedConfig>,
    uid: web::Path<String>,
) -> Option<GroupResponse> {
    let cfg = config.read().ok()?;
    let group = &cfg.shared.get(uid.as_str())?;
    let videos = query_dir(&group.path).ok()?;
    Some(GroupResponse { videos })
}

/// Returns the name of all video files in the specified directory
fn query_dir(dir: &Path) -> Result<Vec<VideoDetails>, std::io::Error> {
    let mut files: Vec<VideoDetails> = vec![];

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            let file_name = entry.file_name().into_string();

            if let Ok(name) = file_name {
                if is_video(&name) {
                    files.push(VideoDetails { name });
                }
            }
        }
    }

    Ok(files)
}

fn is_video(file: &str) -> bool {
    for ext in &VIDEO_EXT {
        if file.ends_with(ext) {
            return true;
        }
    }

    false
}
