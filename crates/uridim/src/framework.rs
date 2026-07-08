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
