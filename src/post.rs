use chrono::Utc;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub fn post(title: &str) -> Result<PathBuf, Box<Error>> {
    let mut dir = env::current_dir()?;
    dir.push("posts");

    // Check that the posts directory exists.
    
    if !dir.is_dir() {
        return Err("posts directory not found".into());
    }
    
    // Get the current date.

    let date = Utc::today().format("%Y-%m-%d");

    // Compute the filename.

    dir.push(format!("{} {}.md", date, title));

    // Create the file.

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&dir)?;

    Ok(dir)
}
