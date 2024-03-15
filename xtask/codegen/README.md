# codegen

This crate contains local commands used to auto-generate source code.

## `cargo codegen grammar`
This command transforms the `*.ungram` files into the `biome_*_syntax` and `biome_*_factory` crates.

The project uses a fork of [`ungrammar`](https://github.com/rust-analyzer/ungrammar) to define the syntax of the language.

`ungrammar` uses a DSL to define and parse the grammar of a language.

Once the library parses the DSL files, some custom logic generates the AST APIs.

## Create a new language


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
