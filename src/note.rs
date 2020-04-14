use crate::front_matter::FrontMatter;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Note {
    pub front_matter: FrontMatter,
    content: String,
}

impl Note {
    pub fn new() -> Note {
        Note {
            front_matter: FrontMatter::new(),
            content: String::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(format!(
            "{}\n{}",
            self.front_matter.to_yaml_string()?,
            self.content
        ))
    }

    pub fn from_string(s: String) -> Result<Note, Box<dyn Error>> {
        if !s.starts_with("---\n") {
            return Ok(Note {
                front_matter: FrontMatter::default(),
                content: s,
            });
        }

        let splits: Vec<_> = s.splitn(3, "---").collect();
        match (splits.get(1), splits.get(2)) {
            (Some(fm), Some(c)) => {
                let front_matter = FrontMatter::from_yaml_string(format!("---{}", fm))?;
                // strip_prefix is experimental right now, but could potentially replace
                // trim_start_matches here if/when that changes
                let content = c.trim_start_matches("\n").to_string();
                Ok(Note {
                    front_matter,
                    content,
                })
            }
            _ => Ok(Note {
                front_matter: FrontMatter::default(),
                content: s,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use chrono::{DateTime, Utc};
    use proptest::prelude::*;

    #[test]
    fn from_string_no_content() -> Result<(), Box<dyn Error>> {
        let s = "---\ntitle: \"Lorem ipsum dolor sit amet\"\ncreated: \"2020-04-08T00:05:56.075997Z\"\ntags:\n  - cats\nlinks_in: []\nlinks_out: []\n---";
        let a = Note::from_string(s.to_string())?;
        let b = Note {
            front_matter: FrontMatter {
                title: String::from("Lorem ipsum dolor sit amet"),
                created: Some(Utc.ymd(2020, 4, 8).and_hms_micro(0, 5, 56, 75_997)),
                tags: vec![String::from("cats")],
                links_in: Vec::new(),
                links_out: Vec::new(),
            },
            content: String::new(),
        };

        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn from_string_no_front_matter() -> Result<(), Box<dyn Error>> {
        let s = "Lorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem...";
        let a = Note::from_string(s.to_string())?;
        let b = Note {
            front_matter: FrontMatter {
                title: String::new(),
                created: None,
                tags: Vec::new(),
                links_in: Vec::new(),
                links_out: Vec::new(),
            },
            content: String::from("Lorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem..."),
        };

        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn from_string_partial_front_matter() -> Result<(), Box<dyn Error>> {
        let s = "---\ntitle: \"Lorem ipsum dolor sit amet\"\ntags: []\nlinks_in:\n  - cats.md\n---\nLorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem...";
        let a = Note::from_string(s.to_string())?;
        let b = Note {
            front_matter: FrontMatter {
                title: String::from("Lorem ipsum dolor sit amet"),
                created: None,
                tags: Vec::new(),
                links_in: vec!(String::from("cats.md")),
                links_out: Vec::new(),
            },
            content: String::from("Lorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem..."),
        };

        assert_eq!(a, b);
        Ok(())
    }

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

    prop_compose! {
        fn arb_note() (
            front_matter in arb_front_matter(),
            content in "\\PC*",
        ) -> Note {
            Note { front_matter, content }
        }
    }

    proptest! {
        #[test]
        fn proptest_to_then_from_string (n in arb_note()) {
            let converted_n = Note::from_string(n.to_string().unwrap()).unwrap();
            assert_eq!(n, converted_n)
        }
    }
}
