use crate::lockfile::{LockFile, LockedPackage};
use crate::manifest::{DependencySpec, Manifest};
use semver::VersionReq;
use std::collections::HashMap;
use std::path::Path;

pub struct Resolver {
    registry: HashMap<String, Vec<String>>,
}

impl Resolver {
    pub fn with_official_registry() -> Self {
        let mut registry = HashMap::new();
        for (name, versions) in [
            ("filesystem", vec!["0.1.0"]),
            ("math", vec!["0.1.0"]),
            ("json", vec!["0.1.0"]),
            ("navigator", vec!["0.1.0", "1.0.0"]),
            ("ai", vec!["0.1.0"]),
            ("database", vec!["0.1.0"]),
            ("network", vec!["0.1.0"]),
        ] {
            registry.insert(
                name.to_string(),
                versions.into_iter().map(String::from).collect(),
            );
        }
        Self { registry }
    }

    pub fn resolve(&self, manifest: &Manifest) -> Result<LockFile, String> {
        let mut lock = LockFile {
            version: 1,
            packages: vec![],
        };
        let mut seen: HashMap<String, String> = HashMap::new();

        for (name, spec) in manifest.dependencies.iter() {
            let resolved = self.resolve_one(name, spec)?;
            if let Some(existing) = seen.get(name) {
                if existing != &resolved.version {
                    return Err(format!(
                        "dependency conflict: {name} requires both {existing} and {}",
                        resolved.version
                    ));
                }
            } else {
                seen.insert(name.clone(), resolved.version.clone());
                lock.packages.push(resolved);
            }
        }
        Ok(lock)
    }

    fn resolve_one(&self, name: &str, spec: &DependencySpec) -> Result<LockedPackage, String> {
        match spec {
            DependencySpec::Version(v) => {
                let req = VersionReq::parse(v)
                    .map_err(|e| format!("invalid version constraint '{v}': {e}"))?;
                let versions = self.registry.get(name).ok_or_else(|| {
                    format!("package '{name}' not found in registry (offline index)")
                })?;
                let chosen = versions
                    .iter()
                    .filter(|ver| {
                        semver::Version::parse(ver)
                            .map(|pv| req.matches(&pv))
                            .unwrap_or(false)
                    })
                    .max()
                    .ok_or_else(|| {
                        format!("no version of '{name}' satisfies '{v}'")
                    })?;
                Ok(LockedPackage {
                    name: name.to_string(),
                    version: chosen.to_string(),
                    source: "registry".to_string(),
                    checksum: String::new(),
                })
            }
            DependencySpec::Detailed(d) => {
                if let Some(path) = &d.path {
                    return Ok(LockedPackage {
                        name: name.to_string(),
                        version: d.version.clone().unwrap_or_else(|| "0.0.0".to_string()),
                        source: format!("path:{path}"),
                        checksum: String::new(),
                    });
                }
                if let Some(git) = &d.git {
                    return Ok(LockedPackage {
                        name: name.to_string(),
                        version: d.version.clone().unwrap_or_else(|| "0.0.0".to_string()),
                        source: format!("git:{git}"),
                        checksum: String::new(),
                    });
                }
                if let Some(v) = &d.version {
                    return self.resolve_one(name, &DependencySpec::Version(v.clone()));
                }
                Err(format!("dependency '{name}' has no version, path, or git source"))
            }
        }
    }

    pub fn search(&self, query: &str) -> Vec<(String, Vec<String>)> {
        self.registry
            .iter()
            .filter(|(name, _)| name.contains(query) || query.is_empty())
            .map(|(name, vers)| (name.clone(), vers.clone()))
            .collect()
    }
}

pub fn install_lock(lock: &LockFile, cache_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(cache_dir).map_err(|e| e.to_string())?;
    for pkg in &lock.packages {
        let dir = cache_dir.join(format!("{}-{}", pkg.name, pkg.version));
        if dir.is_dir() {
            continue;
        }
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let lib = dir.join("lib.lp");
        std::fs::write(
            &lib,
            format!("@ package {} v{}\n", pkg.name, pkg.version),
        )
        .map_err(|e| e.to_string())?;
        std::fs::write(
            dir.join("langp.toml"),
            format!(
                "[package]\nname = \"{}\"\nversion = \"{}\"\n",
                pkg.name, pkg.version
            ),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
