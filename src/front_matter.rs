use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[serde(default)]
pub struct FrontMatter {
    pub title: String,
    pub created: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub links_in: Vec<String>,
    pub links_out: Vec<String>,
}

impl FrontMatter {
    pub fn new() -> FrontMatter {
        FrontMatter {
            title: String::new(),
            created: Some(Utc::now()),
            tags: Vec::new(),
            links_in: Vec::new(),
            links_out: Vec::new(),
        }
    }

    pub fn to_yaml_string(&self) -> Result<String, Box<dyn Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(format!("{}\n---", yaml))
    }

    pub fn from_yaml_string(s: String) -> Result<FrontMatter, Box<dyn Error>> {
        let trimmed = s.trim_end().trim_end_matches("---").trim_end();
        let front_matter = serde_yaml::from_str(&trimmed)?;
        Ok(front_matter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use proptest::prelude::*;

    prop_compose! {
        // The dates here are 1900-01-01 to 2200-01-01. Limited to this range because chrono panics
        // on some values in the full i64 (for sec) and u32 (for nsec) range.
        fn arb_datetime() (s in -2_208_988_800..7_258_118_400i64, ns in 0..1_000_000_000u32) -> DateTime<Utc> {
            Utc.timestamp(s, ns)
        }
    }

    prop_compose! {
        fn arb_front_matter() (
            // The regex here is all non-control, non-unicode-whitespace characters
            title in "[^\\p{C}\\p{Z}]*",
            date_time in arb_datetime(),
            tags in proptest::collection::vec("[^\\p{C}\\p{Z}]*", 3),
            links_in in proptest::collection::vec("[^\\p{C}\\p{Z}]*", 3),
            links_out in proptest::collection::vec("[^\\p{C}\\p{Z}]*", 3),
        ) -> FrontMatter {
            let created = Some(date_time);
            FrontMatter { title, created, tags, links_in, links_out }
        }
    }

    proptest! {
        #[test]
        fn proptest_to_then_from_yaml (fm in arb_front_matter()) {
            let converted_fm = FrontMatter::from_yaml_string(fm.to_yaml_string().unwrap()).unwrap();
            assert_eq!(fm, converted_fm)
        }
    }
}
