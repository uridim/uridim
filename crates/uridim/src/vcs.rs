use std::io;
use std::path::Path;

use crate::candidate::{Candidate, Evidence, VcsEvidence};
use crate::discovery::{EntryKind, PathSpec, find_path_in_ancestors};

pub(crate) const VCS_SPECS: &[PathSpec<VcsEvidence>] = &[PathSpec {
    relative_path: ".git",
    kind: EntryKind::Directory,
    evidence: VcsEvidence::Git,
}];

pub fn find_vcs_candidates(start: &Path) -> io::Result<Vec<Candidate>> {
    find_path_in_ancestors(start, VCS_SPECS, Evidence::Vcs)
}
