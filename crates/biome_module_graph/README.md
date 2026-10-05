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
[cargo-badge]: https://badgen.net/crates/v/biome_module_graph?&color=green
[cargo-url]: https://crates.io/crates/biome_module_graph/

</div>

# Biome module graph

See [`CONTRIBUTING.md`](CONTRIBUTING.md) before adding type-inference requests or queries.

In Biome, a module is conventionally a file. A module a is a known file that imports and exports data, which is used
inside a project.

Modules import and export data based on their source languages. For example, JavaScript explicitely exports data
via `import` and `exports`, CSS "exports" everything declared in a file and imports files via `@import`.

The module graph computes all the required information based on the language that support.

## Under the hood

[salsa](https://github.com/salsa-rs/salsa) powers the module graph. Salsa is a **framework** to incrementally compute information.
Particularly, salsa is able to track information inside a tree of "tracked", and return the memoized result in case the "inputs" don't change.

Salsa comes with its own runtime, which means memory and performance might be slower for a "cold" run, however it fits very well
for cases the same information is requested multiple times e.g. code editors.

## Queries

The module graph must be consulted, if possible, with tracked functions. Tracked functions are tagged with `#[salsa::tracked]`, and their
parameters must be salsa constructs. This allows the framework to return memoized information and skip the computation.

Acceptable salsa constructs:
- types that are tagged with `#[salsa::tracked]`
- types that are tagged with `#[salsa::interned]`
- types that are tagged with `#[salsa::input]`
- the salsa database


