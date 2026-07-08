use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::candidate::{
    BuildEvidence, Candidate, Classification, ComponentEvidence, OperationalEvidence,
};
use crate::marker::{
    BUILD_MARKERS, COMPONENT_MARKERS, OPERATIONAL_MARKERS, ROOT_MARKERS, find_build_candidates,
    find_component_candidates, find_operational_candidates, find_root_candidates,
};

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> io::Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        let path =
            std::env::temp_dir().join(format!("uridim-{name}-{}-{timestamp}", std::process::id()));

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

fn candidate_with_source<'a>(
    candidates: &'a [Candidate],
    source_path: &Path,
) -> Option<&'a Candidate> {
    candidates
        .iter()
        .find(|candidate| candidate.source_path == source_path)
}

// ----- Root markers -----

#[test]
fn returns_empty_when_no_root_markers_are_configured() -> io::Result<()> {
    let test_dir = TestDir::new("no-root-markers")?;
    let nested = test_dir.path().join("project").join("src").join("nested");

    fs::create_dir_all(&nested)?;

    let candidates = find_root_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}

// ----- Component markers -----

#[test]
fn detects_cargo_component_without_git() -> io::Result<()> {
    let test_dir = TestDir::new("cargo-without-git")?;
    let project = test_dir.path().join("project");
    let nested = project.join("src").join("nested");
    let cargo_manifest = project.join("Cargo.toml");

    fs::create_dir_all(&nested)?;
    fs::write(&cargo_manifest, b"[package]\nname = \"example\"\n")?;

    let candidates = find_component_candidates(&nested)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.name, "project");
    assert_eq!(candidate.path, project);
    assert_eq!(
        candidate.classification,
        Classification::Component(ComponentEvidence::Cargo)
    );
    assert_eq!(candidate.source_path, cargo_manifest);

    Ok(())
}

#[test]
fn detects_each_supported_component_marker() -> io::Result<()> {
    let cases = [
        ("Cargo.toml", ComponentEvidence::Cargo),
        ("package.json", ComponentEvidence::NodeJs),
        ("pyproject.toml", ComponentEvidence::Python),
        ("go.mod", ComponentEvidence::Go),
        ("pom.xml", ComponentEvidence::Maven),
    ];

    for (index, (relative_path, expected_evidence)) in cases.into_iter().enumerate() {
        let test_dir = TestDir::new(&format!("component-marker-{index}"))?;

        let project = test_dir.path().join("project");
        let nested = project.join("src").join("nested");
        let source_path = project.join(relative_path);

        fs::create_dir_all(&nested)?;
        fs::write(&source_path, b"test")?;

        let candidates = find_component_candidates(&nested)?;

        assert_eq!(
            candidates.len(),
            1,
            "expected one candidate for {relative_path}"
        );

        let candidate = &candidates[0];

        assert_eq!(candidate.path, project);
        assert_eq!(
            candidate.classification,
            Classification::Component(expected_evidence),
            "wrong evidence for {relative_path}"
        );
        assert_eq!(
            candidate.source_path, source_path,
            "wrong source path for {relative_path}"
        );
    }

    Ok(())
}

#[test]
fn finds_component_candidates_at_multiple_ancestors() -> io::Result<()> {
    let test_dir = TestDir::new("nested-components")?;

    let workspace = test_dir.path().join("workspace");
    let member = workspace.join("member");
    let nested = member.join("src");

    let workspace_manifest = workspace.join("Cargo.toml");
    let member_manifest = member.join("Cargo.toml");

    fs::create_dir_all(&nested)?;

    fs::write(
        &workspace_manifest,
        b"[workspace]\nmembers = [\"member\"]\n",
    )?;

    fs::write(&member_manifest, b"[package]\nname = \"member\"\n")?;

    let candidates = find_component_candidates(&nested)?;

    assert_eq!(candidates.len(), 2);

    let member_candidate = candidate_with_source(&candidates, &member_manifest)
        .expect("member marker should produce candidate");

    assert_eq!(member_candidate.path, member);
    assert_eq!(
        member_candidate.classification,
        Classification::Component(ComponentEvidence::Cargo)
    );

    let workspace_candidate = candidate_with_source(&candidates, &workspace_manifest)
        .expect("workspace marker should produce candidate");

    assert_eq!(workspace_candidate.path, workspace);
    assert_eq!(
        workspace_candidate.classification,
        Classification::Component(ComponentEvidence::Cargo)
    );

    Ok(())
}

