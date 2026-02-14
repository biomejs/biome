---
name: biome-developer
description: General development best practices and common gotchas when working on Biome. Use for avoiding common mistakes, understanding Biome-specific patterns, and learning technical tips. Examples:<example>Working with Biome's AST and syntax nodes</example><example>Understanding string extraction methods</example><example>Handling embedded languages and directives</example>
---

## Purpose

This skill provides general development best practices, common gotchas, and Biome-specific patterns that apply across different areas of the codebase. Use this as a reference when you encounter unfamiliar APIs or need to avoid common mistakes.

## Prerequisites

- Basic familiarity with Rust
- Understanding of Biome's architecture (parser, analyzer, formatter)
- Development environment set up (see CONTRIBUTING.md)

## Common Gotchas and Best Practices

### Working with AST and Syntax Nodes

**DO:**
- ✅ Use parser crate's `quick_test` to inspect AST structure before implementing
- ✅ Understand the node hierarchy and parent-child relationships
- ✅ Check both general cases AND specific types (e.g., Vue has both `VueDirective` and `VueV*ShorthandDirective`)
- ✅ Verify your solution works for all relevant variant types, not just the first one you find

**DON'T:**
- ❌ Build the full Biome binary just to inspect syntax (expensive) - use parser crate's `quick_test` instead
- ❌ Assume syntax patterns without inspecting the AST first

**Example - Inspecting AST:**
```rust
// In crates/biome_html_parser/tests/quick_test.rs
// Modify the quick_test function:
#[test]
pub fn quick_test() {
    let code = r#"<button on:click={handleClick}>Click</button>"#;
    let source_type = HtmlFileSource::svelte();
    let options = HtmlParseOptions::from(&source_type);
    let root = parse_html(code, options);
    dbg!(&root.syntax());  // Shows full AST structure
}
```

Run: `just qt biome_html_parser`

### String Extraction and Text Handling

**DO:**
- ✅ Use `inner_string_text()` when extracting content from quoted strings (removes quotes)
- ✅ Use `text_trimmed()` when you need the full token text without leading/trailing whitespace
- ✅ Use `token_text_trimmed()` on nodes like `HtmlAttributeName` to get the text content
- ✅ Verify whether values use `HtmlString` (quotes) or `HtmlTextExpression` (curly braces)

**DON'T:**
- ❌ Use `text_trimmed()` when you need `inner_string_text()` for extracting quoted string contents

**Example - String Extraction:**
```rust
// WRONG: text_trimmed() includes quotes
let html_string = value.as_html_string()?;
let content = html_string.value_token()?.text_trimmed(); // Returns: "\"handler\""

// CORRECT: inner_string_text() removes quotes
let html_string = value.as_html_string()?;
let inner_text = html_string.inner_string_text().ok()?;
let content = inner_text.text(); // Returns: "handler"
```

### Working with Embedded Languages

**DO:**
- ✅ Verify changes work for different value formats (quoted strings vs text expressions) when handling multiple frameworks
- ✅ Use appropriate `EmbeddingKind` for context (Vue, Svelte, Astro, etc.)
- ✅ Check if embedded content needs `is_source: true` (script tags) vs `is_source: false` (template expressions)
- ✅ Calculate offsets correctly: token start + 1 for opening quote, or use `text_range().start()` for text expressions

**DON'T:**
- ❌ Assume all frameworks use the same syntax (Vue uses quotes, Svelte uses curly braces)
- ❌ Implement features for "widely used" patterns without evidence - ask the user first

**Example - Different Value Formats:**
```rust
// Vue directives use quoted strings: @click="handler"
let html_string = value.as_html_string()?;
let inner_text = html_string.inner_string_text().ok()?;

// Svelte directives use text expressions: on:click={handler}
let text_expression = value.as_html_attribute_single_text_expression()?;
let expression = text_expression.expression().ok()?;
```

### Borrow Checker and Temporary Values

**DO:**
- ✅ Use intermediate `let` bindings to avoid temporary value borrows that get dropped
- ✅ Store method results that return owned values before calling methods on them

**DON'T:**
- ❌ Create temporary value borrows that get dropped before use

**Example - Avoiding Borrow Issues:**
```rust
// WRONG: Temporary borrow gets dropped
let html_string = value.value().ok()?.as_html_string()?;
let token = html_string.value_token().ok()?; // ERROR: html_string dropped

// CORRECT: Store intermediate result
let value_node = value.value().ok()?;
let html_string = value_node.as_html_string()?;
let token = html_string.value_token().ok()?; // OK
```

### Clippy and Code Style

**DO:**
- ✅ Use `let` chains to collapse nested `if let` statements (cleaner and follows Rust idioms)
- ✅ Run `just l` before committing to catch clippy warnings
- ✅ Fix clippy suggestions unless there's a good reason not to

**DON'T:**
- ❌ Ignore clippy warnings - they often catch real issues or suggest better patterns

