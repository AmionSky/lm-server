use crate::config::{self, Config};
use hotwatch::{Event, Hotwatch};
use std::sync::{Arc, Mutex};

pub fn watch(cfg: Arc<Mutex<Config>>) -> Hotwatch {
    let mut hotwatch = Hotwatch::new().expect("Hotwatch failed to initialize.");

    hotwatch
        .watch(config::path(), move |event: Event| {
            if let Event::Write(_path) = event {
                if let Ok(new_cfg) = config::load(config::path()) {
                    if config::verify(&new_cfg).is_ok() {
                        let mut shared_cfg = cfg.lock().unwrap();
                        *shared_cfg = new_cfg;
                        println!("Configuration updated!");
                    } else {
                        eprintln!("Updated config verification failed");
                    }
                } else {
                    eprintln!("Failed to load updated config");
                }
            }
        })
        .expect("Failed to watch file!");

    hotwatch
}
