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
