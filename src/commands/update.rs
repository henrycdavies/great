use super::{error::CommandErrorKind, result::CmdResult, CommandError};
use crate::commands::checkout::open_repo;
use clap::Args;
use git2::{Commit, Repository};

#[derive(Args, Debug)]
pub struct UpdateArgs {
    #[arg(short, long)]
    pub commit: bool,
    pub message: Option<String>,
}

pub fn add_all(repo: &git2::Repository) -> CmdResult<()> {
    let mut index = repo
        .index()
        .map_err(|_| CommandError::new(CommandErrorKind::GitError, "Failed to get index".to_string()))?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    match index.write() {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(CommandError::new(
                CommandErrorKind::GitError,
                "Failed to write index".to_string(),
            ))
        }
    }
}

pub fn update_with_new_commit(args: &UpdateArgs) -> CmdResult<()> {
    let repo = open_repo()?;
    add_all(&repo)?;

    let sig = repo.signature()?;
    let tree_id = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent_commit = find_last_commit(&repo)?;
    let parents = &[&parent_commit];

    let message = match args.message {
        Some(ref message) => message.as_str(),
        None => "Default commit message",
    };

    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, parents)?;

    Ok(())
}

pub fn update_with_amend(args: &UpdateArgs) -> CmdResult<()> {
    let repo = open_repo()?;
    add_all(&repo)?;

    let sig = repo.signature()?;
    let tree_id = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent_commit = find_last_commit(&repo)?;

    let message = match args.message {
        Some(ref message) => message.as_str(),
        None => "Default commit message",
    };

    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent_commit])?;

    Ok(())
}

pub fn update(args: &UpdateArgs) -> CmdResult<()> {
    if args.commit {
        return update_with_new_commit(args);
    }
    update_with_amend(args)
}

fn find_last_commit(repo: &Repository) -> CmdResult<Commit> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    let commit = obj
        .into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find last commit"))?;
    Ok(commit)
}
