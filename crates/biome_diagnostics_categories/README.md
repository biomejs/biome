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
[cargo-badge]: https://badgen.net/crates/v/biome_diagnostics_categories?&color=green
[cargo-url]: https://crates.io/crates/biome_diagnostics_categories/

</div>

# `biome_diagnostics_categories`

This crate contains a static registry of all the diagnostic categories used
throughout the Biome codebase

## Code Generation

The list of categories is defined in `src/categories.rs` using the
`define_dategories!` macro, but instead of relying on conventional Rust macro
expansion this crate instead uses a build script (in `build.rs`) to control how
the code resulting from the macro is generated.

Specifically this lets us generate new identifiers, which is something plain
Rust macros cannot do, without having to use full-blown procedural macros,
which would require creating and building yet another crate.
