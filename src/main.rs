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
                    Arg::with_name("FILE_A")
                        .help("Path to one note to link")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("FILE_B")
                        .help("Path to the other note to link")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("unlink")
                .about("Unlink a note from another note")
                .arg(
                    Arg::with_name("FILE_A")
                        .help("Path to one note to unlink")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("FILE_B")
                        .help("Path to the other note to unlink")
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
                .about("Tag one or more notes")
                .arg(
                    Arg::with_name("TAG")
                        .help("Tag to apply to the note(s)")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("FILE")
                        .help("Path(s) for the note(s) to tag")
                        .required(true)
                        .multiple(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("untag")
                .about("Remove a tag from one or more notes")
                .arg(
                    Arg::with_name("TAG")
                        .help("Tag to remove from the note(s)")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("FILE")
                        .help("Path(s) for the note(s) to untag")
                        .required(true)
                        .multiple(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("graph")
                .about("Generate a representation of the links between all notes in the DOT format")
        )
        .subcommand(SubCommand::with_name("ls").about("List all notes that meet various criteria"))
        .subcommand(SubCommand::with_name("backlink").about("Add backlinks to the front matter of all notes"))
        .subcommand(SubCommand::with_name("mv").about("Retitle a note and update any references to the old title"))
        .get_matches();

    if let Err(e) = zeke::run(&matches) {
        eprintln!("[{} error] {}", crate_name!(), e);
        process::exit(1);
    }
}
