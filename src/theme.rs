use error::{Error, Result};
use std::{io, fs};
use std::process::Command;
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

    let status = Command::new("git")
        .arg("clone")
        .arg(&path)
        .arg("theme")
        .status()?;

    if !status.success() {
        return Err(Error::ExternalProcess);
    }

    Ok(())
}

fn github_like(repo: &str) -> bool {
    // Has only one slash.
    repo.chars().filter(|&c| c == '/').count() == 1 &&
    // That slash neither the first nor the last character.
    repo.chars().next() != Some('/') &&
    repo.chars().last() != Some('/')
}

pub fn execute(repo: &str) {
    match theme(repo) {
        Ok(()) => info!("Theme changed!"),
        Err(e) => error!("Theme change failed: {}.", e),
    }
}
