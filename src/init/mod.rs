use error::Result;
use std::{env, fs};
use std::fs::File;
use std::io::Write;

macro_rules! files {
    ( $( $file:expr, )* ) => {
        &[$( ($file, include_bytes!($file)), )*]
    }
}

fn init(name: &str) -> Result<()> {
    // If the directory doesn't exist, create it and cd into it.
    // Otherwise, error.
    
    fs::create_dir(name)?;
    env::set_current_dir(name)?;

    // Create directories.
    
    let dirs = &[
        "content",
        "theme",
        "theme/assets",
        "theme/templates",
    ];

    for dir in dirs {
        fs::create_dir(dir)?;
    }

    // Create files.

    let files: &[(&str, &[u8])] = files![
        "Leven.toml",
        "content/Hello, Stranger!.md",
        "theme/templates/index.html",
        "theme/templates/archive.html",
        "theme/templates/post.html",
    ];

    for &(path, bytes) in files {
        let mut file = File::create(path)?;
        file.write_all(bytes)?;
    }


    Ok(())
}

pub fn execute(name: &str) {
    match init(name) {
        Ok(()) => {
            info!("blog ready :)");
            info!("see `content/Hello, Stranger!.md` for an introduction");
        }
        Err(e) => {
            error!("initialization failed ({})", e);
        }
    }
}
