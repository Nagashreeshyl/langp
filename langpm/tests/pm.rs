use langpm::{Manifest, Resolver};

#[test]
fn manifest_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("langp.toml");
    let manifest = Manifest {
        package: langpm::PackageMeta {
            name: "demo".into(),
            version: "0.1.0".into(),
            entry: "main.lp".into(),
            description: "test".into(),
        },
        dependencies: Default::default(),
        dev_dependencies: Default::default(),
    };
    manifest.save(&path).unwrap();
    let loaded = Manifest::load(&path).unwrap();
    assert_eq!(loaded.package.name, "demo");
}

#[test]
fn resolver_filesystem() {
    let mut manifest = Manifest {
        package: langpm::PackageMeta {
            name: "app".into(),
            version: "0.1.0".into(),
            entry: "main.lp".into(),
            description: String::new(),
        },
        dependencies: [(
            "filesystem".to_string(),
            langpm::DependencySpec::Version("0.1".into()),
        )]
        .into(),
        dev_dependencies: Default::default(),
    };
    let resolver = Resolver::with_official_registry();
    let lock = resolver.resolve(&manifest).unwrap();
    assert!(lock.packages.iter().any(|p| p.name == "filesystem"));
    let _ = &mut manifest;
}

#[test]
fn search_packages() {
    let resolver = Resolver::with_official_registry();
    let hits = resolver.search("nav");
    assert!(hits.iter().any(|(n, _)| n == "navigator"));
}
