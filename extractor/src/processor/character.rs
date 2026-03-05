use std::mem;

pub struct CharacterSplitter {
    text: String,
    chunk_size: usize,
    chunk_overlap: usize,
}

impl CharacterSplitter {
    pub fn new(text: String, chunk_size: usize, chunk_overlap: usize) -> Self {
        if chunk_size <= chunk_overlap {
            panic!("chunk_size must be greater than chunk_overlap");
        }

        Self {
            text,
            chunk_size,
            chunk_overlap,
        }
    }
}

impl Iterator for CharacterSplitter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }

        if self.text.len() < self.chunk_size {
            return Some(mem::take(&mut self.text));
        }

        let next_chunk: String = self
            .text
            .drain(..self.chunk_size - self.chunk_overlap)
            .collect();

        let overlap = &self.text[..self.chunk_overlap].to_string();

        Some(next_chunk + overlap)
    }
}
