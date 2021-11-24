//! Defines all web routes

pub mod index;
pub mod static_data;
pub mod test_data;

/// Used to pack all assets into the production binary at build time
#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
pub struct WebAppAssets;