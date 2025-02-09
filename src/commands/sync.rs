use crate::utils::merge::handler::MergeHandler;
use crate::utils::pull::pull_changes;
use clap::Args;
use git2::{Config, Cred, RemoteCallbacks};

use super::{checkout::open_repo, result::CmdResult};

#[derive(Args, Debug)]
pub struct SyncCommandArgs {}

pub fn sync(_args: &SyncCommandArgs) -> CmdResult<()> {
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
            if let (Ok(username_cfg_entry), Ok(password_cfg_entry)) =
                (config.get_entry("username"), config.get_entry("password"))
            {
                if let (Some(username), Some(password)) =
                    (username_cfg_entry.value(), password_cfg_entry.value())
                {
                    let cred = Cred::userpass_plaintext(username, password)?;
                    return Ok(cred);
                }
            }
        }

        Err(git2::Error::from_str("No valid credentials found"))
    });
    callbacks.transfer_progress(|stats| {
        println!(
            "Received {}/{} objects",
            stats.received_objects(),
            stats.total_objects()
        );
        true
    });

    // Pull changes
    let fetch_head = pull_changes(&repo, "origin", callbacks)?;

    // Merge changes
    // TODO: We need to fix this. Currently it doesn't stash the locally made commit(s), then apply them on master.
    // It instead tries the opposite way round (merge remote onto local).
    let merge_handler = MergeHandler::new(&repo, Some(&fetch_head));

    merge_handler.try_merge()?;
    println!("Sync successful.");
    Ok(())
}
