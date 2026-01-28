use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Memory {
    #[serde(default)]
    entries: HashMap<String, Vec<String>>,
}

impl Memory {
    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };

        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let Some(path) = Self::path() else {
            return Ok(());
        };

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, content)
    }

    pub fn record(&mut self, dir: &Path, env_files: &[String], max_entries: usize) {
        let canonical = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
        let key = canonical.to_string_lossy().to_string();
        let entry = self.entries.entry(key).or_default();

        for file in env_files.iter().rev() {
            entry.retain(|f| f != file);
            entry.insert(0, file.clone());
        }

        entry.truncate(max_entries);
    }

    pub fn get(&self, dir: &Path) -> Option<&Vec<String>> {
        let canonical = dir.canonicalize().ok()?;
        let key = canonical.to_string_lossy();
        self.entries.get(key.as_ref())
    }

    fn path() -> Option<PathBuf> {
        Config::dir().map(|p| p.join("memory.toml"))
    }
}
