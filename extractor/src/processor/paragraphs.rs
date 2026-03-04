pub struct ParagraphIterator {
    text: String,
}

impl ParagraphIterator {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    fn find_split_index(&self) -> Option<usize> {
        let mut prev_char = '\0';

        for (i, char) in self.text.char_indices() {
            if char == '\n' && prev_char == '\n' {
                return Some(i - 1);
            }

            prev_char = char;
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
