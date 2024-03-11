# codegen

This crate contains local commands used to auto-generate source code.

## `cargo codegen grammar`
This command transforms the `*.ungram` files into the `biome_*_syntax` and `biome_*_factory` crates.

The project a fork of [`ungrammar`](https://github.com/rust-analyzer/ungrammar) to define the syntax of the language.

`ungrammar` uses a DSL to define and parse the grammar of a language.

Once the library parses the DSL files, some custom logic generates the AST APIs.

## Create a new language

Let's say you want to create a new language that has the extension `html`, you'll have to follow this instructions:

1. Create a new `html.ungram` file in this very folder.
   Add this legend to the `.ungram` file

  ```
  // This grammar specifies the structure of Rust's concrete syntax tree.
  // It does not specify parsing rules (ambiguities, precedence, etc are out of scope).
  // Tokens are processed -- contextual keywords are recognised, compound operators glued.
  //
  // Legend:
  //
  //   //          				-- comment
  //   Name =      				-- non-terminal definition
  //   'ident'     				-- token (terminal)
  //   A B         				-- sequence
  //   A | B       				-- alternation
  //   A*          				-- zero or more repetition
  //   (A (',' A)* ','?)	        -- repetition of node A separated by ',' and allowing a trailing comma
  //   (A (',' A)*)	            -- repetition of node A separated by ',' without a trailing comma
  //   A?          				-- zero or one repetition
  //   (A)         				-- same as A
  //   label:A     				-- suggested name for field of AST node
  ```
1. Create a new file called `src/html_kinds_src.rs`. This file must return a static `KindSrc`.
1. Create two new creates: `biome_html_syntax` and `biome_html_factory`. Use `cargo new --lib crates/biome_html_syntax`.
1. Create a `generated/` folder inside the `src/` folder of the newly created crates.
1. Add a new variant to `LanguageKind`, inside `language_kind.rs` file. The new variant will be `Html`. You'll have to implement
  all methods and cover the new variant.
1. Add a new prefix `html_` to `LANGUAGE_PREFIXES` inside `language_kind.rs`.
1. Once you covered all variants, run the command `cargo codegen grammar`.


## Conventions when writing a new grammar in Biome

- All nodes **must** start with the prefix of the language, e.g. `HtmlSimpleAttribute`.
- Unions of nodes **must** start with `Any*`, e.g. `AnyHtmlAttribute`.
- Nodes that are used for enclosing syntax errors must have the **Bogus** word, e.g. `HtmlBogusAttribute`.
- **Bogus** nodes **must be part of a variant**, e.g. 
  ```
  AnyHtmlAttribute = 
    HtmlSimpleAttribute
    HtmlBogusAttribute
  ```
- Nodes that represent a list **must** end with the postfix **List**, e.g. `HtmlAttributeList`.
- Lists are **never** optional. They are mandatory and empty by default, e.g.
  ```
  HtmlTag = 
    attributes: HtmlAttributeList
  ```

## `cargo codegen test`
This command extracts inline comment tests inside `biome_js_parser` into the directory `biome_js_parser/test_data/`.

A usual workflow would be:
```bash
# (modify inline comment tests inside the parser)
cargo codegen test
cargo test parser # for checking failed tests
UPDATE_EXPECT=1 cargo test parser # for committing the changes
```

## `cargo codegen unicode`
This command downloads unicode data from unicode.org and writes it `crates/biome_js_lexer/src/tables.rs`.
Use this command when unicode support has changed.
