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
[cargo-badge]: https://badgen.net/crates/v/biome_markup?&color=green
[cargo-url]: https://crates.io/crates/biome_markup/

</div>

# `biome_markup`

The crate contains procedural macros to build `biome_console` markup object with a JSX-like syntax

The macro cannot be used alone as it generates code that requires supporting types declared in the
`biome_console` crate, so it's re-exported from there and should be used as `biome_console::markup`
