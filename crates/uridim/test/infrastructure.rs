use std::fs;
use std::io;

use tempfile::tempdir;

use crate::candidate::{Evidence, InfrastructureEvidence};
use crate::infrastructure::find_infrastructure_candidates;

#[test]
fn detects_docker_compose() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("project");
    let compose_file = project.join("compose.yaml");

    fs::create_dir_all(&project)?;
    fs::write(&compose_file, b"services:\n  app:\n    image: example\n")?;

    let candidates = find_infrastructure_candidates(&project)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.scope_path, project);
    assert_eq!(
        candidate.evidence,
        Evidence::Infrastructure(InfrastructureEvidence::DockerCompose)
    );
    assert_eq!(candidate.source_path, compose_file);

    Ok(())
}

#[test]
fn detects_kubernetes_kustomization() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("project");
    let marker = project.join("kustomization.yaml");

    fs::create_dir_all(&project)?;
    fs::write(&marker, b"resources:\n  - deployment.yaml\n")?;

    let candidates = find_infrastructure_candidates(&project)?;

    assert_eq!(candidates.len(), 1);
    assert_eq!(
        candidates[0].evidence,
        Evidence::Infrastructure(InfrastructureEvidence::Kubernetes)
    );
    assert_eq!(candidates[0].source_path, marker);

    Ok(())
}

#[test]
fn detects_supabase_config() -> io::Result<()> {
    let temp = tempdir()?;

    let project = temp.path().join("project");
    let supabase_dir = project.join("supabase");
    let config = supabase_dir.join("config.toml");

    fs::create_dir_all(&supabase_dir)?;
    fs::write(&config, b"project_id = \"example\"\n")?;

    let candidates = find_infrastructure_candidates(&project)?;

    assert_eq!(candidates.len(), 1);

    let candidate = &candidates[0];

    assert_eq!(candidate.scope_path, project);
    assert_eq!(
        candidate.evidence,
        Evidence::Infrastructure(InfrastructureEvidence::Supabase)
    );
    assert_eq!(candidate.source_path, config);

    Ok(())
}

#[test]
fn finds_infrastructure_in_nested_descendants() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let infra = root.join("deploy");

    fs::create_dir_all(&infra)?;
    fs::write(
        infra.join("docker-compose.yml"),
        b"services:\n  app:\n    image: example\n",
    )?;

    let candidates = find_infrastructure_candidates(&root)?;

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].scope_path, infra);

    Ok(())
}

#[test]
fn prunes_infrastructure_in_excluded_directories() -> io::Result<()> {
    let temp = tempdir()?;

    let root = temp.path().join("project");
    let excluded = root.join(".next").join("generated");

    fs::create_dir_all(&excluded)?;
    fs::write(
        excluded.join("compose.yaml"),
        b"services:\n  app:\n    image: example\n",
    )?;

    let candidates = find_infrastructure_candidates(&root)?;

    assert!(candidates.is_empty());

    Ok(())
}
