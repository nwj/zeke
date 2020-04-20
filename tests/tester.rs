use assert_cmd::prelude::*;
use assert_fs::TempDir;
use std::error::Error;
use std::process::Command;

pub struct ZekeTester {
    pub temp_dir: TempDir,
}

impl ZekeTester {
    pub fn new() -> Self {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        Self { temp_dir }
    }

    pub fn zeke(&self) -> Result<Command, Box<dyn Error>> {
        let mut cmd = Command::cargo_bin("zeke")?;
        cmd.current_dir(self.temp_dir.path());
        Ok(cmd)
    }

    pub fn zeke_new(&self, path: Option<&str>) -> Result<Command, Box<dyn Error>> {
        let mut cmd = self.zeke()?;
        cmd.arg("new");
        if let Some(p) = path {
            cmd.arg(p);
        }
        Ok(cmd)
    }

    pub fn zeke_link(&self, from: &str, to: &str) -> Result<Command, Box<dyn Error>> {
        let mut cmd = self.zeke()?;
        cmd.arg("link").arg(from).arg(to);
        Ok(cmd)
    }

    pub fn zeke_tag(&self, path: &str, tag: &str) -> Result<Command, Box<dyn Error>> {
        let mut cmd = self.zeke()?;
        cmd.arg("tag").arg(path).arg(tag);
        Ok(cmd)
    }
}
