use error::Result;
use git2::Repository;
use std::{fs, io};
use util::cd2root;

fn theme(repo: &str) -> Result<()> {
    cd2root()?;

    // Delete `theme` if it exists.

    if let Err(e) = fs::remove_dir_all("theme") {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e.into());
        }
    }

    // Build the path.

    let mut path = String::new();

    if github_like(repo) {
        path.push_str("https://github.com/");
    }

    path.push_str(repo);

    // Clone `path` into `theme`.

    Repository::clone(&path, "theme")?;

    Ok(())
}

fn github_like(repo: &str) -> bool {
    // Has only one slash.
    repo.chars().filter(|&c| c == '/').count() == 1 &&
    // That slash neither the first nor the last character.
    !repo.starts_with('/') && !repo.ends_with('/')
}

pub fn execute(repo: &str) {
    match theme(repo) {
        Ok(()) => info!("Theme changed!"),
        Err(e) => error!("Theme change failed: {}.", e),
    }
}
