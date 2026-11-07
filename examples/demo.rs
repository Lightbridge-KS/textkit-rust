//! Example program: also its own crate, using only the public API.
//!
//! Files in `examples/` are not installed or shipped as executables. They exist
//! to show users how to call the library.
//!
//! Run: `cargo run --example demo`

use textkit::parse;
use textkit::stats::TextStats;

fn main() {
    let text = "Rust organizes code into crates\nand crates into modules\n-- like this!";

    // Module path from another crate: textkit → parse → lines
    for (i, line) in parse::lines(text).iter().enumerate() {
        let s = TextStats::from_text(line);
        println!(
            "line {}: {:<34} words={} longest={}",
            i + 1,
            format!("{line:?}"),
            s.words(),
            s.longest_word_len()
        );
    }

    println!("\n{}", textkit::report(text));
}
