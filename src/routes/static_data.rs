use std::path::{Path, PathBuf};

use rocket::http::ContentType;

use super::WebAppAssets;

/// Loads arbitrary files from `assets/static` while handling production memory caches
#[get("/static/<path..>")]
pub fn static_data(path: PathBuf) -> (ContentType, String) {
    let data = WebAppAssets::get(Path::new("static").join(&path).to_str().unwrap())
        .unwrap()
        .data;
    (
        ContentType::from_extension(path.extension().unwrap().to_str().unwrap()).unwrap(),
        String::from_utf8((&data).to_vec()).unwrap(),
    )
}
