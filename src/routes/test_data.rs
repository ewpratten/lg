use rocket::http::ContentType;


/// Return 10M of 0 bytes
#[get("/test_files/10m.bin")]
pub fn test_files_10m() -> (ContentType, Vec<u8>) {
    let data = vec![0; 10_000_000];
    (ContentType::Binary, data)
}

/// Return 100M of 0 bytes
#[get("/test_files/100m.bin")]
pub fn test_files_100m() -> (ContentType, Vec<u8>) {
    let data = vec![0; 100_000_000];
    (ContentType::Binary, data)
}