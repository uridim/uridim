use std::io;
use std::path::Path;

use crate::candidate::{Candidate, Evidence, InfrastructureEvidence};
use crate::discovery::{EntryKind, PathSpec, find_path_in_descendants};
use crate::exclusion::DEFAULT_EXCLUSIONS;

pub(crate) const INFRASTRUCTURE_SPECS: &[PathSpec<InfrastructureEvidence>] = &[
    PathSpec {
        relative_path: "compose.yml",
        kind: EntryKind::File,
        evidence: InfrastructureEvidence::DockerCompose,
    },
    PathSpec {
        relative_path: "compose.yaml",
        kind: EntryKind::File,
        evidence: InfrastructureEvidence::DockerCompose,
    },
    PathSpec {
        relative_path: "docker-compose.yml",
        kind: EntryKind::File,
        evidence: InfrastructureEvidence::DockerCompose,
    },
    PathSpec {
        relative_path: "docker-compose.yaml",
        kind: EntryKind::File,
        evidence: InfrastructureEvidence::DockerCompose,
    },
    PathSpec {
        relative_path: "kustomization.yml",
        kind: EntryKind::File,
        evidence: InfrastructureEvidence::Kubernetes,
    },
    PathSpec {
        relative_path: "kustomization.yaml",
        kind: EntryKind::File,
        evidence: InfrastructureEvidence::Kubernetes,
    },
    PathSpec {
        relative_path: "supabase/config.toml",
        kind: EntryKind::File,
        evidence: InfrastructureEvidence::Supabase,
    },
];

pub fn find_infrastructure_candidates(root: &Path) -> io::Result<Vec<Candidate>> {
    find_path_in_descendants(
        root,
        INFRASTRUCTURE_SPECS,
        DEFAULT_EXCLUSIONS,
        Evidence::Infrastructure,
    )
}
