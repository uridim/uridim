use std::collections::HashMap;
use std::io;
use std::path::Path;

use serde::Deserialize;

use crate::candidate::{Candidate, Evidence, FrameworkEvidence};
use crate::discovery::{
    ContentSpec, EntryKind, PathSpec, find_by_content_in_directory, find_path_in_descendants,
};
use crate::exclusion::DEFAULT_EXCLUSIONS;

#[derive(Debug, Deserialize)]
struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,

    #[serde(rename = "devDependencies", default)]
    dev_dependencies: HashMap<String, String>,
}

const FRAMEWORK_MANIFEST_SPECS: &[PathSpec<FrameworkEvidence>] = &[PathSpec {
    relative_path: "package.json",
    kind: EntryKind::File,
    evidence: FrameworkEvidence::NextJs,
}];

const FRAMEWORK_CONTENT_SPECS: &[ContentSpec<FrameworkEvidence>] = &[ContentSpec {
    relative_path: "package.json",
    evidence: FrameworkEvidence::NextJs,
    matches: contains_next_js,
}];

fn contains_next_js(contents: &str) -> bool {
    let Ok(package_json) = serde_json::from_str::<PackageJson>(contents) else {
        return false;
    };

    package_json.dependencies.contains_key("next")
        || package_json.dev_dependencies.contains_key("next")
}

pub fn find_framework_candidates(root: &Path) -> io::Result<Vec<Candidate>> {
    let manifest_candidates = find_path_in_descendants(
        root,
        FRAMEWORK_MANIFEST_SPECS,
        DEFAULT_EXCLUSIONS,
        Evidence::Framework,
    )?;

    let mut framework_candidates = Vec::new();

    for manifest_candidate in manifest_candidates {
        let mut candidates = find_by_content_in_directory(
            &manifest_candidate.scope_path,
            FRAMEWORK_CONTENT_SPECS,
            Evidence::Framework,
        )?;

        framework_candidates.append(&mut candidates);
    }

    Ok(framework_candidates)
}
