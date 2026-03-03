pub trait Extractor {
    fn supports_format(&self, filename: &str) -> bool;
    fn extract_text(&self, bytes: Vec<u8>) -> anyhow::Result<String>;
}
