use git2::{AnnotatedCommit, Reference, Repository};

use crate::stack::Stack;

use super::{
    conflict::ConflictHandler,
    fast_forward::fast_forward_merge,
    result::{MergeError, MergeErrorKind, MergeResult},
};

pub struct MergeHandler<'a> {
    repo: &'a Repository,
    fetch_head: Option<&'a Reference<'a>>,
}

impl<'a> MergeHandler<'a> {
    pub fn new(repo: &'a Repository, fetch_head: Option<&'a Reference>) -> Self {
        return MergeHandler {
            repo,
            fetch_head: fetch_head,
        };
    }

    pub fn try_merge(self) -> MergeResult<()> {
        match self.fetch_head {
            Some(fetch_head) => {
                let fetch_commit = self.repo.reference_to_annotated_commit(fetch_head)?;
                let (analysis, _) = self.repo.merge_analysis(&[&fetch_commit])?;
                let stack = Stack::new(&self.repo);

                let trunk_branch = stack.find_trunk()?;
                let trunk_branch_ref = format!("refs/heads/{}", trunk_branch);
                if analysis.is_fast_forward() {
                    // Fast-forward merge
                    fast_forward_merge(&self.repo, trunk_branch_ref.as_str(), &fetch_commit)?;
                    return Ok(());
                } else if analysis.is_normal() {
                    self.rebase(Some(fetch_commit)).map_err(|err| {
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
            None => Err(MergeError::new(
                MergeErrorKind::Unknown,
                "No fetch head provided for merge".to_string(),
            )),
        }
    }

    pub fn continue_and_resolve_rebase(self) -> MergeResult<()> {
        self.rebase(None)
    }

    fn rebase(self, maybe_fetch_commit: Option<AnnotatedCommit>) -> MergeResult<()> {
        // Normal merge
        let mut rebase = match maybe_fetch_commit {
            Some(fetch_commit) => self.repo.rebase(None, Some(&fetch_commit), None, None),
            None => self.repo.open_rebase(None),
        }?;
        while let Some(_) = rebase.next() {
            if let Ok(index) = rebase.inmemory_index() {
                if index.has_conflicts() {
                    ConflictHandler::new(self.repo, index).write_all_markers()?;
                    return Ok(());
                }
            }
        }
        rebase.commit(None, &self.repo.signature()?, None)?;
        rebase.finish(None)?;
        Ok(())
    }
}
