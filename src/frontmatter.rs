use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FrontMatter {
    title: String,
    created: DateTime<Utc>,
    tags: Vec<String>,
    links_in: Vec<String>,
    links_out: Vec<String>,
}

impl FrontMatter {
    pub fn default() -> FrontMatter {
        FrontMatter {
            title: String::new(),
            created: Utc::now(),
            tags: Vec::new(),
            links_in: Vec::new(),
            links_out: Vec::new(),
        }
    }

    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(format!("{}\n---", yaml))
    }

    pub fn _from_yaml(s: String) -> Result<FrontMatter, Box<dyn Error>> {
        let trimmed = s.trim_end_matches("\n---");
        let frontmatter = serde_yaml::from_str(&trimmed)?;
        Ok(frontmatter)
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
        fn arb_frontmatter() (
            title in "\\PC*",
            created in arb_datetime(),
            tags in proptest::collection::vec("\\PC*", 3),
            links_in in proptest::collection::vec("\\PC*", 3),
            links_out in proptest::collection::vec("\\PC*", 3),
        ) -> FrontMatter {
            FrontMatter { title, created, tags, links_in, links_out }
        }
    }

    proptest! {
        #[test]
        fn proptest_to_then_from_yaml (fm in arb_frontmatter()) {
            let converted_fm = FrontMatter::_from_yaml(fm.to_yaml().unwrap()).unwrap();
            assert_eq!(fm, converted_fm)
        }
    }
}
