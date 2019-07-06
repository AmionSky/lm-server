#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod error;
mod ignite;
mod pages;
mod watcher;

use config::Config;
use crossbeam_utils::sync::ShardedLock;
use std::error::Error;
use std::sync::Arc;

type SharedConfig = Arc<ShardedLock<Config>>;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = config::load(config::path())?;
    config::verify(&cfg)?;

    let cfg = Arc::new(ShardedLock::new(cfg));
    let _watcher = watcher::watch(cfg.clone())?;

    ignite::start(cfg)
}
