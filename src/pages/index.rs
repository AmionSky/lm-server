use crate::config::Config;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct Index {
    pub media_list: Vec<ListItem>,
}

#[derive(Serialize)]
pub struct ListItem {
    pub name: String,
}

#[get("/")]
pub fn index(config: State<Arc<Mutex<Config>>>) -> Json<Index> {
    let mut list = vec![];

    for media_group in &config.lock().unwrap().shared {
        list.push(ListItem {
            name: media_group.name.clone(),
        })
    }

    Json(Index { media_list: list })
}

#[get("/video.mkv")]
pub fn video() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new(
        "e:\\Videos\\Darling in the Franxx\\Anime\\Darling in the FranXX 01 Alone and Lonesome.mkv",
    );
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {}", path.display())))
}
