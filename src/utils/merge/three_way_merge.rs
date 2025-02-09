use super::{
    conflict::ConflictHandler,
    result::MergeResult,
};

pub fn three_way_merge(repo: &git2::Repository, commit: &git2::Commit) -> MergeResult<()> {
    // Normal merge
    let mut index = repo.merge_commits(&repo.head()?.peel_to_commit()?, commit, None)?;
    if index.has_conflicts() {
        ConflictHandler::new(repo, index).write_all_markers()?;
        return Ok(());
    }
    let result_tree = repo.find_tree(index.write_tree_to(&repo)?)?;
    let signature = repo.signature()?;
    let parent_commit = repo.head()?.peel_to_commit()?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Merge commit",
        &result_tree,
        &[&parent_commit, &commit],
    )?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    Ok(())
}
