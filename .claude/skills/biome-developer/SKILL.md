---
name: biome-developer
description: General development best practices and common gotchas when working on Biome. Use for avoiding common mistakes, understanding Biome-specific patterns (AST, syntax nodes, string extraction, embedded languages), and learning technical tips.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

This skill provides general development best practices, common gotchas, and Biome-specific patterns that apply across different areas of the codebase. Use this as a reference when you encounter unfamiliar APIs or need to avoid common mistakes.

## Prerequisites

- Basic familiarity with Rust
- Understanding of Biome's architecture (parser, analyzer, formatter)
- Development environment set up (see CONTRIBUTING.md)

## Universal Code Standards

### CRITICAL: No Emojis Policy

**Emojis are absolutely BANNED in all code contributions.**

This applies to:
- Source code (Rust, JavaScript, TypeScript, etc.)
- Code comments and documentation (inline comments, rustdoc, JSDoc, etc.)
- Diagnostic messages and error text
- Test files and test data
- Commit messages
- Pull request titles and descriptions
- Any generated code or scaffolding
- Configuration files and JSON data

**Why:**
- Professional codebase standards
- Consistency across the project
- Avoid encoding/rendering issues
- Keep communication clear and technical

**Examples:**

```rust
// Bad: WRONG - Contains emoji
/// This function is super cool!
fn calculate() { }

// Good: CORRECT - No emoji
/// Calculates the optimal value using binary search.
fn calculate() { }
```

```rust
// Bad: WRONG - Emoji in diagnostic
markup! { "This is not allowed!" }

// Good: CORRECT - Clear text
markup! { "This syntax is not allowed." }
```

**Enforcement:** All agents and contributors must follow this rule. No exceptions.

## Common Gotchas and Best Practices

### Working with AST and Syntax Nodes

**DO:**
- Use parser crate's `quick_test` to inspect AST structure before implementing
- Understand the node hierarchy and parent-child relationships
- Check both general cases AND specific types (e.g., Vue has both `VueDirective` and `VueV*ShorthandDirective`)
- Verify your solution works for all relevant variant types, not just the first one you find
- Extract helper functions that return `Option<T>` or `SyntaxResult<T>` instead of scattering early returns throughout the caller — this makes code more readable and composable

**DON'T:**
- Do NOT build the full Biome binary just to inspect syntax (expensive) - use parser crate's `quick_test` instead
- Do NOT assume syntax patterns without inspecting the AST first

**Example - Inspecting AST:**
```rust
// In crates/biome_html_parser/tests/quick_test.rs
// Modify the quick_test function:
#[test]
pub fn quick_test() {
    let code = r#"<button on:click={handleClick}>Click</button>"#;
    let source_type = HtmlFileSource::svelte();
    let options = HtmlParserOptions::from(&source_type);
    let root = parse_html(code, options);
    dbg!(&root.syntax());  // Shows full AST structure
}
```

Run: `just qt biome_html_parser`

**Example - Extracting CST Navigation Logic:**
```rust
// WRONG: Many early returns scattered in the caller
fn visit_attribute(&self, attr: JsxAttribute, collector: &mut Collector) {
    let Ok(name_node) = attr.name() else { return };
    let name_text = match name_node {
        AnyJsxAttributeName::JsxName(n) => match n.value_token() {
            Ok(t) => t.token_text_trimmed(),
            Err(_) => return,
        },
        AnyJsxAttributeName::JsxNamespaceName(_) => return,
    };
    if name_text != "class" && name_text != "className" {
        return;
    }
    let Some(jsx_string) = attr.initializer().and_then(|i| i.value().ok()) else {
        return;
    };
    // ... do the real work
}

// CORRECT: Extract helper that returns Option<T>
fn visit_attribute(&self, attr: JsxAttribute, collector: &mut Collector) {
    if let Some(inner) = self.extract_class_attribute_inner(&attr) {
        self.collect_classes(&inner, collector);
    }
}

fn extract_class_attribute_inner(&self, attr: &JsxAttribute) -> Option<TokenText> {
    let name_node = attr.name().ok()?;
    let name_text = match name_node {
        AnyJsxAttributeName::JsxName(n) => n.value_token().ok()?.token_text_trimmed(),
        AnyJsxAttributeName::JsxNamespaceName(_) => return None,
    };
    if name_text != "class" && name_text != "className" {
        return None;
    }
    let jsx_string = attr.initializer().and_then(|i| i.value().ok())?;
    jsx_string.inner_string_text().ok()
}
```

