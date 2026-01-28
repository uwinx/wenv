use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_env_files(files: &[String]) -> Result<HashMap<String, String>, String> {
    let mut vars = HashMap::new();

    for file in files {
        let content = fs::read_to_string(file).map_err(|e| format!("Error reading {file}: {e}"))?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"').trim_matches('\'');
                vars.insert(key.to_string(), value.to_string());
            }
        }
    }

    Ok(vars)
}

pub fn filter_existing(files: &[String]) -> Vec<String> {
    files
        .iter()
        .filter(|f| Path::new(f).exists())
        .cloned()
        .collect()
}
