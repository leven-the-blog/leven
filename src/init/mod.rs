use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn init() -> Result<(), Box<Error>> {
    let mut dir = env::current_dir()?;

    // Ensure the directory is empty.

    if fs::read_dir(&dir)?.next().is_some() {
       return Err("current directory not empty".into());
    }

    // Create directories.

    dir.push("posts");
    fs::create_dir(&dir)?;
    dir.pop();
    
    dir.push("out");
    fs::create_dir(&dir)?;
    dir.pop();

    dir.push("templates");
    fs::create_dir(&dir)?;

    // Create templates.

    let templates: &[(&str, &[u8])] = &[
        ("index.hbs", include_bytes!("index.hbs")),
        ("archive.hbs", include_bytes!("archive.hbs")),
        ("post.hbs", include_bytes!("post.hbs")),
    ];

    for &(name, bytes) in templates {
        dir.push(name);
        File::create(&dir)?.write_all(bytes)?;
        dir.pop();
    }

    Ok(())
}
