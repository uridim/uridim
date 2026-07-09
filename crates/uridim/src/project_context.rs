use std::path::PathBuf;

use crate::candidate::{
    BuildSystemEvidence, Candidate, EcosystemEvidence, Evidence, FrameworkEvidence,
    InfrastructureEvidence, VcsEvidence,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ProjectContext {
    pub root: PathBuf,
    pub vcs: Vec<Detected<VcsEvidence>>,
    pub ecosystems: Vec<Detected<EcosystemEvidence>>,
    pub frameworks: Vec<Detected<FrameworkEvidence>>,
    pub build_systems: Vec<Detected<BuildSystemEvidence>>,
    pub infrastructure: Vec<Detected<InfrastructureEvidence>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Detected<E> {
    pub evidence: E,
    pub scope_path: PathBuf,
    pub source_path: PathBuf,
}

pub fn build_project_context(root: PathBuf, candidates: Vec<Candidate>) -> ProjectContext {
    let mut context = ProjectContext {
        root,
        vcs: Vec::new(),
        ecosystems: Vec::new(),
        frameworks: Vec::new(),
        build_systems: Vec::new(),
        infrastructure: Vec::new(),
    };

    for candidate in candidates {
        let Candidate {
            scope_path,
            evidence,
            source_path,
            ..
        } = candidate;

        match evidence {
            Evidence::Vcs(evidence) => {
                context.vcs.push(Detected {
                    evidence,
                    scope_path,
                    source_path,
                });
            }
            Evidence::Ecosystem(evidence) => {
                context.ecosystems.push(Detected {
                    evidence,
                    scope_path,
                    source_path,
                });
            }
            Evidence::Framework(evidence) => {
                context.frameworks.push(Detected {
                    evidence,
                    scope_path,
                    source_path,
                });
            }
            Evidence::BuildSystem(evidence) => {
                context.build_systems.push(Detected {
                    evidence,
                    scope_path,
                    source_path,
                });
            }
            Evidence::Infrastructure(evidence) => {
                context.infrastructure.push(Detected {
                    evidence,
                    scope_path,
                    source_path,
                });
            }
        }
    }

    context
}
