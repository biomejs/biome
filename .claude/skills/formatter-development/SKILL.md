---
name: formatter-development
description: Guide for implementing formatting rules using Biome's IR-based formatter infrastructure. Use when working on formatters for JavaScript, CSS, JSON, HTML, or other languages. Examples:<example>User needs to implement formatting for a new syntax node</example><example>User wants to handle comments in formatted output</example><example>User is comparing Biome's formatting against Prettier</example>
---

## Purpose

Use this skill when implementing or modifying Biome's formatters. It covers the trait-based formatting system, IR generation, comment handling, and testing with Prettier comparison.

## Prerequisites

1. Install required tools: `just install-tools` (includes `wasm-bindgen-cli` and `wasm-opt`)
2. Language-specific crates must exist: `biome_{lang}_syntax`, `biome_{lang}_formatter`
3. For Prettier comparison: Install `bun` and run `pnpm install` in repo root

## Common Workflows

### Generate Formatter Boilerplate

For a new language (e.g., HTML):

```shell
just gen-formatter html
```

This generates `FormatNodeRule` implementations for all syntax nodes. Initial implementations use `format_verbatim_node` (formats code as-is).

### Implement FormatNodeRule for a Node

Example: Formatting `JsIfStatement`:

```rust
use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsIfStatement, JsIfStatementFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsIfStatement;

impl FormatNodeRule<JsIfStatement> for FormatJsIfStatement {
    fn fmt_fields(&self, node: &JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = node.as_fields();

        write!(
            f,
            [
                if_token.format(),
                space(),
                l_paren_token.format(),
                test.format(),
                r_paren_token.format(),
                space(),
                consequent.format(),
            ]
        )?;

        if let Some(else_clause) = else_clause {
            write!(f, [space(), else_clause.format()])?;
        }

        Ok(())
    }
}
```

### Using IR Primitives

Common formatting building blocks:

```rust
use biome_formatter::{format_args, write};

write!(f, [
    token("if"),           // Static text
    space(),               // Single space
    soft_line_break(),     // Break if line is too long
    hard_line_break(),     // Always break
    
    // Grouping and indentation
    group(&format_args![
        token("("),
        soft_block_indent(&format_args![
            node.test.format(),
        ]),
        token(")"),
    ]),
    
    // Conditional formatting
    format_with(|f| {
        if condition {
            write!(f, [token("something")])
        } else {
            write!(f, [token("other")])
        }
    }),
])?;
```

### Handle Comments

```rust
use biome_formatter::format_args;
use biome_formatter::prelude::*;

impl FormatNodeRule<JsObjectExpression> for FormatJsObjectExpression {
    fn fmt_fields(&self, node: &JsObjectExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectExpressionFields {
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                block_indent(&format_args![
                    members.format(),
                    // Handle dangling comments (comments not attached to any node)
                    format_dangling_comments(node.syntax()).with_soft_block_indent()
                ]),
                r_curly_token.format(),
            ]
        )
    }
}
```

Leading and trailing comments are handled automatically by the formatter infrastructure.

### Compare Against Prettier

After implementing formatting, validate against Prettier:

```shell
# Compare a code snippet
bun packages/prettier-compare/bin/prettier-compare.js --rebuild 'const x={a:1,b:2}'

# Compare with explicit language
bun packages/prettier-compare/bin/prettier-compare.js --rebuild -l ts 'const x: number = 1'

# Compare a file
bun packages/prettier-compare/bin/prettier-compare.js --rebuild -f path/to/file.tsx

# From stdin (useful for editor selections)
echo 'const x = 1' | bun packages/prettier-compare/bin/prettier-compare.js --rebuild -l js
```

**Always use `--rebuild`** to ensure WASM bundle matches your Rust changes.

### Create Snapshot Tests

Create test files in `tests/specs/` organized by feature:

```
crates/biome_js_formatter/tests/specs/js/
├── statement/
│   ├── if_statement/
│   │   ├── basic.js
│   │   ├── nested.js
│   │   └── with_comments.js
│   └── for_statement/
│       └── various.js
```

Example test file `basic.js`:
```javascript
if (condition) {
  doSomething();
}

if (condition) doSomething();

if (condition) {
  doSomething();
} else {
  doOther();
}
```

Run tests:
```shell
cd crates/biome_js_formatter
cargo test
```

Review snapshots:
```shell
cargo insta review
```

### Test with Custom Options

Create `options.json` in the test folder:

```json
{
  "formatter": {
    "indentStyle": "space",
    "indentWidth": 2,
    "lineWidth": 80
  },
  "javascript": {
    "formatter": {
      "quoteStyle": "single",
      "semicolons": "asNeeded"
    }
  }
}
```

This applies to all test files in that folder.

### Format and Build

After changes:

```shell
just f              # Format Rust code
just l              # Lint
just gen-formatter  # Regenerate formatter infrastructure if needed
```

## Tips

- **format_verbatim_node**: Initial generated code uses this - replace it with proper IR as you implement formatting
- **Space tokens**: Use `space()` instead of `token(" ")` for semantic spacing
- **Breaking**: Use `soft_line_break()` for optional breaks, `hard_line_break()` for mandatory breaks
- **Grouping**: Wrap related elements in `group()` to keep them together when possible
- **Indentation**: Use `block_indent()` for block-level indentation, `indent()` for inline
- **Lists**: Use `join_nodes_with_soft_line()` or `join_nodes_with_hardline()` for formatting lists
- **Mandatory tokens**: Use `node.token().format()` for tokens that exist in AST, not `token("(")`
- **Debugging**: Use `dbg_write!` macro (like `dbg!`) to see IR elements: `dbg_write!(f, [token("hello")])?;`
- **Don't fix code**: Formatter should format existing code, not attempt to fix syntax errors

## IR Primitives Reference

```rust
// Whitespace
space()                    // Single space
soft_line_break()         // Break if needed
hard_line_break()         // Always break
soft_line_break_or_space() // Space or break

// Indentation
indent(&content)          // Indent content
block_indent(&content)    // Block-level indent
soft_block_indent(&content) // Indent with soft breaks

// Grouping
group(&content)           // Keep together if possible
conditional_group(&content) // Advanced grouping

// Text
token("text")             // Static text
dynamic_token(&text, pos) // Dynamic text with position

// Utility
format_with(|f| { ... })  // Custom formatting function
format_args![a, b, c]     // Combine multiple items
if_group_breaks(&content) // Only if group breaks
if_group_fits_on_line(&content) // Only if fits
```

## References

- Full guide: `crates/biome_formatter/CONTRIBUTING.md`
- JS-specific: `crates/biome_js_formatter/CONTRIBUTING.md`
- Prettier comparison tool: `packages/prettier-compare/`
- Examples: `crates/biome_js_formatter/src/js/` for real implementations
