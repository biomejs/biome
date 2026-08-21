---
name: syntax-text-handling
description: Use this skill when extracting or storing Biome AST/CST source text, working with `SyntaxToken`, `TokenText`, quoted strings, token-relative ranges, or embedded-language attribute values, or diagnosing trivia, allocation, range, and temporary-borrow problems. Do not use for parser implementation or formatter comment placement.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Syntax Text Handling

Use typed syntax APIs and preserve source ownership instead of converting syntax text into strings.

## Inspect Before Implementing

Do not infer an AST shape from source spelling. Inspect the parser quick test for the language and search generated node definitions and existing consumers. Load `testing-codegen` for the current quick-test command.

Check every relevant union member and framework-specific value representation. Similar source syntax may use different typed nodes.

## Text Accessors

| API | Result | Use |
| --- | --- | --- |
| `SyntaxToken::text_trimmed()` | `&str` | Borrow token text without leading or trailing trivia |
| `SyntaxToken::token_text_trimmed()` | `TokenText` | Retain token-backed text without trivia |
| `SyntaxToken::text()` | `&str` | Exact source text including trivia |
| `SyntaxNode::text_trimmed()` | node text | Diagnostics or display where interior trivia is intentional |
| `inner_string_text(token)` free helper | `TokenText` | Quote-stripped token content |
| `HtmlString::inner_string_text()` | `SyntaxResult<TokenText>` | Quote-stripped content from a typed HTML string node |

`SyntaxNode::text_trimmed()` removes only outer trivia. Whitespace and comments between child tokens remain, so it is not a semantic accessor for a multi-token node. Prefer a typed token, typed accessor, or structural match.

Do not call `.to_string()`, `.to_trimmed_string()`, or `format!` merely to compare syntax text. Borrow a `&str` or retain a `TokenText`.

## Quoted Strings

Use the syntax crate's `inner_string_text()` implementation. It handles the language's token representation and returns token-backed text.

```rust
let string = value.as_html_string()?;
let inner = string.inner_string_text().ok()?;
let matches_handler = inner.text() == "handler";
```

Do not strip quotes with byte slicing. Besides allocating or risking invalid boundaries, manual slicing duplicates language-specific behavior already covered by the accessor and its tests.

## Store Substrings Without Allocation

`TokenText` already stores a green token plus a token-relative `TextRange`. Use its existing APIs instead of adding another token-and-range wrapper:

```rust
let word = inner.clone().slice(word_range);
let text = word.text();
let relative_range = word.relative_range();
let source_range = word.source_range(value_token.text_range());
```

Pass `slice()` a range relative to the current `TokenText`; it retains the underlying token and adjusts the selected range. `relative_range()` returns the resulting range relative to the whole token, while `source_range()` translates it using the whole token's file range. Build slice ranges from valid byte boundaries in the current text.

## Embedded Languages

Verify value shapes per framework and syntax version. For example, an HTML-family attribute may hold a quoted `HtmlString`, a single text expression, or a framework-specific directive node. Do not handle one shape and assume the others share its accessor or offset rules.

Keep embedded-language parsing concerns separate:

- parser nodes establish the source shape;
- syntax helpers extract source-backed text;
- embedding code translates source ranges and selects the embedding kind;
- analyzers consume the resulting typed data.

Support legacy syntax only when the task requires it and the current parser represents it.

## Temporary Borrows

Typed accessors often return owned wrapper nodes whose tokens borrow through that wrapper. Bind intermediate values when a chained expression would drop the owner too early:

```rust
let value_node = attribute.value().ok()?;
let string = value_node.as_html_string()?;
let token = string.value_token().ok()?;
let text = token.text_trimmed();
```

Do not solve a temporary-borrow error by converting to `String` unless the value genuinely must outlive its syntax tree.

## Review Checklist

- The code uses a typed node or token rather than matching a multi-token node's rendered text.
- Token comparisons exclude trivia intentionally.
- Quote stripping uses the language helper.
- Stored text remains token-backed when the syntax tree outlives it.
- Relative ranges are relative to the stored token text, not the file.
- Absolute offsets account for quotes exactly once.
- Every relevant framework or union variant is covered or deliberately excluded.
- No allocation was introduced solely to satisfy borrowing or comparison.

## References

- Token text: `crates/biome_rowan/src/token_text.rs`
- Syntax tokens: `crates/biome_rowan/src/syntax/token.rs`
- Syntax nodes: `crates/biome_rowan/src/syntax/node.rs`
- HTML string helpers: `crates/biome_html_syntax/src/string_ext.rs`
- Language-specific syntax extension modules under `crates/biome_*_syntax/src/`
