use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new note")
                .arg(
                    Arg::with_name("TITLE")
                        .help("Title for the new note")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("edit")
                        .short("e")
                        .help("Opens the new note in the editor specified by the $ZEKE_EDITOR env variable")
                        .long("edit"),
                ),
        )
        .subcommand(
            SubCommand::with_name("link")
                .about("Link a note to another note")
                .arg(
                    Arg::with_name("FROM")
                        .help("Path to the note to link from")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("TO")
                        .help("Path to the note to link to")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("unlink")
                .about("Unlink a note from another note")
                .arg(
                    Arg::with_name("FROM")
                        .help("Path to the note to unlink from")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("TO")
                        .help("Path to the note to unlink to")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("tags")
                .about("List all tags")
        )
        .subcommand(
            SubCommand::with_name("tag")
                .about("Tag a note")
                .arg(
                    Arg::with_name("FILE")
                        .help("Path to the note to tag")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("TAG")
                        .help("Tag to apply to the note")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("untag")
                .about("Remove a tag from a note")
                .arg(
                    Arg::with_name("FILE")
                        .help("Path to the note to untag")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("TAG")
                        .help("Tag to remove from the note")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List all notes that meet various criteria"))
        .subcommand(SubCommand::with_name("backlink").about("Adds backlinks to the front matter of all notes"))
        .get_matches();

    if let Err(e) = zeke::run(&matches) {
        eprintln!("[{} error] {}", crate_name!(), e);
        process::exit(1);
    }
}
