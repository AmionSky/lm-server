use crate::moviedb;
use crate::SharedConfig;
use log::{warn,error};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

pub const FOLDER:&str = "covers";

pub fn check(app_cfg: SharedConfig) -> Result<(), Box<dyn Error>> {
    let cfg = app_cfg.read().unwrap();
    for (key, value) in &cfg.shared {
        let cover_path = get_path(key);
        if value.cover.is_none() && !cover_path.exists() {
            if let Err(err) = download_cover(&cover_path,&value.name) {
                error!("{}",err);
            }
        }
    }

    Ok(())
}

fn download_cover(cover_path: &PathBuf, name: &str) -> Result<(), Box<dyn Error>> {
    let url = moviedb::cover_url(name, moviedb::PosterSize::Original)?;
    let response = ureq::get(&url).timeout(Duration::from_secs(10)).call();
    if response.ok() {
        let mut cover = std::fs::File::create(cover_path)?;
        std::io::copy(&mut response.into_reader(), &mut cover)?;
    } else {
        warn!("Failed to download cover of {}", name);
    }

    Ok(())
}

pub fn get_path(key: &str) -> PathBuf {
    let mut cover = std::env::current_dir().unwrap();
    cover.push(FOLDER);
    cover.push(key);
    cover
}
