use std::fs;
use std::io;

use tempfile::tempdir;

use crate::candidate::{Evidence, FrameworkEvidence};
use crate::frameworks::find_framework_candidates;

#[test]
fn detects_next_js_from_dependencies() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("frontend");
    let package_json = project.join("package.json");

    fs::create_dir_all(&project)?;
    fs::write(
        &package_json,
        r#"{
            "dependencies": {
                "next": "15.0.0"
            }
        }"#,
    )?;

    let candidates = find_framework_candidates(&project)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.scope_path, project);
    assert_eq!(
        candidate.evidence,
        Evidence::Framework(FrameworkEvidence::NextJs)
    );
    assert_eq!(candidate.source_path, package_json);

    Ok(())
}

#[test]
fn detects_next_js_from_dev_dependencies() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("frontend");

    fs::create_dir_all(&project)?;
    fs::write(
        project.join("package.json"),
        r#"{
            "devDependencies": {
                "next": "15.0.0"
            }
        }"#,
    )?;

    let candidates = find_framework_candidates(&project)?;

    assert_eq!(candidates.len(), 1);
    assert_eq!(
        candidates[0].evidence,
        Evidence::Framework(FrameworkEvidence::NextJs)
    );

    Ok(())
}

#[test]
fn finds_frameworks_in_nested_descendants() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let frontend = root.join("apps").join("web");

    fs::create_dir_all(&frontend)?;
    fs::write(
        frontend.join("package.json"),
        r#"{
            "dependencies": {
                "next": "15.0.0"
            }
        }"#,
    )?;

    let candidates = find_framework_candidates(&root)?;

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].scope_path, frontend);

    Ok(())
}

#[test]
fn ignores_package_json_without_supported_framework() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("frontend");

    fs::create_dir_all(&project)?;
    fs::write(
        project.join("package.json"),
        r#"{
            "dependencies": {
                "react": "19.0.0"
            }
        }"#,
    )?;

    let candidates = find_framework_candidates(&project)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn malformed_package_json_does_not_produce_framework_candidate() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("frontend");

    fs::create_dir_all(&project)?;
    fs::write(project.join("package.json"), b"{ invalid json")?;

    let candidates = find_framework_candidates(&project)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn prunes_framework_manifests_in_excluded_directories() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let excluded = root.join("node_modules").join("some-package");

    fs::create_dir_all(&excluded)?;
    fs::write(
        excluded.join("package.json"),
        r#"{
            "dependencies": {
                "next": "15.0.0"
            }
        }"#,
    )?;

    let candidates = find_framework_candidates(&root)?;

    assert!(candidates.is_empty());

    Ok(())
}