#[test]
fn finds_multiple_component_markers_in_same_directory() -> io::Result<()> {
    let test_dir = TestDir::new("multiple-components")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");

    let cargo_manifest = project.join("Cargo.toml");
    let package_manifest = project.join("package.json");

    fs::create_dir_all(&nested)?;
    fs::write(&cargo_manifest, b"[package]\nname = \"example\"\n")?;
    fs::write(&package_manifest, b"{}")?;

    let candidates = find_component_candidates(&nested)?;

    assert_eq!(candidates.len(), 2);

    let cargo_candidate = candidate_with_source(&candidates, &cargo_manifest)
        .expect("Cargo marker should produce candidate");

    assert_eq!(cargo_candidate.path, project);
    assert_eq!(
        cargo_candidate.classification,
        Classification::Component(ComponentEvidence::Cargo)
    );

    let node_candidate = candidate_with_source(&candidates, &package_manifest)
        .expect("Node.js marker should produce candidate");

    assert_eq!(node_candidate.path, project);
    assert_eq!(
        node_candidate.classification,
        Classification::Component(ComponentEvidence::NodeJs)
    );

    Ok(())
}

#[test]
fn ignores_similarly_named_component_files() -> io::Result<()> {
    let test_dir = TestDir::new("similar-component-names")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");

    fs::create_dir_all(&nested)?;

    fs::write(project.join("Cargo.toml.backup"), b"test")?;
    fs::write(project.join("package.json.old"), b"test")?;
    fs::write(project.join("pyproject.toml.tmp"), b"test")?;
    fs::write(project.join("go.mod.backup"), b"test")?;
    fs::write(project.join("pom.xml.old"), b"test")?;

    let candidates = find_component_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn ignores_directory_named_like_component_marker() -> io::Result<()> {
    let test_dir = TestDir::new("component-directory")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");

    fs::create_dir_all(&nested)?;
    fs::create_dir(project.join("Cargo.toml"))?;

    let candidates = find_component_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn returns_empty_when_no_component_marker_exists() -> io::Result<()> {
    let test_dir = TestDir::new("no-component")?;

    let nested = test_dir.path().join("project").join("src").join("nested");

    fs::create_dir_all(&nested)?;

    let candidates = find_component_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}

// ----- Build markers -----

#[test]
fn detects_cmake_build_marker() -> io::Result<()> {
    let test_dir = TestDir::new("cmake-build")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");
    let cmake_marker = project.join("CMakeLists.txt");

    fs::create_dir_all(&nested)?;
    fs::write(&cmake_marker, b"cmake_minimum_required(VERSION 3.20)\n")?;

    let candidates = find_build_candidates(&nested)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.name, "project");
    assert_eq!(candidate.path, project);
    assert_eq!(
        candidate.classification,
        Classification::Build(BuildEvidence::CMake)
    );
    assert_eq!(candidate.source_path, cmake_marker);

    Ok(())
}

#[test]
fn finds_build_candidates_at_multiple_ancestors() -> io::Result<()> {
    let test_dir = TestDir::new("nested-builds")?;

    let outer = test_dir.path().join("outer");
    let inner = outer.join("inner");
    let nested = inner.join("src");

    let outer_marker = outer.join("CMakeLists.txt");
    let inner_marker = inner.join("CMakeLists.txt");

    fs::create_dir_all(&nested)?;
    fs::write(&outer_marker, b"project(outer)\n")?;
    fs::write(&inner_marker, b"project(inner)\n")?;

    let candidates = find_build_candidates(&nested)?;

    assert_eq!(candidates.len(), 2);

    let inner_candidate = candidate_with_source(&candidates, &inner_marker)
        .expect("inner CMake marker should produce candidate");

    assert_eq!(inner_candidate.path, inner);

    let outer_candidate = candidate_with_source(&candidates, &outer_marker)
        .expect("outer CMake marker should produce candidate");

    assert_eq!(outer_candidate.path, outer);

    Ok(())
}

#[test]
fn ignores_directory_named_like_build_marker() -> io::Result<()> {
    let test_dir = TestDir::new("cmake-directory")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");

    fs::create_dir_all(&nested)?;
    fs::create_dir(project.join("CMakeLists.txt"))?;

    let candidates = find_build_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}

// ----- Operational markers -----

#[test]
fn detects_docker_compose_operational_marker() -> io::Result<()> {
    let test_dir = TestDir::new("docker-compose")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");
    let marker = project.join("docker-compose.yml");

    fs::create_dir_all(&nested)?;
    fs::write(&marker, b"services:\n  app:\n    image: example\n")?;

    let candidates = find_operational_candidates(&nested)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.name, "project");
    assert_eq!(candidate.path, project);
    assert_eq!(
        candidate.classification,
        Classification::Operational(OperationalEvidence::DockerCompose)
    );
    assert_eq!(candidate.source_path, marker);

    Ok(())
}

