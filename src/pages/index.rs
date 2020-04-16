use crate::SharedConfig;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    pub media_list: Vec<IndexListItem>,
}

impl Responder for IndexResponse {
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
pub struct IndexListItem {
    pub uid: String,
    pub name: String,
    pub has_cover: bool,
}

#[get("/")]
pub async fn index(config: web::Data<SharedConfig>) -> Option<IndexResponse> {
    let cfg = config.read().unwrap();
    let mut list = vec![];

    for (uid, mg) in &cfg.shared {
        list.push(IndexListItem {
            uid: uid.clone(),
            name: mg.name.clone(),
            has_cover: mg.cover.is_some(),
        })
    }

    Some(IndexResponse { media_list: list })
}
