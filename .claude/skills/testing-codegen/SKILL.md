---
name: testing-codegen
description: Guide for testing workflows and code generation commands in Biome. Use when running snapshot tests for lint rules, managing insta snapshots, or regenerating analyzer/parser/formatter code after changes.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

Use this skill for testing and code generation. Covers snapshot testing with
`insta` and code generation commands.

## Prerequisites

1. Install required tools: `just install-tools` (installs `cargo-insta`)
2. Install pnpm: `corepack enable` and `pnpm install` in repo root
3. Understand which changes require code generation

## Code Standards

**CRITICAL: No Emojis**

Emojis are BANNED in all test and generated code:
- NO emojis in test files
- NO emojis in snapshot files
- NO emojis in changeset files
- NO emojis in generated code or scaffolding

Keep all code and documentation professional and emoji-free.

## Common Workflows

### Run Tests

```shell
# Run all tests
cargo test

# Run tests for specific crate
cd crates/biome_js_analyze
cargo test

# Run specific test
cargo test quick_test

# Show test output (for dbg! macros)
cargo test quick_test -- --show-output

# Run tests with just (uses CI test runner)
just test

# Test specific crate with just
just test-crate biome_cli
```

### Quick Test for Rules

Fast iteration during development:

```rust
// In crates/biome_js_analyze/tests/quick_test.rs
// Modify the quick_test function:

const SOURCE: &str = r#"
const x = 1;
var y = 2;
"#;

let rule_filter = RuleFilter::Rule("nursery", "noVar");
```

Run:

```shell
just qt biome_js_analyze
```

### Quick Test for Parser Development

**IMPORTANT:** Use this instead of building full Biome binary for syntax inspection - it's much faster!

For inspecting AST structure when implementing parsers or working with embedded languages:

```rust
// In crates/biome_html_parser/tests/quick_test.rs
// Modify the quick_test function:

#[test]
pub fn quick_test() {
  let code = r#"<button on:click={handleClick}>Click</button>"#;

  let source_type = HtmlFileSource::svelte();
  let options = HtmlParserOptions::from(&source_type);
  let root = parse_html(code, options);
  let syntax = root.syntax();

  dbg!(&syntax, root.diagnostics(), root.has_errors());
}
```

Run:

```shell
just qt biome_html_parser
```

The `dbg!` output shows the full AST tree structure, helping you understand:

- How directives/attributes are parsed (e.g., `HtmlAttribute` vs `SvelteBindDirective`)
- Whether values use `HtmlString` (quotes) or `HtmlTextExpression` (curly braces)
- Token ranges and offsets needed for proper snippet creation
- Node hierarchy and parent-child relationships

### Snapshot Testing with Insta

Run tests and generate snapshots:

```shell
cargo test
```

Review generated/changed snapshots:

```shell
# Interactive review (recommended)
cargo insta review

# Accept all changes
cargo insta accept

# Reject all changes
cargo insta reject

# Review for specific test
cargo insta review --test-runner nextest
```

Snapshot commands:

- `a` - accept snapshot
- `r` - reject snapshot
- `s` - skip snapshot
- `q` - quit

### Pruning Orphaned Snapshots

When tests are removed or renamed, their old snapshot files become orphaned. **Never delete snapshot files manually with `rm`** — always use insta's built-in pruning:

```shell
# Delete unreferenced snapshots after a successful test run
cargo insta test --unreferenced delete -p <crate_name>

# Or scoped to specific tests
cargo insta test --unreferenced delete -p biome_cli --test main -- "handle_vue"
```

This runs the tests first, then deletes any `.snap` files that no test references. It is the only safe way to clean up snapshots — manual `rm` risks deleting snapshots that are still needed or creating git conflicts.

### Test Lint Rules

```shell
# Test specific rule by name
just test-lintrule noVar

# Run from analyzer crate
cd crates/biome_js_analyze
cargo test
```

### Create Test Files

**Single file tests** - Place in `tests/specs/{group}/{rule}/` under the appropriate `*_analyze` crate for the language:

