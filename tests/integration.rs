//! Integration tests.
//!
//! Each file in `tests/` is compiled as its **own separate crate** that
//! depends on `textkit`. So, unlike the `#[cfg(test)] mod tests` blocks inside
//! `src/`, these tests can only use the PUBLIC API. That makes them a good
//! check of what real users of the library will experience.
//!
//! Run: `cargo test` (runs unit, integration and doc tests)
//!      `cargo test --test integration` (this file only)

use textkit::stats::TextStats;
use textkit::{report, word_count};

#[test]
fn reexport_and_full_path_name_the_same_function() {
    let text = "a b c";
    assert_eq!(word_count(text), textkit::stats::words::word_count(text));
}

#[test]
fn report_is_the_public_facade_over_private_render() {
    assert_eq!(report("hello rust\nmodules"), "== words: 3, lines: 2 ==");
}

#[test]
fn stats_are_read_through_getters() {
    let s = TextStats::from_text("tiny enormous");
    assert_eq!(s.words(), 2);
    assert_eq!(s.lines(), 1);
    assert_eq!(s.longest_word_len(), 8); // "enormous"
}
