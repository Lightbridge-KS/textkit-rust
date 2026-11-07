//! `stats`: counting things in text.
//!
//! Declared in `src/lib.rs` with `pub mod stats;`
//!
//! File layout for a module WITH children (modern style, Rust 2018+):
//!
//! ```text
//! src/stats.rs          ← this file: the `stats` module itself
//! src/stats/words.rs    ← its child: `stats::words`
//! ```
//!
//! (The older alternative is `src/stats/mod.rs`. Either works, never both.)

pub mod words; // declares the child module → loads src/stats/words.rs

use crate::parse; // ABSOLUTE path: start at the crate root, then go down

/// Summary statistics for a piece of text.
///
/// The struct is `pub`, but its **fields are private**. Code outside this
/// module (including `main.rs` and other crates) cannot write
/// `TextStats { words: 1, .. }` or read `stats.words` directly. It has to go
/// through [`TextStats::from_text`] and the getter methods. That is how Rust
/// enforces encapsulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextStats {
    words: usize,
    lines: usize,
    longest_word_len: usize,
}

impl TextStats {
    /// The only way to create a `TextStats` from outside this module.
    pub fn from_text(text: &str) -> Self {
        Self {
            // RELATIVE path: `words` is a child of the current module.
            words: words::word_count(text),
            lines: parse::lines(text).len(),
            // `tokens` is `pub(super)` in `words`. We are its parent (`super`),
            // so we may call it. Nobody else can.
            longest_word_len: words::tokens(text)
                .map(|w| w.chars().count())
                .max()
                .unwrap_or(0),
        }
    }

    pub fn words(&self) -> usize {
        self.words
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn longest_word_len(&self) -> usize {
        self.longest_word_len
    }
}

/// A short human-readable summary, e.g. `"words: 3, lines: 2"`.
pub fn summary(text: &str) -> String {
    // `self::` is optional here; it just makes "relative to this module"
    // explicit. `TextStats::from_text(text)` would mean the same thing.
    let s = self::TextStats::from_text(text);
    format!("words: {}, lines: {}", s.words(), s.lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_can_read_private_fields() {
        // Allowed: `tests` is a descendant of `stats`, so it sees private fields.
        let s = TextStats::from_text("hello rust\nmodules");
        assert_eq!(s.words, 3);
        assert_eq!(s.lines, 2);
        assert_eq!(s.longest_word_len, 7); // "modules"
    }

    #[test]
    fn summary_format() {
        assert_eq!(summary("one two\nthree"), "words: 3, lines: 2");
    }
}
