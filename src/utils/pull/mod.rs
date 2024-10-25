use git2::{FetchOptions, RemoteCallbacks};

pub fn pull_changes<'a>(repo: &'a git2::Repository, remote_name: &str, remote_callbacks: RemoteCallbacks) -> Result<git2::Reference<'a>, git2::Error>{
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
