use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/video.mkv")]
pub fn video() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new(
        "e:\\Videos\\Darling in the Franxx\\Anime\\Darling in the FranXX 01 Alone and Lonesome.mkv",
    );
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {}", path.display())))
}
