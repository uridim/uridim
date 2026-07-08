use std::fs;
use std::io;
use std::path::Path;

use crate::candidate::{
    BuildEvidence, Candidate, Classification, ComponentEvidence, OperationalEvidence, RootEvidence,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct MarkerSpec<Evidence> {
    pub(crate) relative_path: &'static str,
    pub(crate) evidence: Evidence,
}

pub(crate) const ROOT_MARKERS: &[MarkerSpec<RootEvidence>] = &[];

pub(crate) const COMPONENT_MARKERS: &[MarkerSpec<ComponentEvidence>] = &[
    MarkerSpec {
        relative_path: "Cargo.toml",
        evidence: ComponentEvidence::Cargo,
    },
    MarkerSpec {
        relative_path: "package.json",
        evidence: ComponentEvidence::NodeJs,
    },
    MarkerSpec {
        relative_path: "pyproject.toml",
        evidence: ComponentEvidence::Python,
    },
    MarkerSpec {
        relative_path: "go.mod",
        evidence: ComponentEvidence::Go,
    },
    MarkerSpec {
        relative_path: "pom.xml",
        evidence: ComponentEvidence::Maven,
    },
];

pub(crate) const BUILD_MARKERS: &[MarkerSpec<BuildEvidence>] = &[MarkerSpec {
    relative_path: "CMakeLists.txt",
    evidence: BuildEvidence::CMake,
}];

pub(crate) const OPERATIONAL_MARKERS: &[MarkerSpec<OperationalEvidence>] = &[
    MarkerSpec {
        relative_path: "compose.yml",
        evidence: OperationalEvidence::DockerCompose,
    },
    MarkerSpec {
        relative_path: "compose.yaml",
        evidence: OperationalEvidence::DockerCompose,
    },
    MarkerSpec {
        relative_path: "docker-compose.yml",
        evidence: OperationalEvidence::DockerCompose,
    },
    MarkerSpec {
        relative_path: "docker-compose.yaml",
        evidence: OperationalEvidence::DockerCompose,
    },
    MarkerSpec {
        relative_path: "kustomization.yml",
        evidence: OperationalEvidence::Kubernetes,
    },
    MarkerSpec {
        relative_path: "kustomization.yaml",
        evidence: OperationalEvidence::Kubernetes,
    },
    MarkerSpec {
        relative_path: "supabase/config.toml",
        evidence: OperationalEvidence::Supabase,
    },
];

fn find_candidates<Evidence>(
    start: &Path,
    markers: &[MarkerSpec<Evidence>],
    classify: impl Fn(Evidence) -> Classification,
) -> io::Result<Vec<Candidate>>
where
    Evidence: Copy,
{
    let mut candidates = Vec::new();

    for directory in start.ancestors() {
        for marker in markers {
            let source_path = directory.join(marker.relative_path);

            match fs::metadata(&source_path) {
                Ok(metadata) if metadata.is_file() => {
                    let name = directory
                        .file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or_else(|| directory.display().to_string());

                    candidates.push(Candidate {
                        name,
                        path: directory.to_path_buf(),
                        classification: classify(marker.evidence),
                        source_path,
                    });
                }
                Ok(_) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
    }

    Ok(candidates)
}

pub fn find_root_candidates(start: &Path) -> io::Result<Vec<Candidate>> {
    find_candidates(start, ROOT_MARKERS, Classification::Root)
}

pub fn find_component_candidates(start: &Path) -> io::Result<Vec<Candidate>> {
    find_candidates(start, COMPONENT_MARKERS, Classification::Component)
}

pub fn find_build_candidates(start: &Path) -> io::Result<Vec<Candidate>> {
    find_candidates(start, BUILD_MARKERS, Classification::Build)
}

pub fn find_operational_candidates(start: &Path) -> io::Result<Vec<Candidate>> {
    find_candidates(start, OPERATIONAL_MARKERS, Classification::Operational)
}
