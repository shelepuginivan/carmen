use std::mem;

/// Splits text by paragraphs, i.e. text chunks separated by 2 or more line breaks (LF or CRLF) are
/// considered paragraphs.
pub struct ParagraphIterator {
    text: String,
}

impl ParagraphIterator {
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

impl Iterator for ParagraphIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }

        let idx = match self.find_split_index() {
            Some(i) => i,
            None => return Some(std::mem::take(&mut self.text)),
        };

        let paragraph = self.text.drain(..idx).collect::<String>();

        self.text.drain(..2);

        Some(paragraph)
    }
}
