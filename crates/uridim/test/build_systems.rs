use std::fs;
use std::io;

use tempfile::tempdir;

use crate::build_systems::find_build_system_candidates;
use crate::candidate::{BuildSystemEvidence, Evidence};

#[test]
fn detects_cmake_build_system() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("project");
    let cmake_file = project.join("CMakeLists.txt");

    fs::create_dir_all(&project)?;
    fs::write(&cmake_file, b"cmake_minimum_required(VERSION 3.20)\n")?;

    let candidates = find_build_system_candidates(&project)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.scope_path, project);
    assert_eq!(
        candidate.evidence,
        Evidence::BuildSystem(BuildSystemEvidence::CMake)
    );
    assert_eq!(candidate.source_path, cmake_file);

    Ok(())
}

#[test]
fn finds_cmake_in_nested_descendants() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let native = root.join("native");

    fs::create_dir_all(&native)?;
    fs::write(native.join("CMakeLists.txt"), b"project(native)\n")?;

    let candidates = find_build_system_candidates(&root)?;

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].scope_path, native);

    Ok(())
}

#[test]
fn ignores_directory_named_like_cmake_file() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");

    fs::create_dir_all(root.join("CMakeLists.txt"))?;

    let candidates = find_build_system_candidates(&root)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn prunes_build_systems_in_excluded_directories() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let excluded = root.join("target").join("generated");

    fs::create_dir_all(&excluded)?;
    fs::write(excluded.join("CMakeLists.txt"), b"project(hidden)\n")?;

    let candidates = find_build_system_candidates(&root)?;

    assert!(candidates.is_empty());

    Ok(())
}
