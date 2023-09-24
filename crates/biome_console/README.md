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
