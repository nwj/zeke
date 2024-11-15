use assert_cmd::cargo::CommandCargoExt;
use assert_fs::prelude::*;
use assert_fs::{fixture::ChildPath, TempDir};
use std::process::Command;

pub struct TestContext {
    _temp_dir: TempDir,
    pub home_dir: ChildPath,
    pub xdg_config_dir: ChildPath,
    pub user_level_config_dir: ChildPath,
    pub notebook_dir: ChildPath,
    pub notebook_level_config_dir: ChildPath,
}

impl TestContext {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.child("home");
        let xdg_config_dir = home_dir.child(".config");
        let user_level_config_dir = xdg_config_dir.child("zeke");
        let notebook_dir = home_dir.child("notes");
        let notebook_level_config_dir = notebook_dir.child(".zeke");

        user_level_config_dir.create_dir_all().unwrap();
        notebook_level_config_dir.create_dir_all().unwrap();

        Self {
            _temp_dir: temp_dir,
            home_dir,
            xdg_config_dir,
            user_level_config_dir,
            notebook_dir,
            notebook_level_config_dir,
        }
    }

    pub fn command(&self) -> assert_cmd::Command {
        let std_cmd = Command::cargo_bin("zeke").unwrap();
        let mut cmd = assert_cmd::Command::from_std(std_cmd);
        cmd.env_clear();
        cmd.env("HOME", &self.home_dir.to_path_buf());
        cmd.env("XDG_CONFIG_HOME", &self.xdg_config_dir.to_path_buf());
        cmd.current_dir(&self.notebook_dir.to_path_buf());
        cmd
    }

    pub fn create_user_level_config_file(&self, content: &str) -> ChildPath {
        let config_file = self.user_level_config_dir.child("config.toml");
        config_file.write_str(content).unwrap();
        config_file
    }

    pub fn create_notebook_level_config_file(&self, content: &str) -> ChildPath {
        let config_file = self.notebook_level_config_dir.child("config.toml");
        config_file.write_str(content).unwrap();
        config_file
    }
}
