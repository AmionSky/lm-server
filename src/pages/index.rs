use crate::SharedConfig;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    pub media_list: Vec<IndexListItem>,
}

#[derive(Serialize)]
pub struct IndexListItem {
    pub uid: String,
    pub name: String,
    pub has_cover: bool,
}

#[get("/")]
pub fn index(config: State<SharedConfig>) -> Option<Json<IndexResponse>> {
    let cfg = config.read().unwrap();
    let mut list = vec![];

    for (uid, mg) in &cfg.shared {
        list.push(IndexListItem {
            uid: uid.clone(),
            name: mg.name.clone(),
            has_cover: mg.cover.is_some(),
        })
    }

    Some(Json(IndexResponse { media_list: list }))
}
