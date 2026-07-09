use std::path::PathBuf;

use crate::candidate::{
    BuildSystemEvidence, Candidate, EcosystemEvidence, Evidence, FrameworkEvidence,
    InfrastructureEvidence, VcsEvidence,
};

#[test]
fn creates_vcs_candidate() {
    let candidate = Candidate {
        name: "uridim".to_string(),
        scope_path: PathBuf::from("/projects/uridim"),
        evidence: Evidence::Vcs(VcsEvidence::Git),
        source_path: PathBuf::from("/projects/uridim/.git"),
    };

    assert_eq!(candidate.name, "uridim");
    assert_eq!(candidate.scope_path, PathBuf::from("/projects/uridim"));
    assert_eq!(candidate.evidence, Evidence::Vcs(VcsEvidence::Git));
    assert_eq!(
        candidate.source_path,
        PathBuf::from("/projects/uridim/.git")
    );
}

#[test]
fn creates_ecosystem_candidate() {
    let candidate = Candidate {
        name: "backend".to_string(),
        scope_path: PathBuf::from("/projects/app/backend"),
        evidence: Evidence::Ecosystem(EcosystemEvidence::Cargo),
        source_path: PathBuf::from("/projects/app/backend/Cargo.toml"),
    };

    assert_eq!(
        candidate.evidence,
        Evidence::Ecosystem(EcosystemEvidence::Cargo)
    );
}

#[test]
fn creates_framework_candidate() {
    let candidate = Candidate {
        name: "frontend".to_string(),
        scope_path: PathBuf::from("/projects/app/frontend"),
        evidence: Evidence::Framework(FrameworkEvidence::NextJs),
        source_path: PathBuf::from("/projects/app/frontend/package.json"),
    };

    assert_eq!(
        candidate.evidence,
        Evidence::Framework(FrameworkEvidence::NextJs)
    );
}

#[test]
fn creates_build_system_candidate() {
    let candidate = Candidate {
        name: "native".to_string(),
        scope_path: PathBuf::from("/projects/app/native"),
        evidence: Evidence::BuildSystem(BuildSystemEvidence::CMake),
        source_path: PathBuf::from("/projects/app/native/CMakeLists.txt"),
    };

    assert_eq!(
        candidate.evidence,
        Evidence::BuildSystem(BuildSystemEvidence::CMake)
    );
}

#[test]
fn creates_infrastructure_candidate() {
    let candidate = Candidate {
        name: "infra".to_string(),
        scope_path: PathBuf::from("/projects/app/infra"),
        evidence: Evidence::Infrastructure(InfrastructureEvidence::DockerCompose),
        source_path: PathBuf::from("/projects/app/infra/compose.yaml"),
    };

    assert_eq!(
        candidate.evidence,
        Evidence::Infrastructure(InfrastructureEvidence::DockerCompose)
    );
}
