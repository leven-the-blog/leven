use chrono::Utc;
use std::{env, fs};
use error::{Error, Result};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process::Command;

pub fn post(title: &str, edit: bool) -> Result {
    let mut path = env::current_dir()?;
    path.push("posts");

    // Ensure that the posts directory exists.
    
    if !path.is_dir() {
        fs::create_dir(&path)?;
    }
    
    // Get the current date.

    let date = Utc::today().format("%Y-%m-%d");

    // Compute the filename.

    path.push(format!("{} {}.md", date, title));

    // Create the file.

    if path.exists() {
        return Err(Error::Other("You already made that post today."));
    }

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;

    info!("Created post at `{}`.", path.display());

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
