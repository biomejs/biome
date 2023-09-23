# `biome_diagnostics`

This crate contains the types and utility functions used to implement errors and diagnostics in the Biome codebase.

## Acknowledgement

This crate was initially forked from [rslint_errors](https://github.com/rslint/rslint/tree/master/crates/rslint_errors). The design of the new `Diagnostic` trait, `Error` struct, `Context` trait, and the `Diagnostic` derive macro in `biome_diagnostics_macros` are inspired by various fantastic crates in the Rust error handling space: [miette](https://github.com/zkat/miette), [anyhow](https://github.com/dtolnay/anyhow) and [thiserror](https://github.com/dtolnay/thiserror)
