#[derive(Debug, Clone, Copy)]
pub(crate) struct ExclusionSpec {
    pub(crate) directory_name: &'static str,
}

pub(crate) const DEFAULT_EXCLUSIONS: &[ExclusionSpec] = &[
    ExclusionSpec {
        directory_name: "target",
    },
    ExclusionSpec {
        directory_name: "node_modules",
    },
    ExclusionSpec {
        directory_name: ".next",
    },
];

pub(crate) fn is_excluded(directory_name: &std::ffi::OsStr, exclusions: &[ExclusionSpec]) -> bool {
    exclusions
        .iter()
        .any(|spec| directory_name == spec.directory_name)
}
