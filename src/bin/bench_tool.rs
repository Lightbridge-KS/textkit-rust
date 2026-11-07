//! A second binary crate in the same package.
//!
//! Every file in `src/bin/` is its own crate root and becomes its own
//! executable, named after the file: `src/bin/bench_tool.rs` → `bench_tool`.
//! A package may have any number of binaries, but at most ONE library.
//!
//! Like `main.rs`, it reaches the library only through `textkit::...`.
//!
//! Run it (use --release for meaningful timings):
//!   cargo run --release --bin bench_tool

use std::time::Instant;

use textkit::word_count;

fn main() {
    let text = "the quick brown fox jumps over the lazy dog\n".repeat(200_000);

    let start = Instant::now();
    let n = word_count(&text);
    let elapsed = start.elapsed();

    println!("counted {n} words ({} bytes) in {elapsed:?}", text.len());
}
