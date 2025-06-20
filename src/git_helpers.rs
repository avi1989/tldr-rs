use git2::Repository;
use std::path::Path;

pub fn clone_repository(
    repo_url: &str,
    repo_name: &str,
    tldr_path: &Path,
) -> Result<(), &'static str> {
    let repo_path = tldr_path.join(repo_name);
    if repo_path.exists() {
        return Err("Path already exists");
    }
    Repository::clone(repo_url, repo_path).unwrap();
    Ok(())
}
