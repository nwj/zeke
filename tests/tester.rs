use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::TempDir;
use std::process::Command;

pub struct ZekeTester {
    pub temp_dir: TempDir,
}

impl ZekeTester {
    pub fn new() -> Self {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        Self { temp_dir }
    }

    pub fn zeke(&self) -> Result<Command> {
        let mut cmd = Command::cargo_bin("zeke")?;
        cmd.current_dir(self.temp_dir.path());
        Ok(cmd)
    }

    pub fn zeke_new(&self, title: &str) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("new").arg(title);
        Ok(cmd)
    }

    pub fn zeke_link(&self, from: &str, to: &str) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("link").arg(from).arg(to);
        Ok(cmd)
    }

    pub fn zeke_unlink(&self, from: &str, to: &str) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("unlink").arg(from).arg(to);
        Ok(cmd)
    }

    pub fn zeke_tags(&self) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("tags");
        Ok(cmd)
    }

    pub fn zeke_tag(&self, tag: &str, path: &str) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("tag").arg(tag).arg(path);
        Ok(cmd)
    }

    pub fn zeke_untag(&self, tag: &str, path: &str) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("untag").arg(tag).arg(path);
        Ok(cmd)
    }

    pub fn zeke_graph(&self) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("graph");
        Ok(cmd)
    }

    pub fn zeke_mv(&self, path: &str, title: &str) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("mv").arg(path).arg(title);
        Ok(cmd)
    }

    pub fn zeke_backlink(&self) -> Result<Command> {
        let mut cmd = self.zeke()?;
        cmd.arg("backlink");
        Ok(cmd)
    }
}
