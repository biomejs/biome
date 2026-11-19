<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Biome, with its logo and the phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="400">
  </picture>
</p>

<div align="center">

[![Discord chat][discord-badge]][discord-url]
[![cargo version][cargo-badge]][cargo-url]

[discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=green
[discord-url]: https://biomejs.dev/chat
[cargo-badge]: https://badgen.net/crates/v/biome_diagnostics?&color=green
[cargo-url]: https://crates.io/crates/biome_diagnostics/

</div>

# `biome_diagnostics`

This crate contains the types and utility functions used to implement errors and diagnostics in the Biome codebase.

## Acknowledgement

This crate was initially forked from [rslint_errors](https://github.com/rslint/rslint/tree/master/crates/rslint_errors). The design of the new `Diagnostic` trait, `Error` struct, `Context` trait, and the `Diagnostic` derive macro in `biome_diagnostics_macros` are inspired by various fantastic crates in the Rust error handling space: [miette](https://github.com/zkat/miette), [anyhow](https://github.com/dtolnay/anyhow) and [thiserror](https://github.com/dtolnay/thiserror)
