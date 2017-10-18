extern crate badlog;
extern crate chrono;
#[macro_use] extern crate clap;
extern crate glob;
extern crate handlebars;
#[macro_use] extern crate log;
extern crate pulldown_cmark;
#[macro_use] extern crate serde_json;
extern crate slug;

mod build;
mod init;
mod post;

use build::build;
use init::init;
use post::post;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    badlog::init_from_env("LOG_LEVEL");

    let matches = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())

        (@setting SubcommandRequiredElseHelp)
        (@setting VersionlessSubcommands)
        (@setting DisableHelpSubcommand)
        (@setting ColoredHelp)

        (@subcommand init =>
            (about: "Creates a new blog in the current directory")
        )

        (@subcommand post =>
            (about: "Creates a new post")
            (@arg TITLE: +required "The title of the post")
            (@arg edit: -e --edit "Open the post in the EDITOR")
        )

        (@subcommand build =>
            (about: "Builds the blog")
        )
    ).get_matches();

    let (cmd, matches) = matches.subcommand();
    let matches = matches.unwrap();

    match cmd {
        "init" => match init() {
            Ok(()) => info!("blog initialized"),
            Err(e) => error!("{}", e),
        },

        "post" => match post(matches.value_of("TITLE").unwrap()) {
            Ok(path) => {
                info!("post created at `{}`", path.display());
                if matches.is_present("edit") {
                    edit(&path);
                }
            },
            Err(e) => error!("{}", e),
        },

        "build" => match build() {
            Ok(()) => info!("blog built"),
            Err(e) => error!("{}", e),
        },

        _ => unreachable!(),
    };
}

fn edit(path: &Path) {
    if let Some(ed) = env::var_os("EDITOR") {
        match Command::new(ed).arg(path).spawn() {
            Ok(mut child) => {
                let _ = child.wait();
            },
            Err(e) => warn!("{}", e),
        }
    } else {
        warn!("EDITOR not set");
    }
}
