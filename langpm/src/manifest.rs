use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub package: PackageMeta,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: HashMap<String, DependencySpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    #[serde(default = "default_entry")]
    pub entry: String,
    #[serde(default)]
    pub description: String,
}

fn default_entry() -> String {
    "main.lp".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DependencySpec {
    Version(String),
    Detailed(DetailedDep),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DetailedDep {
    pub version: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub path: Option<String>,
}

impl Manifest {
    pub fn load_from_dir(dir: &Path) -> Result<Self, String> {
        for name in ["langp.toml", "lang.toml"] {
            let path = dir.join(name);
            if path.is_file() {
                return Self::load(&path);
            }
        }
        Err(format!(
            "no langp.toml or lang.toml found in {}",
            dir.display()
        ))
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path)
            .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("invalid manifest: {e}"))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let text = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, text).map_err(|e| e.to_string())
    }

    pub fn init_project(dir: &Path, name: &str) -> Result<PathBuf, String> {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        let manifest = Manifest {
            package: PackageMeta {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                entry: "main.lp".to_string(),
                description: String::new(),
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        };
        let manifest_path = dir.join("langp.toml");
        manifest.save(&manifest_path)?;
        let main_lp = dir.join("main.lp");
        if !main_lp.exists() {
            fs::write(
                &main_lp,
                "@ Entry point\nprint \"Hello from Lang.P\".\n",
            )
            .map_err(|e| e.to_string())?;
        }
        fs::create_dir_all(dir.join("src")).ok();
        fs::create_dir_all(dir.join("tests")).ok();
        Ok(manifest_path)
    }
}
