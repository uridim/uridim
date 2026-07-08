use std::fs;

use crate::framework::{FrameworkError, FrameworkEvidence, FrameworkKind, detect_frameworks};

#[test]
fn detects_nextjs_from_dependencies() {
    let temp_dir = tempfile::tempdir().unwrap();
    let package_json_path = temp_dir.path().join("package.json");

    fs::write(
        &package_json_path,
        r#"{
            "dependencies": {
                "next": "16.0.0"
            }
        }"#,
    )
    .unwrap();

    let frameworks = detect_frameworks(temp_dir.path()).unwrap();

    assert_eq!(
        frameworks,
        vec![FrameworkEvidence {
            framework: FrameworkKind::NextJs,
            source_path: package_json_path,
        }]
    );
}

#[test]
fn detects_nextjs_from_dev_dependencies() {
    let temp_dir = tempfile::tempdir().unwrap();
    let package_json_path = temp_dir.path().join("package.json");

    fs::write(
        &package_json_path,
        r#"{
            "devDependencies": {
                "next": "16.0.0"
            }
        }"#,
    )
    .unwrap();

    let frameworks = detect_frameworks(temp_dir.path()).unwrap();

    assert_eq!(
        frameworks,
        vec![FrameworkEvidence {
            framework: FrameworkKind::NextJs,
            source_path: package_json_path,
        }]
    );
}

#[test]
fn does_not_detect_nextjs_from_unrelated_dependencies() {
    let temp_dir = tempfile::tempdir().unwrap();

    fs::write(
        temp_dir.path().join("package.json"),
        r#"{
            "dependencies": {
                "react": "19.0.0"
            }
        }"#,
    )
    .unwrap();

    let frameworks = detect_frameworks(temp_dir.path()).unwrap();

    assert!(frameworks.is_empty());
}

#[test]
fn does_not_detect_nextjs_from_script_name() {
    let temp_dir = tempfile::tempdir().unwrap();

    fs::write(
        temp_dir.path().join("package.json"),
        r#"{
            "scripts": {
                "next": "echo not-a-framework"
            }
        }"#,
    )
    .unwrap();

    let frameworks = detect_frameworks(temp_dir.path()).unwrap();

    assert!(frameworks.is_empty());
}

#[test]
fn returns_empty_when_package_json_is_absent() {
    let temp_dir = tempfile::tempdir().unwrap();

    let frameworks = detect_frameworks(temp_dir.path()).unwrap();

    assert!(frameworks.is_empty());
}

#[test]
fn returns_error_for_malformed_package_json() {
    let temp_dir = tempfile::tempdir().unwrap();
    let package_json_path = temp_dir.path().join("package.json");

    fs::write(
        &package_json_path,
        r#"{
            "dependencies":
        }"#,
    )
    .unwrap();

    let error = detect_frameworks(temp_dir.path()).unwrap_err();

    match error {
        FrameworkError::ParseManifest { path, .. } => {
            assert_eq!(path, package_json_path);
        }
        other => panic!("expected parse manifest error, got {other:?}"),
    }
}

#[test]
fn does_not_search_ancestor_directories() {
    let temp_dir = tempfile::tempdir().unwrap();
    let child_dir = temp_dir.path().join("nested");

    fs::create_dir(&child_dir).unwrap();

    fs::write(
        temp_dir.path().join("package.json"),
        r#"{
            "dependencies": {
                "next": "16.0.0"
            }
        }"#,
    )
    .unwrap();

    let frameworks = detect_frameworks(&child_dir).unwrap();

    assert!(frameworks.is_empty());
}
