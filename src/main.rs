#[macro_use] extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate tenjin;
extern crate badlog;
extern crate chrono;
extern crate pulldown_cmark;
extern crate rayon;
extern crate slug;
extern crate toml;

mod build;
mod error;
mod init;
mod theme;
mod util;


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

        (@subcommand new =>
            (about: "Create a new blog")
            (@arg path: +required "Where to create the blog")
        )

        (@subcommand theme =>
            (about: "Change the blog theme")
            (@arg repo: +required "The theme repository")
        )

        (@subcommand build =>
            (about: "Build the blog into static files")
        )
    ).get_matches();

    let (cmd, matches) = matches.subcommand();
    let matches = matches.unwrap();

    match cmd {
        "new" => init::execute(matches.value_of("path").unwrap()),
        "theme" => theme::execute(matches.value_of("repo").unwrap()),
        "build" => build::execute(),
        _ => unreachable!(),
    }
}
