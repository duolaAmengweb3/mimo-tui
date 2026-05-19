---
name: rust-style
description: When writing Rust, prefer Result + thiserror over panics, and use clippy-clean idioms.
triggers:
  - rust
  - .rs
  - cargo
---

When writing or editing Rust code in this workspace:

1. **Error handling**: prefer `Result<T, E>` over `panic!` / `unwrap()`. Use `thiserror` for crate-local errors, `anyhow::Result` at app boundaries.
2. **No unsafe** unless absolutely required, with a comment explaining why.
3. **Use `?`** for early return on `Result`. Don't `.unwrap()` outside of tests.
4. **Run `cargo fmt`** mentally before writing — 4-space indent, 100-col lines.
5. **`#[derive(Debug, Clone)]`** on every public type unless there's a reason not to.
6. **Module organization**: prefer `mod foo.rs` over `mod foo/mod.rs` for new modules.
7. **Public API**: every `pub` item must have a `///` doc comment with at least one sentence.
