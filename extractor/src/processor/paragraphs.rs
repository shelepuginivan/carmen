use std::mem;

/// Splits text by paragraphs, i.e. text chunks separated by 2 or more line breaks (LF or CRLF) are
/// considered paragraphs.
pub struct ParagraphSplitter {
    text: String,
}

impl ParagraphSplitter {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    fn find_split_index(&self) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut has_cr = false;

        for (i, ch) in self.text.char_indices() {
            if ch == '\r' {
                has_cr = true;
                count += 1;
                continue;
            }

            if ch == '\n' {
                count += 1;
                continue;
            }

            let threshold = if has_cr { 4 } else { 2 };
            if count >= threshold {
                return Some((i - count, count));
            }

            count = 0;
            has_cr = false;
        }

        let threshold = if has_cr { 4 } else { 2 };
        if count >= threshold {
            return Some((self.text.len() - count, count));
        }

        None
    }
}

impl Iterator for ParagraphSplitter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }

        let (idx, lf_count) = match self.find_split_index() {
            Some(v) => v,
            None => return Some(mem::take(&mut self.text)),
        };

        let paragraph = self.text.drain(..idx).collect();

        self.text.drain(..lf_count);

        Some(paragraph)
    }
}

#[cfg(test)]
mod tests {
    use super::ParagraphSplitter;

    #[test]
    fn test_paragraph_splitter() {
        let tests = vec![
            (
                "Lorem Ipsum\n\ndolor\nsit\namet",
                vec!["Lorem Ipsum", "dolor\nsit\namet"],
            ),
            ("a\n\nb\n\nc\n\ndef", vec!["a", "b", "c", "def"]),
            ("a\r\n\r\nb\r\n\r\nc\r\n\r\ndef", vec!["a", "b", "c", "def"]),
            ("0\n\n1\n\n\n2\n\n\n\n3", vec!["0", "1", "2", "3"]),
        ];

        for (case_idx, (input, expected)) in tests.into_iter().enumerate() {
            let actual: Vec<String> = ParagraphSplitter::new(String::from(input)).collect();

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
