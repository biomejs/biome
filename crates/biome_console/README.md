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
[cargo-badge]: https://badgen.net/crates/v/biome_console?&color=green
[cargo-url]: https://crates.io/crates/biome_console/

</div>

# `biome_console`

The crate contains a general abstraction over printing messages (formatted with markup) and diagnostics to a console.

## Usage example

The `Console` trait can be used to print two types of information to the user: messages (in the form of markup) and diagnostics:

```rust
console.message(markup! {
    <Info>"Processed "<Emphasis>{count}</Emphasis>" files"</Info>
});

console.diagnostic(
    &mut files,
    Diagnostics::error(file_id, code, title),
);
```

The following markup elements are supported:

- `Emphasis`: Print the content in bold text
- `Dim`: Print the content in dimmed text
- `Italic`: Print the content in italic text
- `Underline`: Print the content in underlined text
- `Error`: Set the text color to red
- `Success`: Set the text color to green
- `Warn`: Set the text color to yellow
- `Info`: Set the text color to blue

*Note*: Markup elements that change the "font" of the printed text (`Emphasis`, `Dim`, `Italic` and `Underline`) are not supported by the native Windows Console API and will instead get printed as ANSI control codes if the current terminal supports it, or will be ignored entirely
