mod paragraphs;

pub use paragraphs::*;

use anyhow::bail;

use crate::extractors::Extractor;

#[derive(Default)]
pub struct Processor {
    extractors: Vec<Box<dyn Extractor>>,
}

impl Processor {
    pub fn register_extractor(&mut self, extractor: Box<dyn Extractor>) {
        self.extractors.push(extractor);
    }

    pub fn process(
        &self,
        filename: &str,
        bytes: Vec<u8>,
    ) -> anyhow::Result<impl Iterator<Item = String>> {
        let text = self.extract_text(filename, bytes)?;

        Ok(ParagraphSplitter::new(text))
    }

    fn extract_text(&self, filename: &str, bytes: Vec<u8>) -> anyhow::Result<String> {
        for extractor in self.extractors.iter() {
            if extractor.supports_format(filename) {
                return extractor.extract_text(bytes);
            }
        }

        bail!("no matching extractors")
    }
}
