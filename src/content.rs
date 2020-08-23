use pulldown_cmark::Event as MarkdownParseEvent;
use pulldown_cmark::Parser as MarkdownParser;
use pulldown_cmark::Tag as MarkdownTag;
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Content(String);

impl Content {
    pub fn new() -> Content {
        Content(String::new())
    }

    fn _parse_note_links_from_content(&self) -> Vec<PathBuf> {
        MarkdownParser::new(&self.0)
            .filter_map(|event| match event {
                MarkdownParseEvent::Start(MarkdownTag::Link(_, l, _)) => {
                    Some(PathBuf::from(l.into_string()))
                }
                _ => None,
            })
            .filter(|path| matches!(path.extension().and_then(OsStr::to_str), Some("md")))
            .collect()
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Content {
    fn from(s: &str) -> Content {
        Content(String::from(s))
    }
}

impl From<String> for Content {
    fn from(s: String) -> Content {
        Content(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn parse_note_links_from_content() -> Result<(), Box<dyn Error>> {
        let content = Content::from("This is a [message](one.md) with [some](two.md) [note](two.md) [links](https://www.google.com).");
        assert_eq!(
            content._parse_note_links_from_content(),
            vec!(
                PathBuf::from("one.md"),
                PathBuf::from("two.md"),
                PathBuf::from("two.md")
            )
        );
        Ok(())
    }
}
