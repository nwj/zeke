mod common;
use common::zeke;

#[test]
fn displays_help() {
    let help_text = "A command-line note-taking assistant

Usage: zeke [OPTIONS] <COMMAND>

Commands:
  new     Create new notes
  ls      List and filter notes
  config  Print configuration information
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --debug    Run with debugging output
  -h, --help     Print help
  -V, --version  Print version\n";

    zeke().assert().failure().stderr(help_text);
    zeke().arg("--help").assert().success().stdout(help_text);
    zeke().arg("help").assert().success().stdout(help_text);
}

#[test]
fn displays_version() {
    let version_text = "zeke 0.7.0\n";

    zeke().arg("-V").assert().success().stdout(version_text);
    zeke()
        .arg("--version")
        .assert()
        .success()
        .stdout(version_text);
}
