use clap::Args;
use git2::{Config, Cred, RemoteCallbacks};

use super::{checkout::open_repo, result::Result};

#[derive(Args, Debug)]
pub struct SyncCommandArgs {}

pub fn sync(args: &SyncCommandArgs) -> Result<()> {
    let repo = open_repo()?;

    // Define the remote callbacks
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
    
    // Fetch from remote
    let branch_refspecs = ["refs/heads/*:refs/remotes/origin/*"];
    let mut fetch_options = git2::FetchOptions::new();
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&branch_refspecs, Some(&mut fetch_options), None)?;
    return Ok(())
}