use crate::config;
use crate::SharedConfig;
use hotwatch::{Event, Hotwatch};
use std::error::Error;

pub fn watch(cfg: SharedConfig) -> Result<Hotwatch, Box<dyn Error>> {
    let mut hotwatch = Hotwatch::new()?;

    hotwatch.watch(config::path(), move |event: Event| {
        if let Event::Write(_path) = event {
            if let Ok(new_cfg) = config::load(config::path()) {
                if config::verify(&new_cfg).is_ok() {
                    let mut shared_cfg = cfg.write().unwrap();
                    *shared_cfg = new_cfg;
                    println!("Configuration updated!");
                } else {
                    eprintln!("Updated config verification failed");
                }
            } else {
                eprintln!("Failed to load updated config");
            }
        }
    })?;

    Ok(hotwatch)
}
