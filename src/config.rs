use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::colors;
use crate::protocol::Color;
use crate::variants::Variant;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mode: String,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub zones: Option<HashMap<String, String>>,
}

impl Config {
    pub fn solid(color: &str) -> Self {
        Config {
            mode: "solid".into(),
            color: Some(color.into()),
            zones: None,
        }
    }

    pub fn rainbow() -> Self {
        Config {
            mode: "rainbow".into(),
            color: None,
            zones: None,
        }
    }

    pub fn off() -> Self {
        Config {
            mode: "off".into(),
            color: None,
            zones: None,
        }
    }

    pub fn per_zone(zone_colors: HashMap<String, String>) -> Self {
        Config {
            mode: "per-zone".into(),
            color: None,
            zones: Some(zone_colors),
        }
    }
}

fn config_path() -> PathBuf {
    let dir = dirs_fallback();
    dir.join("config.toml")
}

fn dirs_fallback() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg).join("nzxt-lightctl")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config/nzxt-lightctl")
    } else {
        PathBuf::from("/etc/nzxt-lightctl")
    }
}

pub fn load() -> io::Result<Config> {
    let path = config_path();
    let content = fs::read_to_string(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Cannot read config at {}: {e}", path.display()),
        )
    })?;
    let config: Config = toml::from_str(&content).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid config: {e}"))
    })?;
    Ok(config)
}

pub fn save(config: &Config) -> io::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to serialize config: {e}"))
    })?;
    fs::write(&path, content)?;
    eprintln!("Config saved to {}", path.display());
    Ok(())
}

/// Resolve a Config into a 10-slot internal color array.
pub fn resolve_colors(config: &Config, variant: &Variant) -> io::Result<[Color; 10]> {
    let mapping = variant.user_to_internal();
    let mut internal = [(0u8, 0u8, 0u8); 10];

    match config.mode.as_str() {
        "off" => Ok(internal),
        "solid" => {
            let color_str = config.color.as_deref().unwrap_or("ffffff");
            let color = resolve_color_io(color_str)?;
            internal.fill(color);
            Ok(internal)
        }
        "rainbow" => {
            let user_colors = crate::protocol::rainbow_colors(variant.zone_count);
            for (user_idx, &color) in user_colors.iter().enumerate() {
                if user_idx < mapping.len() {
                    internal[mapping[user_idx]] = color;
                }
            }
            Ok(internal)
        }
        "per-zone" => {
            let zones = config.zones.as_ref().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "per-zone mode requires [zones] table")
            })?;
            for (name, hex) in zones {
                let user_idx = variant.zone_name_to_user_index(name).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unknown zone '{}'. Use 'nzxt-lightctl zones' to list.", name),
                    )
                })?;
                let color = resolve_color_io(hex)?;
                if user_idx < mapping.len() {
                    internal[mapping[user_idx]] = color;
                }
            }
            Ok(internal)
        }
        other => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown mode '{other}'. Valid: solid, rainbow, off, per-zone"),
        )),
    }
}

fn resolve_color_io(input: &str) -> io::Result<Color> {
    colors::resolve_color(input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}