```
tests/specs/nursery/noVar/
├── invalid.js           # Code that should generate diagnostics
├── valid.js             # Code that should not generate diagnostics
└── options.json         # Optional: rule configuration
```

**File and folder naming conventions (IMPORTANT):**

- Use `valid` or `invalid` in file names or parent folder names to indicate expected behaviour.
- Files/folders with `valid` in the name (but not `invalid`) are expected to produce **no diagnostics**.
- Files/folders with `invalid` in the name are expected to produce **diagnostics**.
- When testing cases inside a folder, prefix the name of folder using `valid`/`invalid` e.g. `validResolutionReact`/`invalidResolutionReact`

```
tests/specs/nursery/noShadow/
├── invalid.js                     # should generate diagnostics
├── valid.js                       # should not generate diagnostics
├── validResolutionReact/
└───── file.js              # should generate diagnostics
   └── file2.js             # should not generate diagnostics
```

**Multiple test cases** - Use `.jsonc` files with arrays:

```jsonc
// tests/specs/nursery/noVar/invalid.jsonc
[
  "var x = 1;",
  "var y = 2; var z = 3;",
  "for (var i = 0; i < 10; i++) {}"
]
```

**Test-specific options** - Create `options.json`:

```json
{
  "linter": {
    "rules": {
      "nursery": {
        "noVar": {
          "level": "error",
          "options": {
            "someOption": "value"
          }
        }
      }
    }
  }
}
```

### Top-Level Comment Convention (REQUIRED)

Every test spec file **must** begin with a top-level comment declaring whether it expects diagnostics. The test runner
(`assert_diagnostics_expectation_comment` in `biome_test_utils`) enforces this and panics if the rules are violated.

Write the marker text using whatever comment syntax the language under test supports.
For languages that do not support comments at all, rely on the file/folder naming convention (`valid`/`invalid`) instead.

**For files whose name contains "valid" (but not "invalid"):**

The comment is **mandatory** — the test panics if it is absent.

**For files whose name contains "invalid" (or other names):**

The comment is strongly recommended and is also enforced when present: if the comment says
`should generate diagnostics` but no diagnostics appear, the test panics.

**Rules enforced by the test runner:**

| File name contains        | Comment present?                  | Behaviour                          |
|---------------------------|-----------------------------------|------------------------------------|
| "valid" (not "invalid")   | `should not generate diagnostics` | Passes if no diagnostics           |
| "valid" (not "invalid")   | `should generate diagnostics`     | Passes if diagnostics present      |
| "valid" (not "invalid")   | absent                            | **PANIC** — comment is mandatory   |
| "invalid" or neutral name | `should not generate diagnostics` | Passes if no diagnostics           |
| "invalid" or neutral name | `should generate diagnostics`     | Passes if diagnostics present      |
| "invalid" or neutral name | absent                            | No enforcement (but add it anyway) |

**Important details:**

- The comment is found by scanning the entire file's leading trivia — it does not have to be literally the first token, but putting it at the very top (line 1) is the established convention.
- Fixture/support files (e.g. `foo.js`, `bar.ts`) that don't contain "valid" or "invalid" in their name do **not** require a comment, since they are not considered "valid test files" by the runner.
- Files excluded from comment enforcement regardless of name: `.snap`, `.json`, `.jsonc`.

**HTML-ish files (`.vue`, `.svelte`, `.astro`, `.html`):**

These files are analyzed via the workspace-based test path (`analyze_with_workspace` in `biome_test_utils`), which
checks the expectation comment by scanning the **raw file content** (not the parsed AST trivia). Use an HTML comment
at the very top of the file:

```vue
<!-- should not generate diagnostics -->
<script setup lang="ts">
const x = 1;
</script>
<template>{{ x }}</template>
```

```vue
<!-- should generate diagnostics -->
<script>
debugger;
</script>
```

The same rules apply: valid files **must** have the comment, invalid files **should** have it.
Do not place the comment inside `<script>` — put it at the top level of the file as an HTML comment.

