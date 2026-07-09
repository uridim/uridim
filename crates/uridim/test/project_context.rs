use std::path::PathBuf;

use crate::candidate::{
    BuildSystemEvidence, Candidate, EcosystemEvidence, Evidence, FrameworkEvidence,
    InfrastructureEvidence, VcsEvidence,
};
use crate::project_context::build_project_context;

#[test]
fn groups_candidates_by_evidence_category() {
    let root = PathBuf::from("/projects/uridim");

    let candidates = vec![
        Candidate {
            name: "uridim".to_string(),
            scope_path: root.clone(),
            evidence: Evidence::Vcs(VcsEvidence::Git),
            source_path: root.join(".git"),
        },
        Candidate {
            name: "uridim".to_string(),
            scope_path: root.clone(),
            evidence: Evidence::Ecosystem(EcosystemEvidence::Cargo),
            source_path: root.join("Cargo.toml"),
        },
        Candidate {
            name: "frontend".to_string(),
            scope_path: root.join("frontend"),
            evidence: Evidence::Framework(FrameworkEvidence::NextJs),
            source_path: root.join("frontend/package.json"),
        },
        Candidate {
            name: "native".to_string(),
            scope_path: root.join("native"),
            evidence: Evidence::BuildSystem(BuildSystemEvidence::CMake),
            source_path: root.join("native/CMakeLists.txt"),
        },
        Candidate {
            name: "infra".to_string(),
            scope_path: root.join("infra"),
            evidence: Evidence::Infrastructure(InfrastructureEvidence::DockerCompose),
            source_path: root.join("infra/compose.yaml"),
        },
    ];

    let context = build_project_context(root.clone(), candidates);

    assert_eq!(context.root, root);
    assert_eq!(context.vcs.len(), 1);
    assert_eq!(context.ecosystems.len(), 1);
    assert_eq!(context.frameworks.len(), 1);
    assert_eq!(context.build_systems.len(), 1);
    assert_eq!(context.infrastructure.len(), 1);

    assert_eq!(context.vcs[0].evidence, VcsEvidence::Git);
    assert_eq!(context.ecosystems[0].evidence, EcosystemEvidence::Cargo);
    assert_eq!(context.frameworks[0].evidence, FrameworkEvidence::NextJs);
    assert_eq!(
        context.build_systems[0].evidence,
        BuildSystemEvidence::CMake
    );
    assert_eq!(
        context.infrastructure[0].evidence,
        InfrastructureEvidence::DockerCompose
    );
}

#[test]
fn preserves_scope_and_source_paths() {
    let root = PathBuf::from("/projects/uridim");
    let scope_path = root.join("crates/uridim");
    let source_path = scope_path.join("Cargo.toml");

    let candidates = vec![Candidate {
        name: "uridim".to_string(),
        scope_path: scope_path.clone(),
        evidence: Evidence::Ecosystem(EcosystemEvidence::Cargo),
        source_path: source_path.clone(),
    }];

    let context = build_project_context(root, candidates);

    assert_eq!(context.ecosystems.len(), 1);
    assert_eq!(context.ecosystems[0].scope_path, scope_path);
    assert_eq!(context.ecosystems[0].source_path, source_path);
}

#[test]
fn preserves_multiple_candidates_in_same_category() {
    let root = PathBuf::from("/projects/uridim");

    let candidates = vec![
        Candidate {
            name: "uridim".to_string(),
            scope_path: root.clone(),
            evidence: Evidence::Ecosystem(EcosystemEvidence::Cargo),
            source_path: root.join("Cargo.toml"),
        },
        Candidate {
            name: "uridim".to_string(),
            scope_path: root.join("crates/uridim"),
            evidence: Evidence::Ecosystem(EcosystemEvidence::Cargo),
            source_path: root.join("crates/uridim/Cargo.toml"),
        },
    ];

    let context = build_project_context(root, candidates);

    assert_eq!(context.ecosystems.len(), 2);
}

#[test]
fn builds_empty_context_when_no_candidates_exist() {
    let root = PathBuf::from("/projects/uridim");

    let context = build_project_context(root.clone(), Vec::new());

    assert_eq!(context.root, root);
    assert!(context.vcs.is_empty());
    assert!(context.ecosystems.is_empty());
    assert!(context.frameworks.is_empty());
    assert!(context.build_systems.is_empty());
    assert!(context.infrastructure.is_empty());
}
