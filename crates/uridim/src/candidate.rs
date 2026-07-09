use std::fmt;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Candidate {
    pub name: String,
    pub scope_path: PathBuf,
    pub evidence: Evidence,
    pub source_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Evidence {
    Vcs(VcsEvidence),
    Ecosystem(EcosystemEvidence),
    Framework(FrameworkEvidence),
    BuildSystem(BuildSystemEvidence),
    Infrastructure(InfrastructureEvidence),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VcsEvidence {
    Git,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcosystemEvidence {
    Cargo,
    NodeJs,
    Python,
    Go,
    Maven,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameworkEvidence {
    NextJs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSystemEvidence {
    CMake,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfrastructureEvidence {
    DockerCompose,
    Kubernetes,
    Supabase,
}

impl fmt::Display for VcsEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git => f.write_str("Git"),
        }
    }
}

impl fmt::Display for EcosystemEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cargo => f.write_str("Cargo"),
            Self::NodeJs => f.write_str("Node.js"),
            Self::Python => f.write_str("Python"),
            Self::Go => f.write_str("Go"),
            Self::Maven => f.write_str("Maven"),
        }
    }
}

impl fmt::Display for FrameworkEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NextJs => f.write_str("Next.js"),
        }
    }
}

impl fmt::Display for BuildSystemEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CMake => f.write_str("CMake"),
        }
    }
}

impl fmt::Display for InfrastructureEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DockerCompose => f.write_str("Docker Compose"),
            Self::Kubernetes => f.write_str("Kubernetes"),
            Self::Supabase => f.write_str("Supabase"),
        }
    }
}
