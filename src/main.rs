#![windows_subsystem = "windows"]

mod config;
mod covers;
mod error;
mod ignite;
mod moviedb;
mod pages;
mod watcher;

use config::Config;
use crossbeam_utils::sync::ShardedLock;
use simplelog::{ColorChoice, Config as LoggerConfig, LevelFilter, TermLogger, TerminalMode};
use std::error::Error;
use std::sync::Arc;

type SharedConfig = Arc<ShardedLock<Config>>;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    attach_console();

    TermLogger::init(
        LevelFilter::Info,
        LoggerConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let cfg = config::load(config::path())?;
    config::verify(&cfg)?;

    let cfg = Arc::new(ShardedLock::new(cfg));
    let _watcher = watcher::watch(cfg.clone())?;

    std::fs::create_dir(covers::FOLDER).ok();
    covers::check(cfg.clone())?;
    ignite::start(cfg)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn attach_console() {
    use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
    let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
}
