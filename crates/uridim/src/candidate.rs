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

// ===== TEST ===== //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_git_root_candidate() {
        let candidate = Candidate {
            name: "uridim".to_string(),
            path: PathBuf::from("/projects/uridim"),
            classification: Classification::Root(RootEvidence::Git),
            source_path: PathBuf::from("/projects/uridim/.git"),
        };

        assert_eq!(candidate.name, "uridim");
        assert_eq!(candidate.path, PathBuf::from("/projects/uridim"));
        assert_eq!(
            candidate.classification,
            Classification::Root(RootEvidence::Git)
        );
        assert_eq!(
            candidate.source_path,
            PathBuf::from("/projects/uridim/.git")
        );
    }

    #[test]
    fn creates_cmake_build_candidate() {
        let candidate = Candidate {
            name: "app".to_string(),
            path: PathBuf::from("/projects/app"),
            classification: Classification::Build(BuildEvidence::CMake),
            source_path: PathBuf::from("/projects/app/CMakeLists.txt"),
        };

        assert_eq!(candidate.name, "app");
        assert_eq!(candidate.path, PathBuf::from("/projects/app"));
        assert_eq!(
            candidate.classification,
            Classification::Build(BuildEvidence::CMake)
        );
        assert_eq!(
            candidate.source_path,
            PathBuf::from("/projects/app/CMakeLists.txt")
        );
    }

    #[test]
    fn creates_supabase_operational_candidate() {
        let candidate = Candidate {
            name: "app".to_string(),
            path: PathBuf::from("/projects/app"),
            classification: Classification::Operational(OperationalEvidence::Supabase),
            source_path: PathBuf::from("/projects/app/supabase/config.toml"),
        };

        assert_eq!(candidate.name, "app");
        assert_eq!(candidate.path, PathBuf::from("/projects/app"));
        assert_eq!(
            candidate.classification,
            Classification::Operational(OperationalEvidence::Supabase)
        );
        assert_eq!(
            candidate.source_path,
            PathBuf::from("/projects/app/supabase/config.toml")
        );
    }

    #[test]
    fn creates_node_component_candidate() {
        let candidate = Candidate {
            name: "gateway".to_string(),
            path: PathBuf::from("/projects/app/gateway"),
            classification: Classification::Component(ComponentEvidence::NodeJs),
            source_path: PathBuf::from("/projects/app/gateway/package.json"),
        };

        assert_eq!(candidate.name, "gateway");
        assert_eq!(candidate.path, PathBuf::from("/projects/app/gateway"));
        assert_eq!(
            candidate.classification,
            Classification::Component(ComponentEvidence::NodeJs)
        );
        assert_eq!(
            candidate.source_path,
            PathBuf::from("/projects/app/gateway/package.json")
        );
    }

    #[test]
    fn creates_dist_excluded_candidate() {
        let candidate = Candidate {
            name: "dist".to_string(),
            path: PathBuf::from("/projects/app/dist"),
            classification: Classification::Excluded(ExcludedEvidence::Dist),
            source_path: PathBuf::from("/projects/app/dist"),
        };

        assert_eq!(candidate.name, "dist");
        assert_eq!(candidate.path, PathBuf::from("/projects/app/dist"));
        assert_eq!(
            candidate.classification,
            Classification::Excluded(ExcludedEvidence::Dist)
        );
        assert_eq!(candidate.source_path, PathBuf::from("/projects/app/dist"));
    }
}
