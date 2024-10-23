mod common;
use common::TestContext;
use predicates::prelude::*;

#[test]
fn displays_help() {
    let ctx = TestContext::new();
    let help_text = "A command-line note-taking assistant

Usage: zeke [OPTIONS] <COMMAND>

Commands:
  new     Create new notes
  ls      List and filter notes
  config  Print configuration information
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --debug    Run with debugging output
      --test     Test configuration argument
  -h, --help     Print help
  -V, --version  Print version\n";

    ctx.command().assert().failure().stderr(help_text);
    ctx.command()
        .arg("--help")
        .assert()
        .success()
        .stdout(help_text);
    ctx.command()
        .arg("help")
        .assert()
        .success()
        .stdout(help_text);
}

#[test]
fn displays_version() {
    let ctx = TestContext::new();
    let version_text = "zeke 0.7.0\n";

    ctx.command()
        .arg("-V")
        .assert()
        .success()
        .stdout(version_text);
    ctx.command()
        .arg("--version")
        .assert()
        .success()
        .stdout(version_text);
}

#[test]
fn debug_flag_enables_logging() {
    let ctx = TestContext::new();
    ctx.command()
        .arg("--debug")
        .arg("config")
        .assert()
        .success()
        .stderr(predicate::str::contains("Debug logging enabled"));
}

#[test]
fn new_subcommand_displays_help() {
    let ctx = TestContext::new();
    let help_text = "Create new notes

Usage: zeke new [OPTIONS]

Options:
  -d, --debug    Run with debugging output
      --test     Test configuration argument
  -h, --help     Print help
  -V, --version  Print version\n";

    ctx.command()
        .arg("new")
        .arg("--help")
        .assert()
        .success()
        .stdout(help_text);
}

#[test]
fn new_subcommand_not_implemented() {
    let ctx = TestContext::new();
    ctx.command()
        .arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}

#[test]
fn ls_subcommand_displays_help() {
    let ctx = TestContext::new();
    let help_text = "List and filter notes

Usage: zeke ls [OPTIONS]

Options:
  -d, --debug    Run with debugging output
      --test     Test configuration argument
  -h, --help     Print help
  -V, --version  Print version\n";

    ctx.command()
        .arg("ls")
        .arg("--help")
        .assert()
        .success()
        .stdout(help_text);
}

#[test]
fn ls_subcommand_not_implemented() {
    let ctx = TestContext::new();
    ctx.command()
        .arg("ls")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}

#[test]
fn config_subcommand_displays_help() {
    let ctx = TestContext::new();
    let help_text = "Print configuration information

Usage: zeke config [OPTIONS]

Options:
  -d, --debug          Run with debugging output
      --show-sources   Show the source of each configuration setting
      --show-defaults  Show the system default configuration settings
      --test           Test configuration argument
  -h, --help           Print help
  -V, --version        Print version\n";

    ctx.command()
        .arg("config")
        .arg("--help")
        .assert()
        .success()
        .stdout(help_text);
}

#[test]
fn config_displays_the_configuration() {
    let ctx = TestContext::new();
    ctx.command()
        .arg("config")
        .assert()
        .success()
        .stdout("test = false\n");
}

#[test]
fn config_can_display_the_configuration_with_sources() {
    let ctx = TestContext::new();
    ctx.command()
        .arg("config")
        .arg("--show-sources")
        .arg("--test")
        .assert()
        .success()
        .stdout("test = true # via argument: --test\n");
}

#[test]
fn config_can_display_the_configuration_defaults() {
    let ctx = TestContext::new();
    ctx.command()
        .arg("config")
        .arg("--show-defaults")
        .assert()
        .success()
        .stdout("test = false\n");
}

#[test]
fn config_can_display_the_configuration_defaults_with_sources() {
    let ctx = TestContext::new();
    ctx.command()
        .arg("config")
        .arg("--show-defaults")
        .arg("--show-sources")
        .assert()
        .success()
        .stdout("test = false # via default\n");
}

#[test]
fn config_priority_is_args_then_env_vars_then_config_file_then_defaults() {
    let ctx = TestContext::new();
    let config_file = ctx.create_config_file("test = true\n");
    ctx.command()
        .arg("config")
        .arg("--show-sources")
        .arg("--test")
        .env("ZEKE_TEST", "true")
        .assert()
        .success()
        .stdout("test = true # via argument: --test\n");
    ctx.command()
        .arg("config")
        .arg("--show-sources")
        .env("ZEKE_TEST", "true")
        .assert()
        .success()
        .stdout("test = true # via environment variable: ZEKE_TEST\n");
    ctx.command()
        .arg("config")
        .arg("--show-sources")
        .assert()
        .success()
        .stdout(format!(
            "test = true # via file: {}\n",
            config_file.display()
        ));
    std::fs::remove_file(config_file.path()).unwrap();
    ctx.command()
        .arg("config")
        .arg("--show-sources")
        .assert()
        .success()
        .stdout("test = false # via default\n");
}
