use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

pub fn zeke() -> assert_cmd::Command {
    let cmd = Command::cargo_bin("zeke").unwrap();
    assert_cmd::Command::from_std(cmd)
}
