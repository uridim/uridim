use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Candidate {
    pub name: String,
    pub path: PathBuf,
    pub classification: Classification,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Root(RootEvidence),
    Component(ComponentEvidence),
    Build(BuildEvidence),
    Operational(OperationalEvidence),
    Excluded(ExcludedEvidence),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RootEvidence {
    Git,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ComponentEvidence {
    Cargo,
    NodeJs,
    NextJs,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildEvidence {
    CMake,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperationalEvidence {
    DockerCompose,
    Kubernetes,
    Supabase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExcludedEvidence {
    CargoTarget,
    NodeModules,
    NextBuild,
    Dist,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_git_root_candidate() {
        let candidate = Candidate {
            name: "uridim".to_string(),
            path: PathBuf::from("/projects/uridim"),
            classification: Classification::Root(RootEvidence::Git),
        };

        assert_eq!(candidate.name, "uridim");
        assert_eq!(
            candidate.classification,
            Classification::Root(RootEvidence::Git)
        );
    }

    #[test]
    fn creates_cmake_build_candidate() {
        let candidate = Candidate {
            name: "cmake".to_string(),
            path: PathBuf::from("/projects/app/cmake"),
            classification: Classification::Build(BuildEvidence::CMake),
        };

        assert_eq!(
            candidate.classification,
            Classification::Build(BuildEvidence::CMake)
        );
    }

    #[test]
    fn creates_supabase_operational_candidate() {
        let candidate = Candidate {
            name: "supabase".to_string(),
            path: PathBuf::from("/projects/app/supabase"),
            classification: Classification::Operational(OperationalEvidence::Supabase),
        };

        assert_eq!(
            candidate.classification,
            Classification::Operational(OperationalEvidence::Supabase)
        );
    }

    #[test]
    fn creates_node_component_candidate() {
        let candidate = Candidate {
            name: "gateway".to_string(),
            path: PathBuf::from("/projects/app/gateway"),
            classification: Classification::Component(ComponentEvidence::NodeJs),
        };

        assert_eq!(
            candidate.classification,
            Classification::Component(ComponentEvidence::NodeJs)
        );
    }

    #[test]
    fn creates_dist_excluded_candidate() {
        let candidate = Candidate {
            name: "dist".to_string(),
            path: PathBuf::from("/projects/app/dist"),
            classification: Classification::Excluded(ExcludedEvidence::Dist),
        };

        assert_eq!(
            candidate.classification,
            Classification::Excluded(ExcludedEvidence::Dist)
        );
    }
}
