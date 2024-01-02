use std::fs;
use std::path::Path;
use std::{collections::HashMap, u32};

use git2::{Blame, BlameOptions, Oid, Repository};

pub struct GitBlameInfo {
    pub(crate) name: String,
    pub(crate) commit_id: Oid,
    pub(crate) commit_date: i64,
}

pub type LineGitBlame = HashMap<u32, GitBlameInfo>;

pub fn get_git_blame(filename: &Path) -> Option<LineGitBlame> {
    let repo = Repository::discover(filename).ok()?;
    let repo_path_absolute = fs::canonicalize(repo.workdir()?).ok()?;
    let filepath_absolute = fs::canonicalize(filename).ok()?;
    let filepath_relative_to_repo = filepath_absolute.strip_prefix(&repo_path_absolute).ok()?;

    let file_git_blame: Blame = repo
        .blame_file(filepath_relative_to_repo, Some(&mut BlameOptions::new()))
        .ok()?;

    let mut line_git_blames: LineGitBlame = HashMap::new();

    for git_hunk in file_git_blame.iter() {
        if let Some(name) = git_hunk.final_signature().name() {
            let start_line = git_hunk.final_start_line() as u32;
            let commit_id = git_hunk.final_commit_id();
            let commit_date = git_hunk.final_signature().when().seconds();

            for line_number in start_line..start_line + git_hunk.lines_in_hunk() as u32 {
                line_git_blames.insert(
                    line_number,
                    GitBlameInfo {
                        name: name.to_string().clone(),
                        commit_id,
                        commit_date,
                    },
                );
            }
        }
    }

    Some(line_git_blames)
}
