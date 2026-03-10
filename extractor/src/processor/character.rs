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
        if self.text.len() <= self.chunk_overlap {
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

#[cfg(test)]
mod tests {
    use super::CharacterSplitter;

    #[test]
    fn test_character_splitter() {
        let tests = vec![(
            (
                "Lorem ipsum dolor sit amet. A accusamus consequatur et soluta autem aut neque corrupti non galisum molestiae est facilis voluptatibus cum omnis fuga",
                20,
                4,
            ),
            vec![
                "Lorem ipsum dolor si",
                "r sit amet. A accusa",
                "cusamus consequatur ",
                "tur et soluta autem ",
                "tem aut neque corrup",
                "rrupti non galisum m",
                "um molestiae est fac",
                " facilis voluptatibu",
                "tibus cum omnis fuga",
            ],
        )];

        for (case_idx, ((text, chunk_size, chunk_overlap), expected)) in
            tests.into_iter().enumerate()
        {
            let actual: Vec<String> =
                CharacterSplitter::new(String::from(text), chunk_size, chunk_overlap).collect();

            if expected.len() != actual.len() {
                panic!(
                    "case {case_idx}: length mismatch (want {}, got {})",
                    expected.len(),
                    actual.len()
                );
            }

            for (par_idx, (lhs, rhs)) in actual.iter().zip(expected).enumerate() {
                if lhs != rhs {
                    panic!(
                        "case {case_idx}: paragraph {par_idx} mismatch (want '{rhs}', got '{lhs}')"
                    )
                }
            }
        }
    }
}
