use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontMatter {
    title: String,
    created: DateTime<Local>,
    tags: Vec<String>,
    links_in: Vec<String>,
    links_out: Vec<String>,
}

impl FrontMatter {
    pub fn default() -> FrontMatter {
        FrontMatter {
            title: String::new(),
            created: Local::now(),
            tags: Vec::new(),
            links_in: Vec::new(),
            links_out: Vec::new(),
        }
    }

    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(format!("{}\n---", yaml))
    }
}
