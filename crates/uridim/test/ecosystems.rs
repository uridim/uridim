use std::fs;
use std::io;

use tempfile::tempdir;

use crate::candidate::{EcosystemEvidence, Evidence};
use crate::ecosystems::find_ecosystem_candidates;

#[test]
fn detects_each_supported_ecosystem() -> io::Result<()> {
    let cases = [
        ("Cargo.toml", EcosystemEvidence::Cargo),
        ("package.json", EcosystemEvidence::NodeJs),
        ("pyproject.toml", EcosystemEvidence::Python),
        ("go.mod", EcosystemEvidence::Go),
        ("pom.xml", EcosystemEvidence::Maven),
    ];

    for (relative_path, expected_evidence) in cases {
        let temp = tempdir()?;
        let project = temp.path().join("project");
        let source_path = project.join(relative_path);

        fs::create_dir_all(&project)?;
        fs::write(&source_path, b"test")?;

        let candidates = find_ecosystem_candidates(&project)?;

        assert_eq!(
            candidates.len(),
            1,
            "expected one candidate for {relative_path}"
        );

        let candidate = &candidates[0];

        assert_eq!(candidate.scope_path, project);
        assert_eq!(candidate.evidence, Evidence::Ecosystem(expected_evidence));
        assert_eq!(candidate.source_path, source_path);
    }

    Ok(())
}

#[test]
fn finds_ecosystems_in_nested_descendants() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let frontend = root.join("frontend");
    let backend = root.join("backend");

    let package_json = frontend.join("package.json");
    let cargo_toml = backend.join("Cargo.toml");

    fs::create_dir_all(&frontend)?;
    fs::create_dir_all(&backend)?;

    fs::write(&package_json, b"{}")?;
    fs::write(&cargo_toml, b"[package]\nname = \"backend\"\n")?;

    let candidates = find_ecosystem_candidates(&root)?;

    assert_eq!(candidates.len(), 2);

    assert!(candidates.iter().any(|candidate| {
        candidate.scope_path == frontend
            && candidate.source_path == package_json
            && candidate.evidence == Evidence::Ecosystem(EcosystemEvidence::NodeJs)
    }));

    assert!(candidates.iter().any(|candidate| {
        candidate.scope_path == backend
            && candidate.source_path == cargo_toml
            && candidate.evidence == Evidence::Ecosystem(EcosystemEvidence::Cargo)
    }));

    Ok(())
}

#[test]
fn finds_multiple_ecosystems_in_same_scope() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");

    fs::create_dir_all(&root)?;
    fs::write(root.join("Cargo.toml"), b"[package]\nname = \"app\"\n")?;
    fs::write(root.join("package.json"), b"{}")?;

    let candidates = find_ecosystem_candidates(&root)?;

    assert_eq!(candidates.len(), 2);

    assert!(
        candidates.iter().any(|candidate| {
            candidate.evidence == Evidence::Ecosystem(EcosystemEvidence::Cargo)
        })
    );

    assert!(
        candidates.iter().any(|candidate| {
            candidate.evidence == Evidence::Ecosystem(EcosystemEvidence::NodeJs)
        })
    );

    Ok(())
}

#[test]
fn prunes_default_excluded_directories() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let excluded = root.join("node_modules").join("nested");

    fs::create_dir_all(&excluded)?;
    fs::write(
        excluded.join("Cargo.toml"),
        b"[package]\nname = \"hidden\"\n",
    )?;

    let candidates = find_ecosystem_candidates(&root)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn ignores_directory_named_like_manifest() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");

    fs::create_dir_all(root.join("Cargo.toml"))?;

    let candidates = find_ecosystem_candidates(&root)?;

    assert!(candidates.is_empty());

    Ok(())
}
