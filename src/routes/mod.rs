//! Defines all web routes

pub mod index;

/// Used to pack all assets into the production binary at build time
#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
pub struct WebAppAssets;