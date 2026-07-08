use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Candidate {
    pub name: String,
    pub path: PathBuf,
    pub classification: Classification,
    pub source_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Root(RootEvidence),
    Component(ComponentEvidence),
    Build(BuildEvidence),
    Operational(OperationalEvidence),
    Excluded(ExcludedEvidence),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootEvidence {
    Git,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentEvidence {
    Cargo,
    NodeJs,
    Python,
    Go,
    Maven,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildEvidence {
    CMake,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalEvidence {
    DockerCompose,
    Kubernetes,
    Supabase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExcludedEvidence {
    CargoTarget,
    NodeModules,
    NextBuild,
    Dist,
}
