use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct FrameworkEvidence {
    pub framework: FrameworkKind,
    pub source_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameworkKind {
    NextJs,
}

#[derive(Debug)]
pub enum FrameworkError {
    ReadManifest {
        path: PathBuf,
        source: std::io::Error,
    },
    ParseManifest {
        path: PathBuf,
        source: serde_json::Error,
    },
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,

    #[serde(rename = "devDependencies", default)]
    dev_dependencies: HashMap<String, String>,
}

pub fn detect_frameworks(project_dir: &Path) -> Result<Vec<FrameworkEvidence>, FrameworkError> {
    let package_json_path = project_dir.join("package.json");

    let contents = match fs::read_to_string(&package_json_path) {
        Ok(contents) => contents,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Vec::new());
        }
        Err(source) => {
            return Err(FrameworkError::ReadManifest {
                path: package_json_path,
                source,
            });
        }
    };

    let package_json: PackageJson =
        serde_json::from_str(&contents).map_err(|source| FrameworkError::ParseManifest {
            path: package_json_path.clone(),
            source,
        })?;

    let has_next = package_json.dependencies.contains_key("next")
        || package_json.dev_dependencies.contains_key("next");

    if has_next {
        return Ok(vec![FrameworkEvidence {
            framework: FrameworkKind::NextJs,
            source_path: package_json_path,
        }]);
    }

    Ok(Vec::new())
}

// ===== TEST ===== //
#[cfg(test)]
mod tests {
    use super::*;

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
}