The helper uses `?` operator and `Option` combinators — much cleaner than scattered `else { return }` blocks. The caller now has a single `if let Some` that clearly expresses intent.

### String Extraction and Text Handling

**DO:**
- Use `inner_string_text()` when extracting content from quoted strings — it strips the surrounding quotes and returns a `TokenText` backed by the same green token (no allocation)
- Use `text_trimmed()` when you need the full token text without leading/trailing whitespace
- Use `token_text_trimmed()` on nodes like `HtmlAttributeName` to get the text content
- Verify whether values use `HtmlString` (quotes) or `HtmlTextExpression` (curly braces)
- Use `TokenText::slice()` or `inner_string_text()` to get sub-ranges of a token — both return a `TokenText` backed by the same `GreenToken` (ref-count bump only, no heap allocation)

**DON'T:**
- Use `text_trimmed()` when you need `inner_string_text()` for extracting quoted string contents
- Call `.text()` on a `SyntaxToken` — it returns raw text including surrounding trivia (whitespace, newlines). Always use `.text_trimmed()` instead.
- Strip quotes manually with `&s[1..s.len()-1]` — use `inner_string_text()` instead; it is correct, allocation-free, and communicates intent
- Use `word.to_string()` or `String::from(word)` to store individual words split out of a string token — store the `TokenText` of the whole token plus a token-relative `TextRange` instead (see below)

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

**Example - CSS class name extraction from `CssClassSelector`:**
```rust
// WRONG: .text() includes trivia
let name = selector.name().ok()?.value_token().ok()?.text(); // may include whitespace

// CORRECT: always use text_trimmed() on SyntaxToken
let name: &str = selector.name().ok()?.value_token().ok()?.text_trimmed();
// For owned value:
let name: TokenText = selector.name().ok()?.value_token().ok()?.token_text_trimmed();
```

### Storing Split Token Words Without Allocation

When you need to split a string token (e.g. `class="foo bar baz"`) into individual words and store each word, do **not** allocate a `String` per word. Instead, store the `TokenText` of the whole token and a `TextRange` that is **relative to the token text** (not the file).

```rust
// WRONG: allocates a String per word
for word in content.split_ascii_whitespace() {
    collected.push(word.to_string()); // heap allocation per word
}

// CORRECT: store token + token-relative range
// Use inner_string_text() to get the quote-stripped TokenText first.
let inner: TokenText = html_string.inner_string_text()?;
let content = inner.text();
let mut offset: u32 = 0;
for word in content.split_ascii_whitespace() {
    let word_offset = content[offset as usize..]
        .find(word)
        .map_or(offset, |pos| offset + pos as u32);
    let start = TextSize::from(word_offset);
    let end = start + TextSize::from(word.len() as u32);
    collected.push(MyEntry {
        token: inner.clone(), // refcount bump only
        range: TextRange::new(start, end),
    });
    offset = word_offset + word.len() as u32;
}

// Later, to read the word back:
fn text(&self) -> &str {
    &self.token.text()[usize::from(self.range.start())..usize::from(self.range.end())]
}
```

Key points:
- `inner_string_text()` returns a `TokenText` whose `.text()` starts at byte 0 of the unquoted content. Word offsets within that are directly usable as token-relative ranges.
- `TokenText::clone()` is a refcount bump on the underlying `GreenToken` — it does not copy string data.
- To produce file-level diagnostic ranges from token-relative ranges, add the token's absolute file offset: `u32::from(value_token.text_trimmed_range().start()) + 1` (the `+1` skips the opening quote).

### Working with Embedded Languages

**DO:**
- Verify changes work for different value formats (quoted strings vs text expressions) when handling multiple frameworks
- Use appropriate `EmbeddingKind` for context (Vue, Svelte, Astro, etc.)
- Check if embedded content needs `is_source: true` (script tags) vs `is_source: false` (template expressions)
- Calculate offsets correctly: token start + 1 for opening quote, or use `text_range().start()` for text expressions

**DON'T:**
- Do NOT assume all frameworks use the same syntax (Vue uses quotes, Svelte uses curly braces)
- Do NOT implement features for "widely used" patterns without evidence - ask the user first

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
- Use intermediate `let` bindings to avoid temporary value borrows that get dropped
- Store method results that return owned values before calling methods on them

