use crate::error::LmSrvError;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Port the server should start on
    pub port: u16,
    /// Shared media group
    pub shared: Vec<MediaGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaGroup {
    /// Name of the media group
    pub name: String,
    /// Path to the media's containing directory
    pub path: PathBuf,
    /// Optional subtitle config
    pub subs: Option<Subtitle>,
    /// Optional path to the cover image
    pub cover: Option<String>,
    /// Optional regex to convert the media file names
    pub namerex: Option<String>,
}

/// Subtitle config for a media group.
/// The subtitle files need to have the same names as the video files.
#[derive(Serialize, Deserialize, Debug)]
pub struct Subtitle {
    /// Path to the subtitle's containing directory
    pub path: PathBuf,
    /// File extension of the subtitles
    pub ext: String,
}

/// Loads the config file from disk.
/// * `path` - Path to the config file
pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    Ok(json::from_reader(File::open(path)?)?)
}

/// Runs a basic check on the config
pub fn verify(config: &Config) -> Result<(), LmSrvError> {
    for share in &config.shared {
        if share.name.is_empty() {
            return Err(LmSrvError::new(
                "config",
                "MediaGroup name is empty!".into(),
            ));
        }

        if !share.path.is_dir() {
            return Err(LmSrvError::new(
                "config",
                "MediaGroup path is not valid or isn't a directory!".into(),
            ));
        }

        // TODO other checks
    }

    Ok(())
}

/// Gets the config file path. Determined by command-line argument.
/// Defaults to "lm-server.json".
pub fn path() -> PathBuf {
    PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| String::from("lm-server.json")),
    )
}
