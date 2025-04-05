//! Starforge Config - Configuration system for Starforge compositors
//!
//! This library provides a modular, extensible configuration system.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

/// The main configuration struct for Starforge
#[derive(Debug, Serialize, Deserialize)]
pub struct StarforgeConfig {
    /// General configuration options
    pub general: GeneralConfig,

    /// Rendering configuration
    pub rendering: RenderConfig,
}

/// General configuration options
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Application name
    #[serde(default = "default_app_name")]
    pub app_name: String,

    /// Enable debugging features
    #[serde(default)]
    pub debug: bool,
}

fn default_app_name() -> String {
    "Starforge Compositor".to_string()
}

/// Rendering configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Enable vsync
    #[serde(default = "default_vsync")]
    pub vsync: bool,

    /// Background color (RGBA)
    #[serde(default = "default_background_color")]
    pub background_color: [f32; 4],
}

fn default_vsync() -> bool {
    true
}

fn default_background_color() -> [f32; 4] {
    [0.1, 0.1, 0.2, 1.0] // Dark blue
}

impl Default for StarforgeConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                app_name: default_app_name(),
                debug: false,
            },
            rendering: RenderConfig {
                vsync: default_vsync(),
                background_color: default_background_color(),
            },
        }
    }
}

/// Errors that can occur in the configuration system
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Failed to serialize TOML: {0}")]
    SerializeError(#[from] toml::ser::Error),
}

impl StarforgeConfig {
    /// Load configuration from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config: StarforgeConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
