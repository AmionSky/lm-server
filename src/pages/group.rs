use crate::SharedConfig;
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::path::Path;

const VIDEO_EXT: [&str; 3] = [".mkv", ".mp4", ".avi"];

#[derive(Serialize)]
pub struct GroupResponse {
    pub videos: Vec<VideoDetails>,
}

#[derive(Serialize)]
pub struct VideoDetails {
    pub name: String,
}

#[get("/group/<uid>")]
pub fn group(config: State<SharedConfig>, uid: &RawStr) -> Option<Json<GroupResponse>> {
    let cfg = config.read().unwrap();
    let group = &cfg.shared.get(uid.as_str())?;
    let videos = query_dir(&group.path).ok()?;
    Some(Json(GroupResponse { videos }))
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
