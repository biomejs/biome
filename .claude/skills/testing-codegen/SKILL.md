---
name: testing-codegen
description: Guide for testing workflows and code generation commands in Biome. Use when running tests, managing snapshots, creating changesets, or generating code. Examples:<example>User needs to run snapshot tests for a lint rule</example><example>User wants to create a changeset for a PR</example><example>User needs to regenerate analyzer code after changes</example>
---

## Purpose

Use this skill for testing, code generation, and preparing contributions. Covers snapshot testing with `insta`, code generation commands, and changeset creation.

## Prerequisites

1. Install required tools: `just install-tools` (installs `cargo-insta`)
2. Install pnpm: `corepack enable` and `pnpm install` in repo root
3. Understand which changes require code generation

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
    let options = HtmlParseOptions::from(&source_type);
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

### Test Lint Rules

```shell
# Test specific rule by name
just test-lintrule noVar

# Run from analyzer crate
cd crates/biome_js_analyze
cargo test
```

### Create Test Files

**Single file tests** - Place in `tests/specs/{group}/{rule}/`:
```
tests/specs/nursery/noVar/
├── invalid.js           # Code that triggers the rule
├── valid.js             # Code that doesn't trigger
└── options.json         # Optional: rule configuration
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

### Code Generation Commands

**After modifying analyzers/lint rules:**
```shell
just gen-analyzer
```
This updates:
- Rule registrations
- Configuration schemas  
- Documentation exports
- TypeScript bindings

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

### Create Changeset

For user-visible changes (bug fixes, new features):

```shell
just new-changeset
```

This prompts for:
1. **Package selection**: Usually `@biomejs/biome`
2. **Change type**:
   - `patch` - Bug fixes
   - `minor` - New features
   - `major` - Breaking changes (requires targeting `next` branch)
3. **Description**: What changed (used in CHANGELOG)

**Changeset writing guidelines:**
- Be concise and clear (1-3 sentences)
- Start bug fixes with: `Fixed [#issue](link): ...`
- Use past tense for your actions: "Added", "Fixed", "Changed"
- Use present tense for Biome behavior: "Biome now supports..."
- Include code examples for new rules/features
- Link to rules: `[useConst](https://biomejs.dev/linter/rules/use-const/)`
- End sentences with periods

Example changeset:
```markdown
---
"@biomejs/biome": patch
---

Fixed [#1234](https://github.com/biomejs/biome/issues/1234): The rule [`noVar`](https://biomejs.dev/linter/rules/no-var/) now correctly handles variables in for loops.

Biome now analyzes the scope of loop variables properly.
```

**Edit changeset** - Files created in `.changeset/` directory, edit them directly.

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
- **Changeset timing**: Create before opening PR, can edit after
- **Snapshot review**: Always review snapshots carefully - don't blindly accept
- **Test performance**: Use `#[ignore]` for slow tests, run with `cargo test -- --ignored`
- **Parser inspection**: Use `just qt <package>` to run quick_test and inspect AST, NOT full Biome builds (much faster)
- **String extraction**: Use `inner_string_text()` for quoted strings, not `text_trimmed()` (which includes quotes)
- **Legacy syntax**: Ask users before implementing deprecated/legacy syntax - wait for user demand
- **Borrow checker**: Avoid temporary borrows that get dropped - use `let binding = value; binding.method()` pattern

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

| When you modify... | Run... |
| ------------------- | -------- |
| `.ungram` grammar files | `just gen-grammar <lang>` |
| Lint rules in `*_analyze` | `just gen-analyzer` |
| Formatter in `*_formatter` | `just gen-formatter <lang>` |
| Configuration types | `just gen-bindings` |
| Before committing | `just f && just l` |
| Full rebuild | `just gen-all` (slow) |

## References

- Main testing guide: `CONTRIBUTING.md` § Testing
- Insta documentation: https://insta.rs
- Analyzer testing: `crates/biome_analyze/CONTRIBUTING.md` § Testing
- Changeset guide: `CONTRIBUTING.md` § Changelog
