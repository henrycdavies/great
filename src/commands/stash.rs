use super::result::CmdResult;
use crate::error::{Error, ErrorKind};
use git2::{Oid, Repository, StashFlags};

pub fn stash(repo: &mut Repository, message: &str) -> CmdResult<Oid> {
    let sig = repo.signature()?;
    let oid = repo
        .stash_save(&sig, message, Some(StashFlags::DEFAULT))
        .map_err(|_| Error::new(ErrorKind::GitError, "Failed to stash changes.".to_string()))?;
    Ok(oid)
}

pub fn pop_stash(repo: &mut Repository, oid: Oid) -> CmdResult<()> {
    let mut stash_index: Option<usize> = None;
    repo.stash_foreach(|idx, _, _oid| {
        if *_oid == oid {
            stash_index = Some(idx);
            false
        } else {
            true
        }
    })
    .map_err(|_| Error::new(ErrorKind::GitError, "Failed to find stash.".to_string()))?;
    if let Some(stash_index) = stash_index {
        repo.stash_pop(stash_index, None)
            .map_err(|_| Error::new(ErrorKind::GitError, "Failed to pop stash.".to_string()))?;
    }
    Ok(())
}
