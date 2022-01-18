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

            word = word.trim_end_matches(|c: char| c.is_ascii_punctuation());

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
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, kwindex.count_matches("b"));
    /// ```
    pub fn count_matches(&self, keyword: &str) -> usize {
        let mut count: usize = 0;
        for &word in &self.index {
            if keyword == word {
                count += 1;
            }
        }
        count
    }

    /// Return the *n*-th uppercase word (all characters are
    /// Unicode uppercase, *n*-th counting from 0) that is indexed
    /// by this `KWIndex`, if any.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("I am THE WALRUS");
    /// assert_eq!(Some("THE"), kwindex.nth_uppercase(1));
    /// ```
    pub fn nth_uppercase(&self, n: usize) -> Option<&str> {
        let mut uppercase_words: Vec<&str> = Vec::new();
        for &word in &self.index {
            match word.chars().next() {
                Some(letter) => {
                    if letter.is_ascii_uppercase() {
                        uppercase_words.push(word)
                    }
                }
                None => continue,
            }
        }

        match uppercase_words.get(n) {
            Some(res) => Some(res),
            None => None,
        }
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
        for &word in &self.index {
            println!("{}", word);
        }
    }
}