### Code Generation Commands

**After modifying analyzers/lint rules (during development):**

```shell
just gen-rules          # Updates rule registrations in *_analyze crates
just gen-configuration  # Updates configuration schemas
```

These lightweight commands generate enough code to compile and test without errors.

**Full analyzer codegen (optional — CI autofix handles this):**

```shell
just gen-analyzer
```

This is a composite command that runs `gen-rules`, `gen-configuration`, `gen-migrate`, `gen-bindings`, `lint-rules`, and `format`. You typically don't need to run this locally — the CI autofix job does it automatically when you open a PR.

**After modifying grammar (.ungram files):**

```shell
# Specific language
just gen-grammar html

# Multiple languages
just gen-grammar html css

# All languages
just gen-grammar
```

**After modifying formatters:**

```shell
just gen-formatter html
```

**After modifying configuration:**

```shell
just gen-bindings
```

Generates TypeScript types and JSON schema.

**Full codegen (rarely needed):**

```shell
just gen-all
```

**Before committing:**

```shell
just ready
```

Runs full codegen + format + lint (takes time).

Or run individually:

```shell
just f  # Format Rust and TOML
just l  # Lint code
```

### Run Doctests

Test code examples in documentation comments:

```shell
just test-doc
```

### Debugging Tests

Use `dbg!()` macro in Rust code:

```rust
fn some_function() -> &'static str {
  let some_variable = "debug_value";
  dbg!(&some_variable);  // Prints during test
  some_variable
}
```

Run with output:

```shell
cargo test test_name -- --show-output
```

## Tips

- **Snapshot organization**: Group by feature/rule in separate directories
- **Test both valid and invalid**: Create both `valid.js` and `invalid.js` files
- **Options per folder**: `options.json` applies to all tests in that folder
- **`.jsonc` arrays**: Use for multiple quick test cases in script context (no imports/exports)
- **Code generation order**: Grammar → Analyzer → Formatter → Bindings
- **CI compatibility**: Use `just` commands when possible (matches CI)
- **Snapshot review**: Always review snapshots carefully - don't blindly accept
- **Test performance**: Use `#[ignore]` for slow tests, run with `cargo test -- --ignored`
- **Parser inspection**: Use `just qt <package>` to run quick_test and inspect AST, NOT full Biome builds (much faster)

For general Biome development tips (string extraction, borrow checker patterns, legacy syntax),
see the [biome-developer](../biome-developer/SKILL.md) skill.

## Common Test Patterns

```rust
// Snapshot test in rule file
#[test]
fn test_rule() {
  assert_lint_rule! {
        noVar,
        invalid => [
            "var x = 1;",
            "var y = 2;",
        ],
        valid => [
            "const x = 1;",
            "let y = 2;",
        ]
    }
}

// Quick test pattern
#[test]
#[ignore]  // Uncomment when using
fn quick_test() {
  const SOURCE: &str = r#"
        var x = 1;
    "#;

  let rule_filter = RuleFilter::Rule("nursery", "noVar");
  // Test runs with this configuration
}
```

## Code Generation Dependencies

| When you modify...         | Run during dev...                         | Full (optional, CI does this) |
|----------------------------|-------------------------------------------|-------------------------------|
| `.ungram` grammar files    | `just gen-grammar <lang>`                 | —                             |
| Lint rules in `*_analyze`  | `just gen-rules && just gen-configuration`| `just gen-analyzer`           |
| Formatter in `*_formatter` | `just gen-formatter <lang>`               | —                             |
| Configuration types        | `just gen-bindings`                       | —                             |
| Before committing          | `just f && just l`                        | —                             |
| Full rebuild               | —                                         | `just gen-all` (slow)         |

## References

- Main testing guide: `CONTRIBUTING.md` § Testing
- Insta documentation: https://insta.rs
- Analyzer testing: `crates/biome_analyze/CONTRIBUTING.md` § Testing
- Changeset guide: `../changeset/SKILL.md`
