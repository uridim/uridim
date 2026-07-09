use std::io;
use std::path::Path;

use crate::candidate::{BuildSystemEvidence, Candidate, Evidence};
use crate::discovery::{EntryKind, PathSpec, find_path_in_descendants};
use crate::exclusion::DEFAULT_EXCLUSIONS;

pub(crate) const BUILD_SYSTEM_SPECS: &[PathSpec<BuildSystemEvidence>] = &[PathSpec {
    relative_path: "CMakeLists.txt",
    kind: EntryKind::File,
    evidence: BuildSystemEvidence::CMake,
}];

pub fn find_build_system_candidates(root: &Path) -> io::Result<Vec<Candidate>> {
    find_path_in_descendants(
        root,
        BUILD_SYSTEM_SPECS,
        DEFAULT_EXCLUSIONS,
        Evidence::BuildSystem,
    )
}
