use std::fs;
use std::io;
use std::path::Path;

use crate::candidate::{Candidate, Evidence};
use crate::exclusion::{ExclusionSpec, is_excluded};

#[derive(Debug, Clone, Copy)]
pub(crate) enum EntryKind {
    File,
    Directory,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PathSpec<E> {
    pub(crate) relative_path: &'static str,
    pub(crate) kind: EntryKind,
    pub(crate) evidence: E,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ContentSpec<E> {
    pub(crate) relative_path: &'static str,
    pub(crate) evidence: E,
    pub(crate) matches: fn(&str) -> bool,
}

fn matches_entry_kind(metadata: &fs::Metadata, kind: EntryKind) -> bool {
    match kind {
        EntryKind::File => metadata.is_file(),
        EntryKind::Directory => metadata.is_dir(),
    }
}

pub(crate) fn find_path_in_ancestors<E>(
    start: &Path,
    specs: &[PathSpec<E>],
    classify: impl Fn(E) -> Evidence,
) -> io::Result<Vec<Candidate>>
where
    E: Copy,
{
    let mut candidates = Vec::new();

    for directory in start.ancestors() {
        for spec in specs {
            let source_path = directory.join(spec.relative_path);

            match fs::metadata(&source_path) {
                Ok(metadata) if matches_entry_kind(&metadata, spec.kind) => {
                    candidates.push(Candidate {
                        name: directory
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_else(|| directory.display().to_string()),
                        scope_path: directory.to_path_buf(),
                        evidence: classify(spec.evidence),
                        source_path,
                    });
                }

                Ok(_) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
    }

    Ok(candidates)
}

pub(crate) fn find_path_in_descendants<E>(
    root: &Path,
    specs: &[PathSpec<E>],
    exclusions: &[ExclusionSpec],
    classify: impl Fn(E) -> Evidence,
) -> io::Result<Vec<Candidate>>
where
    E: Copy,
{
    let mut candidates = Vec::new();
    let mut pending = vec![root.to_path_buf()];

    while let Some(scope_dir) = pending.pop() {
        for spec in specs {
            let source_path = scope_dir.join(spec.relative_path);

            match fs::metadata(&source_path) {
                Ok(metadata) if matches_entry_kind(&metadata, spec.kind) => {
                    candidates.push(Candidate {
                        name: scope_dir
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_else(|| scope_dir.display().to_string()),
                        scope_path: scope_dir.clone(),
                        evidence: classify(spec.evidence),
                        source_path,
                    });
                }
                Ok(_) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }

        for entry in fs::read_dir(&scope_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if !file_type.is_dir() {
                continue;
            }

            let name = entry.file_name();

            if is_excluded(&name, exclusions) {
                continue;
            }

            pending.push(entry.path());
        }
    }

    Ok(candidates)
}

pub(crate) fn find_by_content_in_directory<E>(
    directory: &Path,
    specs: &[ContentSpec<E>],
    classify: impl Fn(E) -> Evidence,
) -> io::Result<Vec<Candidate>>
where
    E: Copy,
{
    let mut candidates = Vec::new();

    for spec in specs {
        let source_path = directory.join(spec.relative_path);

        let contents = match fs::read_to_string(&source_path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };

        if (spec.matches)(&contents) {
            candidates.push(Candidate {
                name: directory
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| directory.display().to_string()),
                scope_path: directory.to_path_buf(),
                evidence: classify(spec.evidence),
                source_path,
            });
        }
    }

    Ok(candidates)
}
