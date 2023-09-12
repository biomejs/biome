## Internal installation

```toml
biome_js_formatter = { version = "0.0.1", path = "../biome_js_formatter" }
```

## Usage

The foundation of the formatter relies on two pillars:

- the usage of the [*trait*](https://doc.rust-lang.org/reference/items/traits.html) generic `Format` trait and `FormatNode` for nodes.
- the creation of an intermediate IR via a series of helpers

Import the `FormatNode` trait and implement it for your Node.

```rust
use biome_js_formatter::prelude::*;
use biome_formatter::{write, format_args};

struct Buzz {
 blast: String
}

impl Format for Buzz {
 fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
 	// implementation goes here
	 write!(f, [token("Hello"), dynamic_token(&self.blast)])
 }
}

```

## Rules when formatting AST nodes

1. if a token is mandatory and the AST has that information, please use that token instead, for example:

	 ```rust
	 fn fmt_fields(node: Node, f: &mut JsFormatter) -> FormatResult<()> {
		 write!(f, [node.l_paren_token().format()])?; // yes
		 write!(f, [token("(")])?; // no
	 }
	 ```

2. for tokens that are not mandatory, use our helpers
3. do not attempt to "fix" the code. If you know a token/node is mandatory, return `None` instead

## Debugging formatter output

You can use the `dbg_write!` macro to output the written IR elements to the console (similar to how the `dbg!` macro works).

```rust
dbg_write!(f, [
	token("hello"),
	space_token(),
	token("world")
])?;

// Writes
// [src/main.rs:1][0] = StaticToken("hello")
// [src/main.rs:1][1] = Space
// [src/main.rs:1][0] = StaticToken("world")
```
