//! Binary crate root for the `textkit` executable.
//!
//! IMPORTANT: `src/main.rs` and `src/lib.rs` are **two separate crates** in
//! the same package. This file uses the library exactly like an outside user
//! would: through the crate name `textkit`, and only through its PUBLIC API.
//!
//! The lib + main pattern:
//!
//! ```text
//! main.rs (binary crate)   thin: read input → call library → print
//!        │
//!        │  depends on the public API only
//!        ▼
//! lib.rs  (library crate)  all real logic: testable, reusable
//! ```
//!
//! Run it:
//!   cargo run                         (uses a default sentence)
//!   cargo run -- "some text to count" (args after `--` go to the program)

use std::env;

use textkit::stats::TextStats; // full public path
use textkit::{report, word_count}; // `word_count` is the re-exported shortcut

fn main() {
    // 1. Gather input (the only "app" concern that lives here).
    let args: Vec<String> = env::args().skip(1).collect();
    let text = if args.is_empty() {
        String::from("Hello Rust modules")
    } else {
        args.join(" ")
    };

    // 2. Call the library.
    let banner = report(&text);
    let count = word_count(&text);
    let stats = TextStats::from_text(&text);

    // 3. Present the results.
    println!("{banner}");
    println!("word_count (re-export)  : {count}");
    println!("longest word (getter)   : {} chars", stats.longest_word_len());

    // ── Try it: uncomment ONE line at a time and run `cargo build` ──────
    // Each line below is a compile error. Reading the messages is a fast way
    // to learn the privacy rules.
    //
    // textkit::render::banner(&text);              // E0603: module `render` is private
    // textkit::stats::words::tokens(&text);        // E0603: function `tokens` is private (pub(super))
    // let s = TextStats { words: 1, lines: 1, longest_word_len: 1 }; // E0451: private fields
    // println!("{}", stats.words);                 // E0616: field `words` is private
    // crate::parse::lines(&text);                  // E0433: `crate` here means THIS binary crate,
    //                                              //        which has no `parse` module
}