#[test]
fn detects_kubernetes_operational_marker() -> io::Result<()> {
    let test_dir = TestDir::new("kustomization")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");
    let marker = project.join("kustomization.yaml");

    fs::create_dir_all(&nested)?;
    fs::write(&marker, b"resources:\n  - deployment.yaml\n")?;

    let candidates = find_operational_candidates(&nested)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.path, project);
    assert_eq!(
        candidate.classification,
        Classification::Operational(OperationalEvidence::Kubernetes)
    );
    assert_eq!(candidate.source_path, marker);

    Ok(())
}

#[test]
fn detects_nested_supabase_operational_marker() -> io::Result<()> {
    let test_dir = TestDir::new("supabase")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");
    let marker = project.join("supabase").join("config.toml");

    fs::create_dir_all(&nested)?;
    fs::create_dir_all(
        marker
            .parent()
            .expect("Supabase config should have parent directory"),
    )?;
    fs::write(&marker, b"project_id = \"example\"\n")?;

    let candidates = find_operational_candidates(&nested)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.name, "project");
    assert_eq!(candidate.path, project);
    assert_eq!(
        candidate.classification,
        Classification::Operational(OperationalEvidence::Supabase)
    );
    assert_eq!(candidate.source_path, marker);

    Ok(())
}

#[test]
fn ignores_directory_named_like_operational_marker() -> io::Result<()> {
    let test_dir = TestDir::new("operational-directory")?;

    let project = test_dir.path().join("project");
    let nested = project.join("src");

    fs::create_dir_all(&nested)?;
    fs::create_dir(project.join("docker-compose.yml"))?;

    let candidates = find_operational_candidates(&nested)?;

    assert!(candidates.is_empty());

    Ok(())
}

#[test]
fn finds_operational_candidates_at_multiple_ancestors() -> io::Result<()> {
    let test_dir = TestDir::new("nested-operational")?;

    let outer = test_dir.path().join("outer");
    let inner = outer.join("inner");
    let nested = inner.join("src");

    let outer_marker = outer.join("docker-compose.yml");
    let inner_marker = inner.join("kustomization.yaml");

    fs::create_dir_all(&nested)?;
    fs::write(&outer_marker, b"services:\n  app:\n    image: example\n")?;
    fs::write(&inner_marker, b"resources:\n  - deployment.yaml\n")?;

    let candidates = find_operational_candidates(&nested)?;

    assert_eq!(candidates.len(), 2);

    let inner_candidate = candidate_with_source(&candidates, &inner_marker)
        .expect("inner operational marker should match");

    assert_eq!(inner_candidate.path, inner);
    assert_eq!(
        inner_candidate.classification,
        Classification::Operational(OperationalEvidence::Kubernetes)
    );

    let outer_candidate = candidate_with_source(&candidates, &outer_marker)
        .expect("outer operational marker should match");

    assert_eq!(outer_candidate.path, outer);
    assert_eq!(
        outer_candidate.classification,
        Classification::Operational(OperationalEvidence::DockerCompose)
    );

    Ok(())
}

// ----- Marker policy -----

#[test]
fn configured_marker_paths_are_not_empty() {
    assert!(
        ROOT_MARKERS
            .iter()
            .all(|marker| !marker.relative_path.is_empty())
    );

    assert!(
        COMPONENT_MARKERS
            .iter()
            .all(|marker| !marker.relative_path.is_empty())
    );

    assert!(
        BUILD_MARKERS
            .iter()
            .all(|marker| !marker.relative_path.is_empty())
    );

    assert!(
        OPERATIONAL_MARKERS
            .iter()
            .all(|marker| !marker.relative_path.is_empty())
    );
}
