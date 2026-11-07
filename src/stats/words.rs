//! `stats::words`: word-level logic.
//!
//! Declared in `src/stats.rs` with `pub mod words;`
//! Because its parent is `stats`, its file lives in the `src/stats/` folder.
//!
//! Full path from the crate root: `crate::stats::words`
//!
//! This module shows three levels of visibility side by side:
//!
//! ```text
//! pub        fn word_count   → anyone who can reach this module
//! pub(super) fn tokens       → the parent module (`stats`) only
//!            fn is_word      → this module and its children only
//! ```

/// Count the "real" words in `text`: tokens with at least one letter or digit.
/// Punctuation-only tokens such as `--` or `!!` are not counted.
pub fn word_count(text: &str) -> usize {
    tokens(text).count()
}

/// Iterate over the word tokens of `text`.
///
/// `pub(super)`: visible to the parent module `stats` (which uses it to find
/// the longest word), but NOT to `render`, `main.rs` or any other crate.
pub(super) fn tokens(text: &str) -> impl Iterator<Item = &str> {
    text.split_whitespace().filter(|w| is_word(w))
}

/// Private helper: no `pub`, so only this module (and its child `tests`)
/// can call it. Even the parent `stats` cannot.
fn is_word(token: &str) -> bool {
    token.chars().any(char::is_alphanumeric)
}

#[cfg(test)]
mod tests {
    use super::*; // brings `word_count`, `tokens` AND private `is_word` into scope

    #[test]
    fn child_tests_can_call_private_helper() {
        assert!(is_word("abc"));
        assert!(!is_word("---"));
    }

    #[test]
    fn counts_only_real_words() {
        assert_eq!(word_count("hello -- world !!"), 2);
    }

    #[test]
    fn tokens_skip_punctuation() {
        let t: Vec<&str> = tokens("a -- b").collect();
        assert_eq!(t, vec!["a", "b"]);
    }
}
