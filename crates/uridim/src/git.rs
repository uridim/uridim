use std::io;
use std::path::Path;

use crate::candidate::{Candidate, Classification, RootEvidence};

pub fn find_git_root(start: &Path) -> io::Result<Option<Candidate>> {
    for directory in start.ancestors() {
        let git_marker = directory.join(".git");

        if git_marker.try_exists()? {
            let name = directory
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| directory.display().to_string());

            return Ok(Some(Candidate {
                name,
                path: directory.to_path_buf(),
                classification: Classification::Root(RootEvidence::Git),
            }));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(name: &str) -> std::path::PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!("uridim-{name}-{}-{timestamp}", std::process::id()))
    }

    #[test]
    fn finds_git_root_from_nested_directory() -> io::Result<()> {
        let test_root = unique_temp_dir("git-root");
        let project = test_root.join("project");
        let nested = project.join("src").join("nested");

        fs::create_dir_all(&nested)?;
        fs::create_dir(project.join(".git"))?;

        let candidate = find_git_root(&nested)?.expect("Git root should be discovered");

        assert_eq!(candidate.path, project);
        assert_eq!(candidate.name, "project");
        assert_eq!(
            candidate.classification,
            Classification::Root(RootEvidence::Git)
        );

        fs::remove_dir_all(test_root)?;

        Ok(())
    }

    #[test]
    fn returns_none_when_no_git_marker_exists() -> io::Result<()> {
        let test_root = unique_temp_dir("no-git");
        let nested = test_root.join("a").join("b").join("c");

        fs::create_dir_all(&nested)?;

        let candidate = find_git_root(&nested)?;

        assert_eq!(candidate, None);

        fs::remove_dir_all(test_root)?;

        Ok(())
    }

    #[test]
    fn accepts_git_marker_as_file() -> io::Result<()> {
        let test_root = unique_temp_dir("git-file");
        let project = test_root.join("project");
        let nested = project.join("src");

        fs::create_dir_all(&nested)?;
        fs::write(project.join(".git"), b"gitdir: somewhere")?;

        let candidate = find_git_root(&nested)?.expect("Git marker file should be discovered");

        assert_eq!(candidate.path, project);
        assert_eq!(
            candidate.classification,
            Classification::Root(RootEvidence::Git)
        );

        fs::remove_dir_all(test_root)?;

        Ok(())
    }
}
