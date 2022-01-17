#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a> {
    index: Vec<&'a str>,
}

impl<'a> KWIndex<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        KWIndex { index: Vec::new() }
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this `KWIndex`
    /// index.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up an index.
    ///
    /// Words are separated by whitespace or punctuation,
    /// and consist of a span of one or more consecutive
    /// letters (any UTF-8 character in the "letter" class)
    /// with no internal punctuation.
    ///
    /// For example, the text
    ///
    /// ```text
    /// "It ain't over untïl it ain't, over."
    /// ```
    ///
    /// contains the sequence of words `"It"`, `"over"`,
    /// `"untïl"`, `"it"`, `"over"`.
    ///
    /// # Examples
    ///
    /// ```
    /// let index = kwindex::KWIndex::new()
    ///     .extend_from_text("Hello world.");
    /// assert_eq!(2, index.len());
    /// assert_eq!(1, index.count_matches("world"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        for mut word in target.split_whitespace() {
            let mut add_word = false;

            word = word.trim_end_matches(|c: char|  c.is_ascii_punctuation());

            for letter in word.chars() {
                if !letter.is_alphabetic() {
                    add_word = false;
                    break;
                } else {
                    add_word = true;
                }
            }
            if add_word {
                self.index.push(word);
            }

        }
        self
    }

    /// Count the number of occurrences of the given `keyword`
    /// that are indexed by this `KWIndex`.
    pub fn count_matches(&self, keyword: &str) -> usize {
        let mut count: usize = 0;
        for &word in &self.index {
            if keyword == word {
                count += 1;
            }
        }
        count
    }

    /// Count the number of words that are indexed by this
    /// `KWIndex`.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Is this index empty?
    pub fn is_empty(&self) -> bool {
        match self.index.len() == 0 {
            true => true,
            false => false,
        }
    }

    pub fn display_stored_words(&self) {
        for &word in &self.index{
            println!("{}", word);
        }
    }
}
