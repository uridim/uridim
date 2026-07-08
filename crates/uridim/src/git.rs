use std::io;
use std::path::Path;

use crate::candidate::{Candidate, Classification, RootEvidence};

pub fn find_git_root(start: &Path) -> io::Result<Option<Candidate>> {
    for directory in start.ancestors() {
        let git_marker = directory.join(".git");

        if git_marker.try_exists()? {
            let name = directory
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| directory.display().to_string());

            return Ok(Some(Candidate {
                name,
                path: directory.to_path_buf(),
                classification: Classification::Root(RootEvidence::Git),
                source_path: git_marker,
            }));
        }
    }

    Ok(None)
}
