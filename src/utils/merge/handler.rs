use git2::{Reference, Repository};

use crate::utils::stack::Stack;

use super::{fast_forward::fast_forward_merge, result::{MergeError, MergeErrorKind, MergeResult}, three_way_merge::three_way_merge};

pub struct MergeHandler<'a> {
    repo: &'a Repository,
    fetch_head: &'a Reference<'a>
}

impl<'a> MergeHandler<'a> {
    pub fn new(repo: &'a Repository, fetch_head: &'a Reference) -> Self {
        return MergeHandler { repo, fetch_head }
    }

    pub fn try_merge(self) -> MergeResult<()> {
        let fetch_commit = self.repo.reference_to_annotated_commit(self.fetch_head)?;
        let (analysis, _) = self.repo.merge_analysis(&[&fetch_commit])?;

        let stack = Stack::new(self.repo);

        let trunk_branch = stack.find_trunk()?;
        let trunk_branch_ref = format!("refs/heads/{}", trunk_branch);
        if analysis.is_fast_forward() {
            // Fast-forward merge
            fast_forward_merge(&self.repo, trunk_branch_ref.as_str(), &fetch_commit)?;
            return Ok(());
        } else if analysis.is_normal() {
            let commit = self.repo.find_commit(fetch_commit.id())?;
            three_way_merge(&self.repo, &commit).map_err(|err| {
                MergeError::new(
                    MergeErrorKind::Unknown,
                    format!("Failed to merge changes: {}", err.message()),
                )
            })?;
            return Ok(());
        } else if analysis.is_up_to_date() {
            println!("Trunk is up-to-date.");
            return Ok(());
        }
        let error = MergeError::new(MergeErrorKind::Unknown, "Unknown error".to_string());
        Err(error)
    }
}
