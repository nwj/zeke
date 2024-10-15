use clap::{crate_version, Arg, ArgAction, ArgMatches, Command};

pub fn get_cli_arguments() -> ArgMatches {
    let cli_command = Command::new("zeke")
        .version(crate_version!())
        .about("A command-line note-taking assistant")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .propagate_version(true)
        .help_expected(true)
        .max_term_width(80)
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .action(ArgAction::SetTrue)
                .help("Run with debugging output")
                .global(true),
        )
        .subcommand(Command::new("new").about("Create new notes"))
        .subcommand(Command::new("ls").about("List and filter notes"))
        .subcommand(Command::new("config").about("Print configuration information"));

    cli_command.get_matches()
}
