use crate::core::drawio::mxfile::{read_file, Mxfile};
use anyhow::{Context, Result};
use git2::{DiffOptions, Object, ObjectType, Repository};
use std::path::{Path, PathBuf};

pub fn explore_path(path: &Path, git_reference: &str) -> Result<Vec<(PathBuf, Mxfile)>> {
    let drawio_paths: Vec<PathBuf> = collect_files_from_git(path, git_reference)?;

    let mut files: Vec<(PathBuf, Mxfile)> = vec![];
    for drawio_path in drawio_paths {
        files.push((drawio_path.clone(), read_file(&drawio_path)?))
    }

    files.sort_by(|(a, _), (b, _)| a.cmp(b));

    Ok(files)
}

fn collect_files_from_git(root_path: &Path, git_reference: &str) -> Result<Vec<PathBuf>> {
    let repo = Repository::discover(root_path)
        .with_context(|| format!("need to be a git repository {}", &root_path.display()))?;
    let mut opts = DiffOptions::new();
    let old_tree = reference_as_tree(&repo, git_reference).with_context(|| {
        format!(
            "can't found reference {} on {}",
            git_reference,
            repo.path().display()
        )
    })?;
    let diff_output = repo
        .diff_tree_to_index(old_tree.as_tree(), None, Some(&mut opts))
        .with_context(|| {
            format!(
                "can't found modified files from {} under {}",
                repo.path().display(),
                &root_path.display()
            )
        })?;

    let diff_files = diff_output
        .deltas()
        .map(|delta| PathBuf::from(delta.new_file().path().unwrap()))
        .filter(|path| match path.extension() {
            Some(ext) => ext.eq("drawio"),
            None => false,
        })
        .filter(|path| {
            path.canonicalize()
                .unwrap()
                .starts_with(root_path.canonicalize().unwrap())
        })
        .collect::<Vec<PathBuf>>();

    Ok(diff_files)
}

fn reference_as_tree<'a>(repo: &'a Repository, git_reference: &'a str) -> Result<Object<'a>> {
    let object = repo.revparse_single(git_reference)?;
    let tree_object = object.peel(ObjectType::Tree)?;
    Ok(tree_object)
}
