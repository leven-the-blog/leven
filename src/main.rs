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
mod error;
mod init;
mod post;

use build::build;
use init::init;
use post::post;
use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{self, Command};

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
            (about: "Creates a new blog")
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

    let done = match cmd {
        "init" => init(),
        "post" => {
            let title = matches.value_of("TITLE").unwrap();
            let edit = matches.is_present("edit");
            post(title, edit)
        },
        "build" => build(),
        _ => unreachable!(),
    };

    match done {
        Ok(()) => process::exit(0),
        Err(e) => {
            error!("{}", e);
            process::exit(1)
        },
    }
}
