//! Lang.P package manager — manifest, lock file, dependency resolution, CLI helpers.

mod lockfile;
mod manifest;
mod resolver;

pub use lockfile::{LockFile, LockedPackage};
pub use manifest::{DependencySpec, Manifest, PackageMeta};
pub use resolver::{install_lock, Resolver};

use std::path::PathBuf;
use std::process::Command;

pub fn project_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn cmd_init(name: Option<&str>) -> Result<(), String> {
    let root = project_root();
    let name = name.unwrap_or(
        root.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("my-project"),
    );
    let path = manifest::Manifest::init_project(&root, name)?;
    println!("✓ initialized Lang.P project '{name}'");
    println!("  manifest: {}", path.display());
    println!("  entry:    main.lp");
    Ok(())
}

pub fn cmd_install(package: Option<&str>) -> Result<(), String> {
    let root = project_root();
    let mut manifest = Manifest::load_from_dir(&root)?;
    if let Some(pkg) = package {
        manifest
            .dependencies
            .insert(pkg.to_string(), manifest::DependencySpec::Version("0.1".into()));
        manifest.save(&root.join("langp.toml"))?;
        println!("✓ added dependency '{pkg}'");
    }
    let resolver = Resolver::with_official_registry();
    let lock = resolver.resolve(&manifest)?;
    let lock_path = root.join("langp.lock");
    lock.save(&lock_path)?;
    let cache = cache_dir();
    install_lock(&lock, &cache)?;
    println!("✓ installed {} package(s) to {}", lock.packages.len(), cache.display());
    println!("  lock file: {}", lock_path.display());
    Ok(())
}

pub fn cmd_remove(package: &str) -> Result<(), String> {
    let root = project_root();
    let mut manifest = Manifest::load_from_dir(&root)?;
    if manifest.dependencies.remove(package).is_none() {
        return Err(format!("dependency '{package}' not in manifest"));
    }
    manifest.save(&root.join("langp.toml"))?;
    cmd_install(None)?;
    println!("✓ removed '{package}'");
    Ok(())
}

pub fn cmd_update(_package: Option<&str>) -> Result<(), String> {
    cmd_install(None)?;
    println!("✓ dependencies updated");
    Ok(())
}

pub fn cmd_search(query: &str) -> Result<(), String> {
    let resolver = Resolver::with_official_registry();
    let hits = resolver.search(query);
    if hits.is_empty() {
        println!("no packages matching '{query}'");
        return Ok(());
    }
    for (name, versions) in hits {
        println!("{name} ({})", versions.join(", "));
    }
    Ok(())
}

pub fn cmd_doctor() -> Result<(), String> {
    let root = project_root();
    print!("lang ... ");
    if which("lang") {
        println!("ok");
    } else {
        println!("missing");
    }
    print!("langc ... ");
    if which("langc") {
        println!("ok");
    } else {
        println!("missing");
    }
    print!("manifest ... ");
    match Manifest::load_from_dir(&root) {
        Ok(m) => println!("ok ({})", m.package.name),
        Err(e) => println!("{e}"),
    }
    print!("cache ... ");
    let cache = cache_dir();
    if cache.is_dir() {
        println!("ok ({})", cache.display());
    } else {
        println!("not created yet");
    }
    Ok(())
}

pub fn cmd_build() -> Result<(), String> {
    let root = project_root();
    let manifest = Manifest::load_from_dir(&root)?;
    let entry = root.join(&manifest.package.entry);
    if !entry.is_file() {
        return Err(format!("entry file not found: {}", entry.display()));
    }
    let status = Command::new("lang")
        .arg("check")
        .arg(&entry)
        .status()
        .map_err(|e| format!("failed to run lang check: {e}"))?;
    if !status.success() {
        return Err("build failed: lang check reported errors".into());
    }
    println!("✓ build ok — {}", entry.display());
    Ok(())
}

pub fn cmd_test() -> Result<(), String> {
    let root = project_root();
    let tests_dir = root.join("tests");
    if !tests_dir.is_dir() {
        println!("no tests/ directory");
        return Ok(());
    }
    let mut ran = 0;
    for entry in std::fs::read_dir(&tests_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "lp") {
            ran += 1;
            print!("{} ... ", path.file_name().unwrap().to_string_lossy());
            let status = Command::new("lang")
                .arg("check")
                .arg(&path)
                .status()
                .map_err(|e| e.to_string())?;
            if status.success() {
                println!("ok");
            } else {
                println!("failed");
                return Err(format!("test failed: {}", path.display()));
            }
        }
    }
    println!("✓ {ran} test file(s) checked");
    Ok(())
}

pub fn cmd_fmt() -> Result<(), String> {
    let root = project_root();
    let manifest = Manifest::load_from_dir(&root).unwrap_or(Manifest {
        package: PackageMeta {
            name: "app".into(),
            version: "0.1.0".into(),
            entry: "main.lp".into(),
            description: String::new(),
        },
        dependencies: Default::default(),
        dev_dependencies: Default::default(),
    });
    let entry = root.join(manifest.package.entry);
    let status = Command::new("lang")
        .arg("check")
        .arg(&entry)
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        println!("✓ fmt/check ok");
        Ok(())
    } else {
        Err("fmt check failed".into())
    }
}

pub fn cmd_publish() -> Result<(), String> {
    Err("lang publish requires registry authentication (not yet connected)".into())
}

pub fn cmd_login() -> Result<(), String> {
    Err("lang login requires registry API (not yet connected)".into())
}

fn cache_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".cache/langp/packages")
}

fn which(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        || Command::new("where")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
}
