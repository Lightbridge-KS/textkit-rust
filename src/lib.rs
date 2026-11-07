//! # textkit
//!
//! A tiny text-processing library that exists to show how Rust organizes
//! code into packages, crates and modules.
//!
//! This file, `src/lib.rs`, is the **crate root of the library crate**.
//! The compiler starts here and builds the module tree by following each
//! `mod` declaration. A `.rs` file that no `mod` declares is simply ignored.
//!
//! ```text
//! crate  (src/lib.rs)
//! ├── pub mod parse                        src/parse.rs
//! │   └── pub fn lines
//! ├── pub mod stats                        src/stats.rs
//! │   ├── pub struct TextStats             (fields private)
//! │   ├── pub fn summary
//! │   └── pub mod words                    src/stats/words.rs
//! │       ├── pub        fn word_count
//! │       ├── pub(super) fn tokens         (only `stats` can use it)
//! │       └──            fn is_word        (private)
//! ├──     mod render                       src/render.rs (private module)
//! │       └── pub(crate) fn banner
//! ├── pub fn report
//! └── pub use stats::words::word_count     (re-export / shortcut)
//! ```

// ── Module declarations ─────────────────────────────────────────────
// `mod x;` means: "module x exists, load its code from src/x.rs".
// Each module is declared exactly ONCE, in its parent.

pub mod parse; // public: other crates can write `textkit::parse::...`
pub mod stats; // public, and has a child module of its own (`stats::words`)

mod render; // private: usable inside this crate only

// ── Re-exports ──────────────────────────────────────────────────────
// `pub use` publishes an item under a shorter path. Users can now write
// `textkit::word_count` instead of `textkit::stats::words::word_count`.
// Both paths name the same function.
pub use stats::words::word_count;

// ── Public API at the crate root ────────────────────────────────────

/// Build a one-line report for `text`.
///
/// This public function is a facade over the private `render` module:
/// callers get the result without knowing (or depending on) how it is built.
///
/// The example below is a *doc test*. `cargo test` compiles it as its own
/// small crate, so it must use the public path `textkit::...`, just like
/// any outside user would.
///
/// ```
/// let out = textkit::report("hello rust\nmodules");
/// assert_eq!(out, "== words: 3, lines: 2 ==");
/// ```
pub fn report(text: &str) -> String {
    render::banner(text)
}
