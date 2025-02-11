use git2::{Config, Cred, RemoteCallbacks, FetchOptions};

pub fn configure_callbacks<'a>() -> RemoteCallbacks<'a> {
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
    callbacks
}


pub fn pull_changes<'a>(
    repo: &'a git2::Repository,
    remote_name: &str,
    remote_callbacks: RemoteCallbacks,
) -> Result<git2::Reference<'a>, git2::Error> {
    let mut remote = repo.find_remote(remote_name)?;
    let branch_ref_specs = ["refs/heads/*:refs/remotes/origin/*"];
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);
    fetch_options.prune(git2::FetchPrune::On);

    // Fetch
    remote.fetch(&branch_ref_specs, Some(&mut fetch_options), None)?;

    // Merge fetched changes
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(fetch_head)
}
