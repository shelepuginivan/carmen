use super::Extractor;

pub struct PlaintextExtractor {}

impl Extractor for PlaintextExtractor {
    fn supports_format(&self, filename: &str) -> bool {
        filename.ends_with(".txt")
    }

    fn extract_text(&self, bytes: Vec<u8>) -> anyhow::Result<String> {
        Ok(String::from_utf8(bytes)?)
    }
}