**DON'T:**
- Do NOT create temporary value borrows that get dropped before use

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
- Use `let` chains to collapse nested `if let` statements (cleaner and follows Rust idioms)
- Run `just l` before committing to catch clippy warnings
- Fix clippy suggestions unless there's a good reason not to

**DON'T:**
- Do NOT ignore clippy warnings - they often catch real issues or suggest better patterns

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

### Code Comments

Comments exist for the next developer who reads this code, not for the developer currently writing it. Write them like you are explaining the code to a colleague who walked into the room ten minutes ago — not to a reviewer on this specific PR.

**DO:**
- Explain code that is hard to read, or document exceptions and edge cases
- Provide context when names alone are not descriptive enough
- Describe the business logic a function implements
- Clarify contextual words like "normalize" — e.g., "normalize a file path" and "normalize a URL" mean different things; spell out what normalization means here
- Strike a balance between plain English and technical precision. Prefer concrete nouns ("the HTML file", "the `<style>` block") over abstract ones ("the host CST", "the delegated pipeline") when both convey the same idea
- Add comments only where they are needed, for example, docstrings, or code paths that are particular and require a special explanation. Most of the code (even new that you write) doesn't need a comment if it follows the business logic.
- Write comments using proper English grammar and punctuation.

**DON'T:**
- Do NOT embed the context of the current work into comments. A comment like `// As per issue #1234, we skip this case` ties the code to a transient artifact. Instead, explain *why* the case is skipped in terms any future reader would understand.
- Do NOT scope comments to the specific trigger that prompted the change. For example, if a bug was reported for Astro but the fix applies broadly, do NOT write `// Fix for Astro embedding`. Write a comment that describes the general condition being handled.
- Do NOT scope comments narrower than the code itself. If the function is generic across all embedded languages, the comment should not name "CSS" or "`<style>`" — describe the contract the code enforces for any embed, and use a concrete example only as illustration.
- Do NOT lead with formal-methods / math jargon like `// Invariant:`, `// Precondition:`, `// Lemma:` unless the surrounding code genuinely uses those terms. For most Biome code, plain prose ("When X happens, Y must hold, otherwise …") reads better and is just as precise.
- Do NOT pile technical terms on top of each other ("delegated format pipeline", "canonical embed IR", "host CST token text") when one plain-English sentence would do. Jargon density should be low; a reader should not need a glossary to understand a comment.
- Do NOT just paraphrase the function name or the next line of code. If a comment can be deleted without losing information, delete it.

**Think big picture, not current task.** Before writing a comment, ask three things:

1. If someone reads this a year from now with no knowledge of the issue or PR, does this comment give them the context they need?
2. Is my comment describing the code at the same level of abstraction as the code? (A generic helper deserves a generic explanation; a specific branch deserves a specific one.)
3. Could I swap any technical term for a plainer word without losing meaning? If yes, swap it.

**Example 1 — issue/task context and over-specificity:**
```rust
// WRONG: Carries issue/task context
// Fix for #5678: Astro files need special handling here
if is_embedded_script(node) {
    return normalize_offset(node);
}

// WRONG: Describes what the code does (the code already says that)
// Check if the node is an embedded script and normalize the offset
if is_embedded_script(node) {
    return normalize_offset(node);
}

// CORRECT: Explains why and clarifies "normalize"
// Embedded script blocks (e.g. <script> inside .vue/.svelte/.astro files)
// report offsets relative to the embedding document, not the script itself.
// Normalize here means: subtract the script block's start position so the
// offset is relative to the script content.
if is_embedded_script(node) {
    return normalize_offset(node);
}
```

**Example 2 — jargon, narrow scope, and abstraction mismatch.** This is a real example from a generic helper that replaces an embedded snippet inside any host document (HTML, Vue, Svelte, Astro, …):