**Example - Collapsible If:**
```rust
// WRONG: Nested if let (clippy::collapsible_if warning)
if let Some(directive) = VueDirective::cast_ref(&element) {
    if let Some(initializer) = directive.initializer() {
        // ... do something
    }
}

// CORRECT: Use let chains
if let Some(directive) = VueDirective::cast_ref(&element)
    && let Some(initializer) = directive.initializer()
{
    // ... do something
}
```

### Legacy and Deprecated Syntax

**DO:**
- ✅ Ask users before implementing deprecated/legacy syntax support
- ✅ Wait for user demand before spending time on legacy features
- ✅ Document when features are intentionally not supported due to being legacy

**DON'T:**
- ❌ Implement legacy/deprecated syntax without checking with the user first
- ❌ Claim patterns are "widely used" or "common" without evidence

**Example:**
Svelte's `on:click` event handler syntax is legacy (Svelte 3/4). Modern Svelte 5 runes mode uses regular attributes. Unless users specifically request it, don't implement legacy syntax support.

### Testing and Development

**DO:**
- ✅ Use `just qt <package>` to run quick tests (handles test execution automatically)
- ✅ Review snapshot changes carefully - don't blindly accept
- ✅ Test with multiple variants when working with enums (e.g., all `VueV*ShorthandDirective` types)
- ✅ Add tests for both valid and invalid cases
- ✅ Use CLI tests for testing embedded languages (Vue/Svelte directives, etc.)

**DON'T:**
- ❌ Blindly accept all snapshot changes
- ❌ Try to test embedded languages in analyzer packages (they don't have embedding capabilities)

## Pattern Matching Tips

### Working with Node Variants

When working with enum variants (like `AnySvelteDirective`), check if there are also non-enum types that need handling:

```rust
// Check AnySvelteDirective enum (bind:, class:, style:, etc.)
if let Some(directive) = AnySvelteDirective::cast_ref(&element) {
    // Handle special Svelte directives
}

// But also check regular HTML attributes with specific prefixes
if let Some(attribute) = HtmlAttribute::cast_ref(&element) {
    if let Ok(name) = attribute.name() {
        // Some directives might be parsed as regular attributes
    }
}
```

### Checking Multiple Variant Types

For frameworks with multiple directive syntaxes, handle each type:

```rust
// Vue has multiple shorthand types
if let Some(directive) = VueVOnShorthandDirective::cast_ref(&element) {
    // Handle @click
}
if let Some(directive) = VueVBindShorthandDirective::cast_ref(&element) {
    // Handle :prop
}
if let Some(directive) = VueVSlotShorthandDirective::cast_ref(&element) {
    // Handle #slot
}
if let Some(directive) = VueDirective::cast_ref(&element) {
    // Handle v-if, v-show, etc.
}
```

## Common API Confusion

### String/Text Methods

| Method | Use When | Returns |
| --- | --- | --- |
| `inner_string_text()` | Extracting content from quoted strings | Content without quotes |
| `text_trimmed()` | Getting token text without whitespace | Full token text |
| `token_text_trimmed()` | Getting text from nodes like `HtmlAttributeName` | Node text content |
| `text()` | Getting raw text | Exact text as written |

### Value Extraction Methods

| Type | Method | Framework |
| --- | --- | --- |
| `HtmlString` | `inner_string_text()` | Vue (quotes) |
| `HtmlAttributeSingleTextExpression` | `expression()` | Svelte (curly braces) |
| `HtmlTextExpression` | `html_literal_token()` | Template expressions |

## References

- Main contributing guide: `../../CONTRIBUTING.md`
- Testing workflows: `../testing-codegen/SKILL.md`
- Parser development: `../parser-development/SKILL.md`
- Biome internals docs: https://biomejs.dev/internals

## Documentation and Markdown Formatting

**DO:**
- ✅ Use spaces around table separators: `| --- | --- | --- |` (not `|---|---|---|`)
- ✅ Ensure all Markdown tables follow "compact" style with proper spacing
- ✅ Test documentation changes with markdown linters before committing

**DON'T:**
- ❌ Use compact table separators without spaces (causes CI linting failures)

**Example - Table Formatting:**
```markdown
<!-- WRONG: No spaces around separators -->
| Method | Use When | Returns |
|--------|----------|---------|

<!-- CORRECT: Spaces around separators -->
| Method | Use When | Returns |
| --- | --- | --- |
```

The CI uses `markdownlint-cli2` which enforces the "compact" style requiring spaces.

## When to Use This Skill

Load this skill when:
- Working with unfamiliar Biome APIs
- Getting borrow checker errors with temporary values
- Extracting strings or text from syntax nodes
- Implementing support for embedded languages (Vue, Svelte, etc.)
- Wondering why your AST inspection doesn't match expectations
- Making decisions about legacy/deprecated syntax support
- Writing or updating markdown documentation
