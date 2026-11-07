//! `render`: formatting output. This is a **private** module.
//!
//! Declared in `src/lib.rs` with `mod render;` (no `pub`).
//!
//! Who can use it?
//!
//! ```text
//! inside the library crate (lib.rs, stats, parse, ...)  ✅  every module is a
//!                                                           descendant of the root,
//!                                                           where `render` lives
//! src/main.rs, tests/, examples/, other crates          ❌  `render` is invisible
//! ```

use crate::stats; // absolute path to a sibling module

/// Wrap the stats summary in a banner, e.g. `"== words: 3, lines: 1 =="`.
///
/// `pub(crate)` = usable anywhere inside this crate, never outside it
/// (like `internal` in C#). Since the `render` module itself is already
/// private, plain `pub` would behave the same here; `pub(crate)` just states
/// the intent explicitly and stays correct if `render` is made `pub` later.
pub(crate) fn banner(text: &str) -> String {
    format!("== {} ==", stats::summary(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_summary() {
        assert_eq!(banner("hi"), "== words: 1, lines: 1 ==");
    }
}
