# textkit: Rust packages, crates & modules (lib + main pattern)

A tiny, fully working Rust package whose real purpose is to show **how Rust
code is organized**. The "text statistics" logic is intentionally trivial so
that the structure is the focus. Every file is commented to explain its role.

Requires Rust **1.85+** (edition 2024). Check with `rustc --version`;
update with `rustup update`.

## Quick start

```bash
cargo run                                  # main binary, default sentence
cargo run -- "count the words in this"     # args after `--` go to your program
cargo run --release --bin bench_tool       # the second binary
cargo run --example demo                   # an example program
cargo test                                 # unit + integration + doc tests
cargo doc --open                           # browse the public API as users see it
```

## 1. Files on disk → crates

```text
textkit/                         PACKAGE  (Cargo.toml)
├── Cargo.toml
├── src/
│   ├── lib.rs                   ★ crate root: LIBRARY crate `textkit`
│   ├── parse.rs                   module  crate::parse
│   ├── stats.rs                   module  crate::stats
│   ├── stats/
│   │   └── words.rs               module  crate::stats::words
│   ├── render.rs                  module  crate::render (private)
│   ├── main.rs                  ★ crate root: BINARY crate → `textkit` executable
│   └── bin/
│       └── bench_tool.rs        ★ crate root: BINARY crate → `bench_tool` executable
├── tests/
│   └── integration.rs           ★ crate root: test crate (public API only)
└── examples/
    └── demo.rs                  ★ crate root: example crate (public API only)
```

★ = a separate crate. The package holds **one library** and **several other
crates that all depend on it**:

```text
   main.rs   bench_tool.rs   tests/integration.rs   examples/demo.rs   doc tests
      │            │                  │                     │               │
      └────────────┴──────────┬───────┴─────────────────────┴───────────────┘
                              │  `use textkit::...`  (PUBLIC API only)
                              ▼
                 ┌─────────────────────────┐
                 │ library crate `textkit` │   src/lib.rs + its modules
                 └─────────────────────────┘
```

## 2. The module tree inside the library crate

The tree is built by `mod` declarations, **not** by the folder layout.
A `.rs` file that no `mod` declares is ignored by the compiler.

```text
crate  (src/lib.rs)
├── pub mod parse                        ← `pub mod parse;`  in lib.rs
│   └── pub fn lines
├── pub mod stats                        ← `pub mod stats;`  in lib.rs
│   ├── pub struct TextStats             (fields private; use getters)
│   ├── pub fn summary
│   └── pub mod words                    ← `pub mod words;`  in stats.rs
│       ├── pub        fn word_count
│       ├── pub(super) fn tokens         (only `stats` may call it)
│       └──            fn is_word        (private)
├──     mod render                       ← `mod render;`     in lib.rs (private)
│       └── pub(crate) fn banner
├── pub fn report                        (public facade over `render`)
└── pub use stats::words::word_count     (re-export: `textkit::word_count`)
```

## 3. Where to look for each concept

| Concept | Where |
|---|---|
| Declaring modules with `mod` | `src/lib.rs`, `src/stats.rs` |
| Modern file layout for a module with children (`stats.rs` + `stats/`) | `src/stats.rs` |
| Absolute paths (`crate::`) | `src/stats.rs`, `src/render.rs` |
| Relative paths (`self::`, child name) | `src/stats.rs` |
| `super::` | every `#[cfg(test)] mod tests` block |
| `pub` / `pub(crate)` / `pub(super)` / private | `src/stats/words.rs`, `src/render.rs` |
| Private struct fields + constructor + getters | `src/stats.rs` (`TextStats`) |
| Re-export with `pub use` | `src/lib.rs` |
| Facade over a private module | `src/lib.rs` (`report` → `render::banner`) |
| `main.rs` is a different crate (`textkit::` not `crate::`) | `src/main.rs` |
| Multiple binaries + `default-run` | `src/bin/bench_tool.rs`, `Cargo.toml` |
| Unit tests that can see private items | `src/stats/words.rs`, `src/stats.rs` |
| Integration tests that see only the public API | `tests/integration.rs` |
| Doc tests (also a separate crate) | `src/lib.rs` (`report` doc comment) |

## 4. Exercises

1. **Privacy errors.** At the bottom of `src/main.rs`, uncomment one line at a
   time and run `cargo build`. Each one fails with a different error
   (E0603, E0451, E0616, E0433). Read the message, then comment it back.
2. **"Orphan" file.** Create `src/unused.rs` with `pub fn hi() {}` and try
   `textkit::unused::hi()` from `main.rs`. It fails. Then add `pub mod unused;`
   to `lib.rs` and try again.
3. **`use` is not `mod`.** In `lib.rs`, change `pub mod parse;` to
   `pub use parse;`. Observe the error: `use` only creates a shortcut to
   something that already exists in the tree; it never loads a file.
4. **Old layout.** Rename `src/stats.rs` → `src/stats/mod.rs`. It still
   compiles. Then copy it back to `src/stats.rs` as well and see the
   "file for module found at both" error.
5. **Visibility levels.** Change `tokens` in `words.rs` from `pub(super)` to
   private (remove `pub(super)`). Which module breaks, and why?
6. **Re-export.** Remove `pub use stats::words::word_count;` from `lib.rs`.
   Which crates break? Fix them by using the full path instead.

Running `cargo test` lists unit tests by their module path
(e.g. `stats::words::tests::counts_only_real_words`), another way to see the
module tree.
