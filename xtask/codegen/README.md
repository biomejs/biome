# codegen

This crate contains local commands used to auto-generate source code.

## `cargo codegen grammar`
This command transforms the `js.ungram` file into the `rome_js_syntax` crate.

The project uses [`ungrammar`](https://github.com/rust-analyzer/ungrammar) to define the syntax of the language.

`ungrammar` uses a DSL to define and parse the grammar of a language.

Once the library parses the DSL files, some custom logic generates the AST APIs.

### Conventions to create syntax

Here's a list of internal conventions that we follow to write grammar:

1. use the prefix `manual__` to mark methods that are implemented manually;

	```text
	MyDeclaration = manual__decl:Body
	```

	If you define like this, it means that somewhere in your code you will create something like:

	```rust
	impl MyDeclaration {
		fn decl(self) -> Option<Body> {
			// custom logic goes here
		}
	}
	```

1. unions of tokens have to have a label. In this way, the code generation can handle it properly and generate a better AST.

	```text
	BinExpr = left: Expr op: ('+' | '-' | '*') right: Expr
	```

	This will generate the correct implantation:

	```rust
	impl BinExpr {
		fn op(self) -> Option<SyntaxToken> {
			// custom logic goes here
			support::find_token(
			&self.syntax,
			&[
				T![+],
				T![-],
				T![*],
			],
		)
		}
	}
	```

1. Nodes used to track broken code should contain the **Bogus** word in its name (case sensitive).
This is needed because it will generate a different type of code, useful in case of errors inside the source code Biome will parse.

## `cargo codegen test`
This command extracts inline comment tests inside `rome_js_parser` into the directory `rome_js_parser/test_data/`.

A usual workflow would be:
```bash
# (modify inline comment tests inside the parser)
cargo codegen test
cargo test parser # for checking failed tests
UPDATE_EXPECT=1 cargo test parser # for committing the changes
```

## `cargo codegen unicode`
This command downloads unicode data from unicode.org and writes it `crates/rome_js_lexer/src/tables.rs`.
Use this command when unicode support has changed.
