use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashSet;

pub mod xpath;

/// List all xml paths (deduplicated and sorted, where siblings have the same path if they have the same element type).
pub fn paths(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_file(path)?;
    let mut xpath: xpath::XPath = xpath::XPath::new();
    let mut buf = Vec::new();
    let mut xpath_strings = HashSet::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                xpath.push(xpath::start_tag_string(e)?);
            }
            Ok(Event::End(ref e)) => {
                xpath.pop_checked(xpath::end_tag_string(e)?);
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        xpath_strings.insert(xpath.as_string());
    }
    let mut xpath_strings: Vec<String> = xpath_strings.into_iter().collect();
    xpath_strings.sort();
    Ok(xpath_strings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        let paths = paths("example_data/note.xml").unwrap();
        assert_eq!(
            paths,
            vec![
                "",
                "notes",
                "notes/note",
                "notes/note/body",
                "notes/note/from",
                "notes/note/heading",
                "notes/note/to"
            ]
        );
    }
}
