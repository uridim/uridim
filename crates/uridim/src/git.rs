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
                source_path: git_marker,
            }));
        }
    }

    Ok(None)
}

// ===== TEST ===== //
#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(name: &str) -> io::Result<Self> {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after Unix epoch")
                .as_nanos();

            let path = std::env::temp_dir()
                .join(format!("uridim-{name}-{}-{timestamp}", std::process::id()));

            fs::create_dir_all(&path)?;

            Ok(Self { path })
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn finds_git_root_from_nested_directory() -> io::Result<()> {
        let test_dir = TestDir::new("git-root")?;
        let project = test_dir.path().join("project");
        let nested = project.join("src").join("nested");
        let git_marker = project.join(".git");

        fs::create_dir_all(&nested)?;
        fs::create_dir(&git_marker)?;

        let candidate = find_git_root(&nested)?.expect("Git root should be discovered");

        assert_eq!(candidate.name, "project");
        assert_eq!(candidate.path, project);
        assert_eq!(
            candidate.classification,
            Classification::Root(RootEvidence::Git)
        );
        assert_eq!(candidate.source_path, git_marker);

        Ok(())
    }

    #[test]
    fn returns_none_when_no_git_marker_exists() -> io::Result<()> {
        let test_dir = TestDir::new("no-git")?;
        let nested = test_dir.path().join("a").join("b").join("c");

        fs::create_dir_all(&nested)?;

        let candidate = find_git_root(&nested)?;

        assert_eq!(candidate, None);

        Ok(())
    }

    #[test]
    fn accepts_git_marker_as_file() -> io::Result<()> {
        let test_dir = TestDir::new("git-file")?;
        let project = test_dir.path().join("project");
        let nested = project.join("src");
        let git_marker = project.join(".git");

        fs::create_dir_all(&nested)?;
        fs::write(&git_marker, b"gitdir: somewhere")?;

        let candidate = find_git_root(&nested)?.expect("Git marker file should be discovered");

        assert_eq!(candidate.path, project);
        assert_eq!(
            candidate.classification,
            Classification::Root(RootEvidence::Git)
        );
        assert_eq!(candidate.source_path, git_marker);

        Ok(())
    }

    #[test]
    fn returns_nearest_git_root_when_roots_are_nested() -> io::Result<()> {
        let test_dir = TestDir::new("nested-git-roots")?;

        let outer_project = test_dir.path().join("outer");
        let inner_project = outer_project.join("inner");
        let nested = inner_project.join("src");

        let outer_git_marker = outer_project.join(".git");
        let inner_git_marker = inner_project.join(".git");

        fs::create_dir_all(&nested)?;
        fs::create_dir(&outer_git_marker)?;
        fs::create_dir(&inner_git_marker)?;

        let candidate = find_git_root(&nested)?.expect("nearest Git root should be discovered");

        assert_eq!(candidate.path, inner_project);
        assert_eq!(candidate.source_path, inner_git_marker);

        Ok(())
    }
}
