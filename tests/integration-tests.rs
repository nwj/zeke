mod common;
use common::zeke;
use predicates::prelude::*;

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

#[test]
fn new_subcommand_displays_help() {
    let help_text = "Create new notes

Usage: zeke new [OPTIONS]

Options:
  -d, --debug    Run with debugging output
  -h, --help     Print help
  -V, --version  Print version\n";

    zeke()
        .arg("new")
        .arg("--help")
        .assert()
        .success()
        .stdout(help_text);
}

#[test]
fn new_subcommand_not_implemented() {
    zeke()
        .arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}

#[test]
fn ls_subcommand_displays_help() {
    let help_text = "List and filter notes

Usage: zeke ls [OPTIONS]

Options:
  -d, --debug    Run with debugging output
  -h, --help     Print help
  -V, --version  Print version\n";

    zeke()
        .arg("ls")
        .arg("--help")
        .assert()
        .success()
        .stdout(help_text);
}

#[test]
fn ls_subcommand_not_implemented() {
    zeke()
        .arg("ls")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}

#[test]
fn config_subcommand_displays_help() {
    let help_text = "Print configuration information

Usage: zeke config [OPTIONS]

Options:
  -d, --debug          Run with debugging output
      --show-sources   Show the source of each configuration setting
      --show-defaults  Show the system default configuration settings
  -h, --help           Print help
  -V, --version        Print version\n";

    zeke()
        .arg("config")
        .arg("--help")
        .assert()
        .success()
        .stdout(help_text);
}

#[test]
fn config_displays_the_configuration() {
    zeke()
        .arg("config")
        .assert()
        .success()
        .stdout("debug = false\n");
}

#[test]
fn config_can_display_the_configuration_with_sources() {
    zeke()
        .arg("config")
        .arg("--show-sources")
        .arg("--debug")
        .assert()
        .success()
        .stdout("debug = true (via argument)\n");
}

#[test]
fn config_can_display_the_configuration_defaults() {
    zeke()
        .arg("config")
        .arg("--show-defaults")
        .assert()
        .success()
        .stdout("debug = false\n");
}
