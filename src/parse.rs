//! `parse`: splitting text into pieces.
//!
//! Declared in `src/lib.rs` with `pub mod parse;`
//! Full path from the crate root: `crate::parse`
//! Path for other crates:         `textkit::parse`

/// Split `text` into its lines.
pub fn lines(text: &str) -> Vec<&str> {
    text.lines().collect()
}

// ── Unit tests ──────────────────────────────────────────────────────
// `tests` is a CHILD module of `parse`, so it can see everything here,
// including private items. `#[cfg(test)]` compiles it only for `cargo test`.
#[cfg(test)]
mod tests {
    use super::*; // `super` = the parent module, i.e. `parse`

    #[test]
    fn splits_on_newlines() {
        assert_eq!(lines("a\nb\nc"), vec!["a", "b", "c"]);
    }

    #[test]
    fn empty_text_has_no_lines() {
        assert!(lines("").is_empty());
    }
}
