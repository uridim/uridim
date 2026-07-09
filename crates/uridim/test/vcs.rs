use std::fs;
use std::io;

use tempfile::tempdir;

use crate::candidate::{Evidence, VcsEvidence};
use crate::vcs::find_vcs_candidates;

#[test]
fn finds_git_directory_in_ancestors() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("project");
    let nested = project.join("src").join("nested");
    let git_dir = project.join(".git");

    fs::create_dir_all(&nested)?;
    fs::create_dir(&git_dir)?;

    let candidates = find_vcs_candidates(&nested)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.name, "project");
    assert_eq!(candidate.scope_path, project);
    assert_eq!(candidate.evidence, Evidence::Vcs(VcsEvidence::Git));
    assert_eq!(candidate.source_path, git_dir);

    Ok(())
}

#[test]
fn returns_empty_when_no_vcs_marker_exists() -> io::Result<()> {
    let temp = tempdir()?;

    let nested = temp.path().join("project").join("src");
    fs::create_dir_all(&nested)?;

    let candidates = find_vcs_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn finds_multiple_git_ancestors() -> io::Result<()> {
    let temp = tempdir()?;

    let outer = temp.path().join("outer");
    let inner = outer.join("inner");
    let nested = inner.join("src");

    let outer_git = outer.join(".git");
    let inner_git = inner.join(".git");

    fs::create_dir_all(&nested)?;
    fs::create_dir(&outer_git)?;
    fs::create_dir(&inner_git)?;

    let candidates = find_vcs_candidates(&nested)?;

    assert_eq!(candidates.len(), 2);

    assert_eq!(candidates[0].scope_path, inner);
    assert_eq!(candidates[0].source_path, inner_git);

    assert_eq!(candidates[1].scope_path, outer);
    assert_eq!(candidates[1].source_path, outer_git);

    Ok(())
}

#[test]
fn ignores_git_file_because_spec_requires_directory() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("project");
    let nested = project.join("src");
    let git_file = project.join(".git");

    fs::create_dir_all(&nested)?;
    fs::write(&git_file, b"gitdir: ../somewhere")?;

    let candidates = find_vcs_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}
