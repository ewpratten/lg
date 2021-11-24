use rocket::http::ContentType;

use super::WebAppAssets;

#[get("/")]
pub fn index() -> (ContentType, String) {
    let data = WebAppAssets::get("index.html").unwrap().data;
    (
        ContentType::HTML,
        String::from_utf8((&data).to_vec()).unwrap(),
    )
}
