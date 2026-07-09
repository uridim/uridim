use std::io;

use uridim::build_systems::find_build_system_candidates;
use uridim::ecosystems::find_ecosystem_candidates;
use uridim::frameworks::find_framework_candidates;
use uridim::infrastructure::find_infrastructure_candidates;
use uridim::project_context::build_project_context;
use uridim::vcs::find_vcs_candidates;

fn main() -> io::Result<()> {
    let start = std::env::current_dir()?;

    let vcs_candidates = find_vcs_candidates(&start)?;

    let root = vcs_candidates
        .first()
        .map(|candidate| candidate.scope_path.clone())
        .unwrap_or_else(|| start.clone());

    let ecosystem_candidates = find_ecosystem_candidates(&root)?;
    let framework_candidates = find_framework_candidates(&root)?;
    let build_system_candidates = find_build_system_candidates(&root)?;
    let infrastructure_candidates = find_infrastructure_candidates(&root)?;

    let mut candidates = Vec::new();
    candidates.extend(vcs_candidates);
    candidates.extend(ecosystem_candidates);
    candidates.extend(framework_candidates);
    candidates.extend(build_system_candidates);
    candidates.extend(infrastructure_candidates);

    let context = build_project_context(root, candidates);

    println!("{context:#?}");

    Ok(())
}
