use error::{Error, Result};
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn init() -> Result {
    // Ensure the current directory is empty.

    if fs::read_dir(".")?.next().is_some() {
        return Err(Error::Other("The current directory isn't empty."));
    }

    // Create directories.

    fs::create_dir("out")?;
    fs::create_dir("posts")?;
    fs::create_dir("templates")?;

    // Create templates.

    let files: &[(&str, &[u8])] = &[
        ("templates/index.hbs", include_bytes!("index.hbs")),
        ("templates/archive.hbs", include_bytes!("archive.hbs")),
        ("templates/post.hbs", include_bytes!("post.hbs")),
    ];

    for &(path, bytes) in files {
        File::create(path)?.write_all(bytes)?;
    }

    // Done!

    info!("Your blog is ready!");

    Ok(())
}
