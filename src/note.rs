use crate::front_matter::FrontMatter;
use regex::Regex;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Note {
    pub front_matter: FrontMatter,
    pub content: String,
    pub path: Option<PathBuf>,
}

impl Note {
    pub fn new() -> Note {
        Note {
            front_matter: FrontMatter::new(),
            content: String::new(),
            path: None,
        }
    }

    pub fn write_to_file(&self, create_new: bool) -> Result<(), Box<dyn Error>> {
        let path = self.path.as_ref().ok_or_else(|| {
            IOError::new(
                IOErrorKind::NotFound,
                "Cannot write note as it does not have a corresponding path.",
            )
        })?;

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(create_new)
            .truncate(true)
            .open(&path)?;

        file.write_all(self.to_string()?.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(path: &PathBuf) -> Result<Note, Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .create_new(false)
            .open(&path)?;

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        let (front_matter, content) = Note::from_string(file_content)?;
        Ok(Note {
            front_matter,
            content,
            path: Some(path.clone()),
        })
    }

    pub fn generate_path(&self) -> Result<PathBuf, Box<dyn Error>> {
        let title_part = Regex::new(r"\s")?
            .replace_all(&self.front_matter.title, "_")
            .to_lowercase();
        let path_string = match self.front_matter.created {
            Some(ts) => format!("{}-{}.md", ts.format("%Y%m%d"), title_part),
            None => format!("{}.md", title_part),
        };
        Ok(PathBuf::from(path_string))
    }

    fn to_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(format!(
            "{}\n{}",
            self.front_matter.to_yaml_string()?,
            self.content
        ))
    }

    fn from_string(s: String) -> Result<(FrontMatter, String), Box<dyn Error>> {
        if !s.starts_with("---\n") {
            return Ok((FrontMatter::default(), s));
        }

        let splits: Vec<_> = s.splitn(3, "---").collect();
        match (splits.get(1), splits.get(2)) {
            (Some(fm), Some(c)) => {
                let front_matter = FrontMatter::from_yaml_string(format!("---{}", fm))?;
                let content = c.trim_start_matches("\n").to_string();
                Ok((front_matter, content))
            }
            _ => Ok((FrontMatter::default(), s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use chrono::{DateTime, Utc};
    use proptest::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn from_string_no_content() -> Result<(), Box<dyn Error>> {
        let s = "---\ntitle: \"Lorem ipsum dolor sit amet\"\ncreated: \"2020-04-08T00:05:56.075997Z\"\ntags:\n  - cats\nlinks: []\n---";
        let (front_matter, content) = Note::from_string(s.to_string())?;
        let a = Note {
            front_matter,
            content,
            path: None,
        };
        let mut ts = HashSet::new();
        ts.insert(String::from("cats"));
        let b = Note {
            front_matter: FrontMatter {
                title: String::from("Lorem ipsum dolor sit amet"),
                created: Some(Utc.ymd(2020, 4, 8).and_hms_micro(0, 5, 56, 75_997)),
                tags: ts,
                links: HashSet::new(),
            },
            content: String::new(),
            path: None,
        };

        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn from_string_no_front_matter() -> Result<(), Box<dyn Error>> {
        let s = "Lorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem...";
        let (front_matter, content) = Note::from_string(s.to_string())?;
        let a = Note {
            front_matter,
            content,
            path: None,
        };
        let b = Note {
            front_matter: FrontMatter {
                title: String::new(),
                created: None,
                tags: HashSet::new(),
                links: HashSet::new(),
            },
            content: String::from("Lorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem..."),
            path: None,
        };

        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn from_string_partial_front_matter() -> Result<(), Box<dyn Error>> {
        let s = "---\ntitle: \"Lorem ipsum dolor sit amet\"\ntags: []\nlinks:\n  - cats.md\n---\nLorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem...";
        let (front_matter, content) = Note::from_string(s.to_string())?;
        let a = Note {
            front_matter,
            content,
            path: None,
        };
        let mut ls = HashSet::new();
        ls.insert(PathBuf::from("cats.md"));
        let b = Note {
            front_matter: FrontMatter {
                title: String::from("Lorem ipsum dolor sit amet"),
                created: None,
                tags: HashSet::new(),
                links: ls,
            },
            content: String::from("Lorem ipsum dolir sit amet\nSed ut perspiciatis unde omnis iste natus error sit voluptatem..."),
            path: None,
        };

        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn generate_path() -> Result<(), Box<dyn Error>> {
        let n = Note {
            front_matter: FrontMatter {
                title: String::from("This is a test"),
                created: Some(Utc.ymd(2020, 4, 8).and_hms_micro(0, 5, 56, 75_997)),
                tags: HashSet::new(),
                links: HashSet::new(),
            },
            content: String::new(),
            path: None,
        };
        assert_eq!(
            n.generate_path()?,
            PathBuf::from("20200408-this_is_a_test.md")
        );
        Ok(())
    }

    #[test]
    fn generate_path_no_created_date() -> Result<(), Box<dyn Error>> {
        let n = Note {
            front_matter: FrontMatter {
                title: String::from("This is a test"),
                created: None,
                tags: HashSet::new(),
                links: HashSet::new(),
            },
            content: String::new(),
            path: None,
        };
        assert_eq!(n.generate_path()?, PathBuf::from("this_is_a_test.md"));
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
        fn arb_path() (s in "[^\\p{C}\\p{Z}]*") -> PathBuf {
            PathBuf::from(s)
        }
    }

    prop_compose! {
        fn arb_front_matter() (
            // The regex here is all non-control, non-unicode-whitespace characters
            title in "[^\\p{C}\\p{Z}]*",
            date_time in arb_datetime(),
            tags in proptest::collection::hash_set("[^\\p{C}\\p{Z}]*", 3),
            links in proptest::collection::hash_set(arb_path(), 3),
        ) -> FrontMatter {
            let created = Some(date_time);
            FrontMatter { title, created, tags, links }
        }
    }

    prop_compose! {
        fn arb_note() (
            front_matter in arb_front_matter(),
            content in "\\PC*",
            path in arb_path(),
        ) -> Note {
            let path = Some(path);
            Note { front_matter, content, path }
        }
    }

    proptest! {
        #[test]
        fn proptest_to_then_from_string (n in arb_note()) {
            let s = n.to_string().unwrap();
            let (front_matter, content) = Note::from_string(s).unwrap();
            let n2 = Note {front_matter, content, path: None};
            assert_eq!(n.front_matter, n2.front_matter);
            assert_eq!(n.content, n2.content);
        }
    }
}
