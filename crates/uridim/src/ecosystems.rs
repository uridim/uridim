use std::io;
use std::path::Path;

use crate::candidate::{Candidate, EcosystemEvidence, Evidence};
use crate::discovery::{EntryKind, PathSpec, find_path_in_descendants};
use crate::exclusion::DEFAULT_EXCLUSIONS;

pub(crate) const ECOSYSTEM_SPECS: &[PathSpec<EcosystemEvidence>] = &[
    PathSpec {
        relative_path: "Cargo.toml",
        kind: EntryKind::File,
        evidence: EcosystemEvidence::Cargo,
    },
    PathSpec {
        relative_path: "package.json",
        kind: EntryKind::File,
        evidence: EcosystemEvidence::NodeJs,
    },
    PathSpec {
        relative_path: "pyproject.toml",
        kind: EntryKind::File,
        evidence: EcosystemEvidence::Python,
    },
    PathSpec {
        relative_path: "go.mod",
        kind: EntryKind::File,
        evidence: EcosystemEvidence::Go,
    },
    PathSpec {
        relative_path: "pom.xml",
        kind: EntryKind::File,
        evidence: EcosystemEvidence::Maven,
    },
];

pub fn find_ecosystem_candidates(root: &Path) -> io::Result<Vec<Candidate>> {
    find_path_in_descendants(
        root,
        ECOSYSTEM_SPECS,
        DEFAULT_EXCLUSIONS,
        Evidence::Ecosystem,
    )
}
