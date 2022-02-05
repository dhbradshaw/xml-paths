use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use std::fmt;

pub fn start_tag_string(bytes_start: &BytesStart) -> Result<String, Box<dyn std::error::Error>> {
    let tag = bytes_start.name();
    let tag = tag.to_owned();
    let tag = String::from_utf8(tag)?;
    Ok(tag)
}
pub fn end_tag_string(bytes_end: &BytesEnd) -> Result<String, Box<dyn std::error::Error>> {
    let tag = bytes_end.name();
    let tag = tag.to_owned();
    let tag = String::from_utf8(tag)?;
    Ok(tag)
}

pub struct XPath(Vec<String>);

impl XPath {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn push(&mut self, tag: String) {
        self.0.push(tag);
    }
    pub fn pop(&mut self) -> Option<String> {
        self.0.pop()
    }
    pub fn pop_checked(&mut self, tag: String) {
        assert_eq!(self.pop().expect("can't end without starting."), tag);
    }
    pub fn as_string(&self) -> String {
        self.0.join("/")
    }
}

impl Default for XPath {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for XPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl fmt::Display for XPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
