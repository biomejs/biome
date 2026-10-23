<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Biome, with its logo and the phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="400">
  </picture>
</p>

<div align="center">

[![Discord chat][discord-badge]][discord-url]
[![CI][ci-badge]][ci-url]
[![cargo version][cargo-badge]][cargo-url]

[discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=green
[discord-url]: https://biomejs.dev/chat
[ci-badge]: https://github.com/biomejs/biome/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/biomejs/biome/actions/workflows/main.yml
[cargo-badge]: https://badgen.net/crates/v/biome_formatter?&color=green
[cargo-url]: https://crates.io/crates/biome_formatter/

</div>

# `biome_formatter`

`biome_formatter` is the formatting engine of [Biome](https://biomejs.dev). It decides where to break lines and how to indent code, so that the result fits within a line width. You describe the layout of your code, and the engine prints it.

The engine doesn't depend on a parser. You can use it with the syntax tree of any parser, including parsers that describe locations with lines and columns instead of byte offsets.

## How it works

Formatting happens in two phases:

1. Your code builds a **document**. A document is a list of instructions, such as "write this text", "start a group", or "break the line here if the group doesn't fit". You build it from your syntax tree.
2. The **printer** reads the document and writes the formatted code. It tries to print each group on a single line. When a group doesn't fit within the line width, the printer breaks the lines of that group.

## Installation

Biome's own formatters use the default features of the crate. To use the engine with another parser, turn the default features off:

```toml
[dependencies]
biome_formatter = { version = "0.7", default-features = false }
```

Without the default features, the crate depends only on `biome_string_case` and `biome_text_size` from Biome.

### Features

| Feature       | Enabled by default | What it adds                                                                      |
|---------------|--------------------|-----------------------------------------------------------------------------------|
| `rowan`       | Yes                | Formatting of syntax trees built with `biome_rowan`, Biome's syntax tree library. |
| `diagnostics` | Yes                | Biome diagnostics for formatter errors, and console output for formatter options. |
| `deserialize` | Yes                | Reading formatter options from Biome's configuration file.                        |
| `serde`       | No                 | Serialization of formatter options and of the formatted result.                   |
| `schema`      | No                 | JSON Schema for formatter options. Also enables `serde`.                          |

## Build your first document

The following example formats a list. With a line width of 80 characters, the list fits on one line. With a line width of 5 characters, the printer breaks it into one item per line:

```rust
use biome_formatter::prelude::*;
use biome_formatter::{LineWidth, SimpleFormatOptions, format, format_args, write};

fn main() -> FormatResult<()> {
    let list = format_with(|f| {
        write!(
            f,
            [group(&format_args![
                token("["),
                soft_block_indent(&format_args![
                    token("1,"),
                    soft_line_break_or_space(),
                    token("2,"),
                    soft_line_break_or_space(),
                    token("3"),
                ]),
                token("]"),
            ])]
        )
    });

    let formatted = format!(SimpleFormatContext::default(), [list])?;
    assert_eq!(formatted.print()?.as_code(), "[1, 2, 3]");

    let narrow = SimpleFormatContext::new(SimpleFormatOptions {
        line_width: LineWidth::try_from(5).unwrap(),
        ..SimpleFormatOptions::default()
    });
    let formatted = format!(narrow, [list])?;
    assert_eq!(formatted.print()?.as_code(), "[\n\t1,\n\t2,\n\t3\n]");

    Ok(())
}
```

`group` marks content that the printer keeps on one line when it can. `soft_line_break_or_space` prints a space when its group fits, and a line break when the group breaks. `soft_block_indent` indents its content when the group breaks. The [crate documentation](https://docs.rs/biome_formatter/) lists all the building blocks.

The default options indent with tabs and use a line width of 80 characters.

## Format your syntax tree

Implement `Format` for the nodes of your syntax tree. Each node writes its part of the document:

```rust
use biome_formatter::prelude::*;
use biome_formatter::{format, format_args, write};

/// A function call from your syntax tree, such as `print(a, b)`.
struct Call {
    name: String,
    arguments: Vec<String>,
}

impl Format<SimpleFormatContext> for Call {
    fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
        let arguments = format_with(|f| {
            f.join_with(&format_args![token(","), soft_line_break_or_space()])
                .entries(self.arguments.iter().map(|argument| text(argument, None)))
                .finish()
        });

        write!(
            f,
            [
                text(&self.name, None),
                group(&format_args![token("("), soft_block_indent(&arguments), token(")")]),
            ]
        )
    }
}

fn main() -> FormatResult<()> {
    let call = Call {
        name: "print".to_string(),
        arguments: vec!["a".to_string(), "b".to_string()],
    };

    let formatted = format!(SimpleFormatContext::default(), [call])?;
    assert_eq!(formatted.print()?.as_code(), "print(a, b)");

    Ok(())
}
```

`token` writes text that never changes, such as punctuation. `text` writes text from your source code, such as a name. Its second argument is the location of the text in your source code. Pass `None` unless you map the formatted code back to your source code, as described in [Map the formatted code back to your source code](#map-the-formatted-code-back-to-your-source-code).

`SimpleFormatContext` only holds the options that the printer needs. When your nodes need more information while they format, such as the comments of the file, implement `FormatContext` for your own context type.

## Format code written in another language

Some languages contain code written in another language, such as JavaScript inside an HTML `<script>` tag. You can format that code with the formatter of its own language:

1. In your formatter, format the code yourself and wrap the result in `FormatEmbedded`. Give it a key, so that you can find the code again later.
2. After formatting, call `Formatted::format_embedded`. It calls your function with the key of each piece of embedded code. Return the document of the other formatter to replace your output, or `None` to keep it.

```rust
use biome_formatter::prelude::*;
use biome_formatter::{format, write};

fn main() -> FormatResult<()> {
    let key = 1;
    let paragraph = format_with(|f| {
        let embedded = FormatEmbedded::new(key, &token("a+b"), f)?;
        write!(f, [token("<p>"), embedded, token("</p>")])
    });

    let replacement = format!(SimpleFormatContext::default(), [token("a + b")])?.into_document();
    let mut formatted = format!(SimpleFormatContext::default(), [paragraph])?;
    formatted.format_embedded(|embedded_key| (embedded_key == key).then(|| replacement.clone()));

    assert_eq!(formatted.print()?.as_code(), "<p>a + b</p>");
    Ok(())
}
```

When you don't replace a piece of embedded code, it keeps the output of your formatter. The file is still formatted when the other formatter fails.

## Map the formatted code back to your source code

This section is for tools that format only part of a file. To replace only the selected code, such a tool needs to know which part of the formatted code comes from the selection.

The printer can record where each piece of the formatted code comes from in your source code. These records form a **source map**, which links positions in the formatted code to positions in your source code.

### Describe your locations

You decide what a location looks like by implementing the `Source` trait. The formatter never reads your locations. It stores them, and it gives them back to you in the formatted result.

`SimpleFormatContext::default()` uses `PlainSource`, a source from this crate whose locations are byte offsets. The following example describes a source code whose locations are lines and columns instead:

```rust
use biome_formatter::{NoSourceMap, Source};
use std::rc::Rc;

/// A location as a line and a column, both starting at 0.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct LineColumn {
    pub line: u32,
    pub column: u32,
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct LineColumnSource;

impl Source for LineColumnSource {
    type Position = LineColumn;
    type Span = (LineColumn, LineColumn);
    type Text = Rc<str>;
    type EmbeddedKey = u32;
    type SourceMap = NoSourceMap;

    fn advance(position: LineColumn, text: &str) -> LineColumn {
        let mut position = position;
        for character in text.chars() {
            if character == '\n' {
                position.line += 1;
                position.column = 0;
            } else {
                position.column += 1;
            }
        }
        position
    }

    fn span(start: LineColumn, end: LineColumn) -> Self::Span {
        (start, end)
    }
}
```

Each part of the trait has one job:

- `Position` is a location in your source code. Its default value must be the location of the first character.
- `Span` is a range of your source code.
- `Text` is text from your source code. `PlainSource` uses `Rc<str>`. A syntax tree that already stores shared text can use its own type to avoid copying it.
- `EmbeddedKey` identifies a piece of code written in another language, as described in [Format code written in another language](#format-code-written-in-another-language).
- `SourceMap` maps locations back to your original source code when you change your syntax tree before formatting it. For example, Biome's JavaScript formatter removes parentheses from its tree before formatting it. Use `NoSourceMap` when you don't change your tree.
- `advance` returns the location that comes after a piece of text. The printer uses it to compute the location of each piece of the output.
- `span` returns the range between two locations.

### Record locations while formatting

Give `text` the location of your source text. Then print with `Formatted::print_with_indent` and `SourceMapGeneration::Enabled`. `Formatted::print` doesn't build a source map:

```rust
use biome_formatter::prelude::*;
use biome_formatter::{
    SimpleFormatOptions, SourceMapGeneration, SourceMarker, format, format_args, write,
};

/// A function call from your syntax tree, such as `print(a, b)`.
struct Call {
    name: String,
    name_position: LineColumn,
    arguments: Vec<String>,
}

type Context = SimpleFormatContext<LineColumnSource>;

impl Format<Context> for Call {
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let arguments = format_with(|f| {
            f.join_with(&format_args![token(","), soft_line_break_or_space()])
                .entries(self.arguments.iter().map(|argument| text(argument, None)))
                .finish()
        });

        write!(
            f,
            [
                text(&self.name, Some(self.name_position)),
                group(&format_args![token("("), soft_block_indent(&arguments), token(")")]),
            ]
        )
    }
}

fn main() -> FormatResult<()> {
    let call = Call {
        name: "print".to_string(),
        name_position: LineColumn { line: 3, column: 4 },
        arguments: vec!["a".to_string(), "b".to_string()],
    };

    let context = Context::from_options(SimpleFormatOptions::default());
    let formatted = format!(context, [call])?;
    let printed = formatted.print_with_indent(0, SourceMapGeneration::Enabled)?;

    assert_eq!(printed.as_code(), "print(a, b)");
    assert_eq!(
        printed.sourcemap()[0],
        SourceMarker {
            source: LineColumn { line: 3, column: 4 },
            dest: LineColumn { line: 0, column: 0 },
        }
    );

    Ok(())
}
```

The source map reports both positions with your `LineColumn` type. `source` is the location in your source code, and `dest` is the location in the formatted code.

## Learn more

- The [crate documentation](https://docs.rs/biome_formatter/) describes every type and building block.
- The [contribution guide](./CONTRIBUTING.md) explains how to add a formatter for a new language to Biome.
