//! This module defines the structures backing the app config files.

/// The "local config" contains data specific to this instance
#[derive(Debug, Deserialize)]
pub struct LocalConfig {
    /// The page title
    pub title: String,
    /// The page subtitle
    pub subtitle: String,
    /// The country code for this instance
    pub country_code: String,
    /// The location name of this instance
    pub location_name: String,
}

/// Used in the instance listing config
#[derive(Debug, Deserialize, Default)]
pub struct Instance {
    /// The location name of this instance
    pub location_name: String,
    /// The country code for this instance
    pub country_code: String,
    /// The IP address or hostname of this instance
    pub host: String,
}


/// The "listing config" contains data that can be shared between multiple running instances
#[derive(Debug, Deserialize, Default)]
pub struct InstanceListingConfig {
    /// All instances
    pub instances: Vec<Instance>,
}