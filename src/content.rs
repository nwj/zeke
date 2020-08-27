use path_clean::PathClean;
use pulldown_cmark::CowStr;
use pulldown_cmark::Event as MarkdownEvent;
use pulldown_cmark::Parser as MarkdownParser;
use pulldown_cmark::Tag as MarkdownTag;
use pulldown_cmark_to_cmark::cmark;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Content(String);

impl Content {
    pub fn new() -> Content {
        Content(String::new())
    }

    fn _replace_note_links<P: AsRef<Path>>(
        &self,
        from: &P,
        to: &P,
    ) -> Result<Content, Box<dyn Error>> {
        let from_ref = from.as_ref();
        let to_ref = to.as_ref();

        if !self._has_note_link(&from_ref) {
            return Ok(Content::from(self.0.clone()));
        }

        let mut s = String::with_capacity(&self.0.len() + 128);
        let md = self._markdown_parser().map(|event| match event.clone() {
            MarkdownEvent::Start(MarkdownTag::Link(link_type, url, title)) => {
                if PathBuf::from(url.into_string()).clean() == from_ref {
                    MarkdownEvent::Start(MarkdownTag::Link(
                        link_type,
                        CowStr::from(to_ref.to_string_lossy().into_owned()),
                        title,
                    ))
                } else {
                    event
                }
            }
            MarkdownEvent::End(MarkdownTag::Link(link_type, url, title)) => {
                if PathBuf::from(url.into_string()).clean() == from_ref {
                    MarkdownEvent::End(MarkdownTag::Link(
                        link_type,
                        CowStr::from(to_ref.to_string_lossy().into_owned()),
                        title,
                    ))
                } else {
                    event
                }
            }
            _ => event,
        });
        cmark(md, &mut s, None)?;
        Ok(Content::from(s))
    }

    fn _markdown_parser(&self) -> MarkdownParser {
        MarkdownParser::new(&self.0)
    }

    fn _has_note_link<P: AsRef<Path>>(&self, target: &P) -> bool {
        for event in self._markdown_parser() {
            if let MarkdownEvent::Start(MarkdownTag::Link(_, url, _)) = event {
                if PathBuf::from(url.into_string()).clean() == target.as_ref() {
                    return true;
                }
            }
        }

        false
    }

    fn _get_note_links(&self) -> Vec<PathBuf> {
        self._markdown_parser()
            .filter_map(|event| match event {
                MarkdownEvent::Start(MarkdownTag::Link(_, url, _)) => {
                    Some(PathBuf::from(url.into_string()).clean())
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

    #[test]
    fn replace_note_links() -> Result<(), Box<dyn Error>> {
        let content = Content::from("This is a [message](one.md) with [some](two.md) [note](two.md) [links](https://www.google.com).");
        assert_eq!(
            content._replace_note_links(&Path::new("two.md"), &Path::new("three.md"))?,
            Content::from("This is a [message](one.md) with [some](three.md) [note](three.md) [links](https://www.google.com).")
        );
        Ok(())
    }

    #[test]
    fn replace_note_links_doesnt_modify_content_if_no_link() -> Result<(), Box<dyn Error>> {
        let content = Content::from("This is a [message](one.md) with [some](two.md) [note](two.md) [links](https://www.google.com).");
        assert_eq!(
            content._replace_note_links(&Path::new("garbage.md"), &Path::new("more_garbage.md"))?,
            content
        );
        Ok(())
    }

    #[test]
    fn has_note_link() -> Result<(), Box<dyn Error>> {
        let content = Content::from("This is a [message](./one.md) with [some](two.md) [note](catdog.md) [links](https://www.google.com).");
        assert!(content._has_note_link(&"one.md"));
        assert!(!content._has_note_link(&"dog.md"));
        Ok(())
    }

    #[test]
    fn get_note_links() -> Result<(), Box<dyn Error>> {
        let content = Content::from("This is a [message](one.md) with [some](two.md) [note](two.md) [links](https://www.google.com).");
        assert_eq!(
            content._get_note_links(),
            vec!(
                PathBuf::from("one.md"),
                PathBuf::from("two.md"),
                PathBuf::from("two.md")
            )
        );
        Ok(())
    }
}
