use git2::{Commit, Repository};

use super::RepoResult;

pub fn new_commit(repo: Repository, message: Option<String>) -> RepoResult<()> {
    let sig = repo.signature()?;
    let tree_id = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent_commit = find_last_commit(&repo)?;
    let parents = &[&parent_commit];

    let message = match message {
        Some(ref message) => message.as_str(),
        None => "Default commit message",
    };

    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, parents)?;
    Ok(())
}

fn find_last_commit(repo: &Repository) -> RepoResult<Commit> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    let commit = obj
        .into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find last commit"))?;
    Ok(commit)
}
