pub mod build_systems;
pub mod candidate;
pub mod ecosystems;
pub mod frameworks;
pub mod infrastructure;
pub mod project_context;
pub mod vcs;

mod discovery;
mod exclusion;

#[cfg(test)]
#[path = "../test/mod.rs"]
mod tests;
