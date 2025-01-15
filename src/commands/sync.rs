
use clap::Args;
use git2::build::CheckoutBuilder;
use git2::{Config, Cred, Index, MergeOptions, RemoteCallbacks, Repository};

use crate::utils::{merge::normal::three_way_merge, pull::pull_changes};
use crate::utils::merge::fast_forward::fast_forward_merge;

use super::{checkout::open_repo, result::CmdResult, trunk::find_trunk_branch};

#[derive(Args, Debug)]
pub struct SyncCommandArgs {}

pub fn sync(args: &SyncCommandArgs) -> CmdResult<()> {
    let repo = open_repo()?;

    // Configure auth & progress reporting
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|url, username_from_url, allowed_types| {
        let config = Config::open_default()?;

        // Try git credential manager first
        if let Ok(cred) = Cred::credential_helper(&config, url, username_from_url) {
            return Ok(cred);
        }

        if allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            if let (Ok(username_cfg_entry), Ok(password_cfg_entry)) = (config.get_entry("username"), config.get_entry("password")) {
                if let (Some(username), Some(password)) = (username_cfg_entry.value(), password_cfg_entry.value()) {
                    let cred = Cred::userpass_plaintext(username, password)?;
                    return Ok(cred);
                }
            }
        }

        Err(git2::Error::from_str("No valid credentials found"))
    });
    callbacks.transfer_progress(|stats| {
        println!("Received {}/{} objects", stats.received_objects(), stats.total_objects());
        true
    });

    // Pull changes
    let fetch_head = pull_changes(&repo, "origin", callbacks)?;

    // Merge changes
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;

    let trunk_branch = find_trunk_branch(&repo)?;
    let trunk_branch_ref = format!("refs/heads/{}", trunk_branch);
    if analysis.is_fast_forward() {
        // Fast-forward merge
        fast_forward_merge(&repo, trunk_branch_ref.as_str(), &fetch_commit)?;
        return Ok(());
    } else if analysis.is_normal() {
        let commit = repo.find_commit(fetch_commit.id())?;
        three_way_merge(&repo, &commit)?;
        return Ok(());
    } else if analysis.is_up_to_date() {
        println!("Trunk is up-to-date.");
        return Ok(());
    }
    
    // Check for merge conflicts
    let index = repo.index()?;
    if index.has_conflicts() {
        handle_conflict(&repo, &index)?;
    }

    Ok(())
}

fn handle_conflict(repo: &Repository, index: &Index) -> Result<(), git2::Error> {
    for conflict in index.conflicts()?.into_iter().filter_map(Result::ok) {
        match (conflict.ancestor, conflict.our, conflict.their) {
            (Some(ancestor), Some(our), Some(their)) => {
                
                let ancestor_blob = repo.find_blob(ancestor.id)?;
                let our_blob = repo.find_blob(our.id)?;
                let their_blob = repo.find_blob(their.id)?;

                // Example of writing conflict markers
                let ancestor_content = std::str::from_utf8(ancestor_blob.content())?;
                let our_content = std::str::from_utf8(our_blob.content())?;
                let their_content = std::str::from_utf8(their_blob.content())?;

                let conflict_markers = format!(
                    "<<<<<<< OURS\n{}\n=======\n{}\n>>>>>>> THEIRS\n",
                    our_content, their_content
                );

                // Write the conflict markers to the file
                let path = std::str::from_utf8(our.path)?;
                std::fs::write(path, conflict_markers)?;

                return Ok(())
            }
            _ => {
                return Err(git2::Error::from_str("Conflict missing one or more sides"))
            }
        }
    }
    Ok(())
}
