use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

mod content;
mod front_matter;
mod fs;
mod note;
mod subcommands;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(display_order = 1)]
    /// Create a new note
    New {
        /// Title for the new note
        title: String,
        /// Open the new note in the editor specified by the EDITOR or ZEKE_EDITOR env variables
        #[clap(long, short)]
        edit: bool,
    },
    #[clap(display_order = 2)]
    /// Rename a note and update references to the old name
    Mv {
        #[clap(parse(from_os_str))]
        /// Path to the note to move
        path: PathBuf,
        /// New title for the note
        title: String,
    },
    #[clap(display_order = 3)]
    /// Tag one or more notes
    Tag {
        /// Tag to apply to the note(s)
        tag: String,
        #[clap(parse(from_os_str))]
        /// Path(s) for the note(s) to tag
        paths: Vec<PathBuf>,
    },
    #[clap(display_order = 4)]
    /// Remove a tag from one or more notes
    Untag {
        /// Tag to remove from the note(s)
        tag: String,
        #[clap(parse(from_os_str))]
        /// Path(s) for the note(s) to untag
        paths: Vec<PathBuf>,
    },
    #[clap(display_order = 5)]
    /// List all tags
    Tags,
    #[clap(display_order = 6)]
    /// Link a note to another note
    Link {
        #[clap(parse(from_os_str))]
        /// Path to one note to link
        path_a: PathBuf,
        #[clap(parse(from_os_str))]
        /// Path to the other note to link
        path_b: PathBuf,
    },
    #[clap(display_order = 7)]
    /// Unlink a note from another note
    Unlink {
        #[clap(parse(from_os_str))]
        /// Path to one note to unlink
        path_a: PathBuf,
        #[clap(parse(from_os_str))]
        /// Path to the other note to unlink
        path_b: PathBuf,
    },
    #[clap(display_order = 8)]
    /// Add backlinks to the front matter of all notes
    Backlink,
}

fn main() {
    match run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1)
        }
    }
}

fn run() -> Result<i32> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { title, edit } => subcommands::new::run(title, edit),
        Commands::Mv { path, title } => subcommands::mv::run(path, title),
        Commands::Tag { tag, paths } => subcommands::tag::run(tag, paths),
        Commands::Untag { tag, paths } => subcommands::untag::run(tag, paths),
        Commands::Tags => subcommands::tags::run(),
        Commands::Link { path_a, path_b } => subcommands::link::run(path_a, path_b),
        Commands::Unlink { path_a, path_b } => subcommands::unlink::run(path_a, path_b),
        Commands::Backlink => subcommands::backlink::run(),
    }
}
