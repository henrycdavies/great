pub fn fast_forward_merge(
    repo: &git2::Repository,
    branch_name: &str,
    target_commit: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let mut reference = repo.find_reference(branch_name)?;
    reference.set_target(target_commit.id(), "Fast-forward")?;
    repo.set_head(branch_name)?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    Ok(())
}