```rust
// WRONG: starts with formal-methods jargon, names a specific case
// (`<style>`) even though the function handles any embed, and stacks
// technical terms ("host CST token text", "delegated pipeline") that a
// new reader has to decode before they can understand the point.
// Invariant: for a file that required no fix actions, `fix_file` and
// `format_file` must produce byte-identical output. For `<style>`
// blocks, `fix_all`'s final format pass prints embedded content
// verbatim from the host CST token text, while `format_file` routes
// through the delegated `format_embedded` pipeline and re-wraps the
// result with the host's indent. …

// CORRECT: plain language, stays at the generic level of the function,
// uses `<style>` only as a parenthetical example, and is understandable
// without prior context.
// The embedded formatter (e.g. the CSS formatter for a <style> block)
// doesn't know how deeply its code is nested inside the HTML file, so
// it always returns the code indented from column zero. If we pasted
// that code back as-is, only the first line would get the HTML
// indentation (from the leading whitespace we already captured); every
// other line would end up too far to the left. Add the same indentation
// to every line so the embed lines up with its surroundings.
```

The corrected comment names one concrete example (`<style>` / CSS) to make the reader's mental picture vivid, but the rest of the sentence is generic enough to cover any host/embed pair. That is the balance to aim for.

### Cargo Dependencies: `workspace = true` vs `path = "..."`

Internal `biome_*` crates listed under `[dev-dependencies]` **MUST** use `path = "../<crate_name>"`, not `workspace = true`. Using `workspace = true` for dev-dependencies can cause Cargo to resolve the crate from the registry instead of the local workspace, which is incorrect.

Regular `[dependencies]` still use `workspace = true` as normal — this rule only applies to `[dev-dependencies]`.

**DO:**
- Use `path = "../biome_foo"` for all `biome_*` dev-dependencies
- Preserve any extra attributes like `features` when converting

**DON'T:**
- Do NOT use `workspace = true` for `biome_*` crates in `[dev-dependencies]`

**Example:**
```toml
# WRONG: may resolve from registry
[dev-dependencies]
biome_js_parser = { workspace = true }
biome_formatter = { workspace = true, features = ["countme"] }

# CORRECT: always resolves locally
[dev-dependencies]
biome_js_parser = { path = "../biome_js_parser" }
biome_formatter = { path = "../biome_formatter", features = ["countme"] }
```

All crates live as siblings under `crates/`, so the relative path is always `../biome_<name>`.

### Legacy and Deprecated Syntax

**DO:**
- Ask users before implementing deprecated/legacy syntax support
- Wait for user demand before spending time on legacy features
- Document when features are intentionally not supported due to being legacy

**DON'T:**
- Do NOT implement legacy/deprecated syntax without checking with the user first
- Do NOT claim patterns are "widely used" or "common" without evidence

**Example:**
Svelte's `on:click` event handler syntax is legacy (Svelte 3/4). Modern Svelte 5 runes mode uses regular attributes. Unless users specifically request it, don't implement legacy syntax support.

### Testing and Development

For testing commands, snapshot workflows, and code generation, see the
[testing-codegen](../testing-codegen/SKILL.md) skill. Key reminders specific to
Biome development patterns:

- Test with multiple variants when working with enums (e.g., all `VueV*ShorthandDirective` types)
- Use CLI tests for testing embedded languages (Vue/Svelte directives, etc.)
- Do NOT try to test embedded languages in analyzer packages (they don't have embedding capabilities)

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
| `inner_string_text()` | Extracting content from quoted strings | Content without quotes, as `TokenText` (no alloc) |
| `text_trimmed()` | Getting token text without whitespace | `&str` — full token text |
| `token_text_trimmed()` | Getting an owned, cloneable token text | `TokenText` — backed by green token |
| `text()` | Getting raw text including trivia | `&str` — exact text as written |

### `Text` vs `TokenText` vs `String`

| Type | Size | Clone cost | Use when |
| --- | --- | --- | --- |
| `TokenText` | 16 bytes | Refcount bump | You have a `SyntaxToken` and want allocation-free ownership |
| `Text` | 16 bytes | Refcount bump (token) or heap copy (owned) | Union of `TokenText` and an owned string — use when the source may not be a token |
| `String` | 24 bytes | Heap copy | Only when you actually need an owned, mutable string (e.g. for a diagnostic message) |

`Text` is the richer type: `From<TokenText>` is implemented, so a `TokenText` can always be cheaply wrapped in `Text`. When storing data extracted directly from a syntax token, prefer `TokenText` or the token+range pattern.

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
- Use spaces around table separators: `| --- | --- | --- |` (not `|---|---|---|`)
- Ensure all Markdown tables follow "compact" style with proper spacing
- Test documentation changes with markdown linters before committing

**DON'T:**
- Do NOT use compact table separators without spaces (causes CI linting failures)

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
