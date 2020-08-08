use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

const APIURL: &str = "https://api.themoviedb.org/3/";
const APIKEY: &str = "api_key=5080cc62760a871965429917413a6b31";
const OPTIONS: &str = "include_adult=false";

#[allow(dead_code)]
pub enum PosterSize {
    W92,
    W154,
    W185,
    W342,
    W500,
    W780,
    Original,
}

impl PosterSize {
    pub fn value(&self) -> &str {
        match *self {
            PosterSize::W92 => "w92",
            PosterSize::W154 => "w154",
            PosterSize::W185 => "w185",
            PosterSize::W342 => "w342",
            PosterSize::W500 => "w500",
            PosterSize::W780 => "w780",
            PosterSize::Original => "original",
        }
    }
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    pub total_results: i64,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    pub poster_path: Option<String>,
}

pub fn cover_url(name: &str, size: PosterSize) -> Result<String, Box<dyn Error>> {
    let poster = search(name)?;
    Ok(format!(
        "https://image.tmdb.org/t/p/{}{}",
        size.value(),
        poster
    ))
}

fn search(input: &str) -> Result<String, Box<dyn Error>> {
    let query = utf8_percent_encode(input, NON_ALPHANUMERIC).to_string();
    let url = format!(
        "{}search/multi?{}&{}&query={}",
        APIURL, APIKEY, OPTIONS, query
    );

    let response = ureq::get(&url).timeout(Duration::from_secs(10)).call();
    if response.ok() {
        let mut sres: SearchResponse = serde_json::from_reader(response.into_reader())?;
        if sres.total_results > 0 && !sres.results.is_empty() {
            return Ok(sres.results[0].poster_path.take().unwrap());
        } else {
            return Err("Not found".into());
        }
    }

    Err("themoviedb request failed".into())
}
