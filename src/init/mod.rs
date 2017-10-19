use error::{Result, ResultExt};
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn init() -> Result<()> {
    // Ensure the current directory is empty.

    let mut files = fs::read_dir(".")
        .chain_err(|| "couldn't read current directory")?;

    if files.next().is_some() {
        return Err("the current directory isn't empty".into());
    }

    // Create directories.

    fs::create_dir("out")
        .chain_err(|| "couldn't create output directory")?;
    fs::create_dir("posts")
        .chain_err(|| "couldn't create posts directory")?;
    fs::create_dir("templates")
        .chain_err(|| "couldn't create templates directory")?;

    // Create templates.

    let files: &[(&str, &[u8])] = &[
        ("templates/index.hbs", include_bytes!("index.hbs")),
        ("templates/archive.hbs", include_bytes!("archive.hbs")),
        ("templates/post.hbs", include_bytes!("post.hbs")),
    ];

    for &(path, bytes) in files {
        File::create(path)
            .chain_err(|| format!("couldn't create '{}'", path))?
            .write_all(bytes)
            .chain_err(|| format!("couldn't write to '{}'", path))?;
    }

    // Done!

    info!("Your blog is ready!");

    Ok(())
}
