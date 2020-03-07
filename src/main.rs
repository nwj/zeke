#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("new").about("Create a new note").arg(
                Arg::with_name("PATH")
                    .help("Path for the new note. Autogenerated if not provided.")
                    .required(false)
                    .index(1),
            ),
        )
        .subcommand(SubCommand::with_name("link").about("Link a note to another note or notes"))
        .subcommand(SubCommand::with_name("unlink").about("Unlink a note from another note or notes"))
        .subcommand(SubCommand::with_name("tag").about("Tag one or more notes"))
        .subcommand(SubCommand::with_name("untag").about("Remove a tag from one or more notes"))
        .subcommand(SubCommand::with_name("list").about("List all notes that meet various criteria"))
        .subcommand(SubCommand::with_name("backlink").about("Adds backlinks to the frontmatter of all notes"))
        .get_matches();

    if let Err(e) = zeke::run(&matches) {
        eprintln!("[{} error] {}", crate_name!(), e);
        process::exit(1);
    }
}
