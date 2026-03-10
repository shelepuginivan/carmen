use comrak::nodes::NodeValue;
use comrak::{Arena, Options, parse_document};

use super::Extractor;

#[derive(Default)]
pub struct MarkdownExtractor {}

impl Extractor for MarkdownExtractor {
    fn supports_format(&self, filename: &str) -> bool {
        filename.ends_with(".md")
    }

    fn extract_text(&self, bytes: Vec<u8>) -> anyhow::Result<String> {
        let arena = Arena::new();
        let root = parse_document(&arena, str::from_utf8(&bytes)?, &Options::default());
        let mut extracted_text = String::new();

        for node in root.descendants() {
            if let NodeValue::Text(ref text) = node.data.borrow().value {
                extracted_text.push_str(text);
            }
        }

        Ok(extracted_text)
    }
}
