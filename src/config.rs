use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub memory: MemoryConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_max_entries")]
    pub max_entries: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LocalConfig {
    #[serde(default)]
    pub memory: LocalMemoryConfig,
    #[serde(default)]
    pub aliases: HashMap<String, Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LocalMemoryConfig {
    pub enabled: Option<bool>,
}

fn default_enabled() -> bool {
    true
}

fn default_max_entries() -> usize {
    10
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            max_entries: default_max_entries(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };

        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("wenv").join("config.toml"))
    }

    pub fn dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("wenv"))
    }
}

impl LocalConfig {
    pub fn load(dir: &Path) -> Option<Self> {
        let path = dir.join(".wenv.toml");
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn expand_aliases(&self, files: &[String]) -> Vec<String> {
        files
            .iter()
            .flat_map(|f| {
                if let Some(name) = f.strip_prefix('@') {
                    self.aliases
                        .get(name)
                        .cloned()
                        .unwrap_or_else(|| vec![f.clone()])
                } else {
                    vec![f.clone()]
                }
            })
            .collect()
    }
}
