use std::collections::HashMap;
use std::fs;
use std::path::Path;

use git2::{Blame, BlameOptions, Repository};

pub type LineGitBlame = HashMap<u32, Option<&'static str>>;

pub fn get_git_blame(filename: &Path) -> Option<LineGitBlame> {
    let repo = Repository::discover(filename).ok()?;
    let repo_path_absolute = fs::canonicalize(repo.workdir()?).ok()?;
    let filepath_absolute = fs::canonicalize(filename).ok()?;
    let filepath_relative_to_repo = filepath_absolute.strip_prefix(&repo_path_absolute).ok()?;

    let file_git_blame: Blame = repo
        .blame_file(filepath_relative_to_repo, Some(&mut BlameOptions::new()))
        .ok()?;

    let mut line_git_blames: LineGitBlame = HashMap::new();
    line_git_blames.insert(1, Option::from("Oliver"));

    // for

    // iter()
    // .enumerate()
    // .map(|(i, hunk)| {
    //     (i as u32 + 1, hunk.final_signature().name().map(String::from))
    // })
    // .collect();

    Some(line_git_blames)
}
