use chrono::Utc;
use std::{env, fs, path};
use error::{Result, ResultExt};
use std::fs::OpenOptions;
use std::process::Command;

pub fn post(title: &str, edit: bool) -> Result<()> {
    //TODO: Find the directory through a configuration file or something.
    let mut path = env::current_dir()
        .chain_err(|| "couldn't get current directory")?;

    path.push("posts");

    // Ensure that the title is valid.

    if title.chars().any(path::is_separator) {
        return Err("the title contains a path separator".into());
    }

    // Ensure that the posts directory exists.
    
    if !path.is_dir() {
        fs::create_dir(&path)
            .chain_err(|| "couldn't create posts directory")?;
    }
    
    // Get the current date.

    let date = Utc::today().format("%Y-%m-%d");

    // Compute the filename.

    path.push(format!("{} {}.md", date, title));

    // Create the file.

    if path.exists() {
        return Err("post already exists".into());
    }

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .chain_err(|| "couldn't create post")?;

    info!("Created post at '{}'.", path.display());

    // Open in EDITOR if the user asked us to.

    if edit {
        if let Some(ed) = env::var_os("EDITOR") {
            match Command::new(ed).arg(path).spawn() {
                Ok(mut child) => {
                    let _ = child.wait();
                },
                Err(e) => {
                    warn!("The EDITOR could not be started: {}.", e);
                }
            }
        } else {
            warn!("Could not find EDITOR environment variable.");
        }
    }

    Ok(())
}
