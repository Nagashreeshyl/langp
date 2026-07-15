use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LockFile {
    pub version: u32,
    #[serde(default)]
    pub packages: Vec<LockedPackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    #[serde(default)]
    pub checksum: String,
}

impl LockFile {
    pub fn load(path: &Path) -> Result<Self, String> {
        if !path.is_file() {
            return Ok(Self {
                version: 1,
                packages: vec![],
            });
        }
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        toml::from_str(&text).map_err(|e| format!("invalid lock file: {e}"))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let text = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, text).map_err(|e| e.to_string())
    }
}
