#![windows_subsystem = "windows"]

mod config;
mod error;
mod ignite;
mod pages;
mod watcher;

use config::Config;
use crossbeam_utils::sync::ShardedLock;
use std::error::Error;
use std::sync::Arc;
use simplelog::{TermLogger,LevelFilter,TerminalMode,Config as LoggerConfig};

type SharedConfig = Arc<ShardedLock<Config>>;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    attach_console();

    TermLogger::init(LevelFilter::Debug, LoggerConfig::default(), TerminalMode::Mixed)?;

    let cfg = config::load(config::path())?;
    config::verify(&cfg)?;

    let cfg = Arc::new(ShardedLock::new(cfg));
    let _watcher = watcher::watch(cfg.clone())?;

    ignite::start(cfg)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn attach_console() {
    use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
    let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
}
