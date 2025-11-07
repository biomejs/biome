# Embedded Language Integration: Generic Solution

## Problem Statement

**This is a general problem affecting ALL embedded template languages** (Glimmer, Vue, Svelte, Astro):

1. Parser transforms source code: `<template>` → `__BIOME_TEMPLATE_MARKER__`
2. AST is built from transformed text
3. Semantic analysis only has access to transformed AST
4. **But we need original untransformed source to scan templates!**

### Why This Happens

```
Original Source → Parser (transforms) → Transformed Text → AST → Semantic Analysis
     ↑                                                                    ↓
     |                            NEED ACCESS HERE ←←←←←←←←←←←←←←←←←←←←←←←←
     |
  <template> blocks exist here, but not in AST!
```

## Generic Solution

### 1. Created `OriginalSourceText` Service

**Location**: `crates/biome_js_analyze/src/services/semantic.rs`

```rust
/// Generic service that holds the original source text for ANY embedded language
/// (before template/component transformation).
///
/// This is needed because parsers transform embedded templates (Glimmer, Vue, Svelte, Astro)
/// by replacing `<template>` tags with markers like `__BIOME_GLIMMER_TEMPLATE_0__`.
/// The semantic analysis needs access to the original untransformed source
/// to scan for template references.
#[derive(Clone)]
pub struct OriginalSourceText(Arc<String>);
```

### 2. Extended Semantic Model Builder

**Location**: `crates/biome_js_analyze/src/services/semantic.rs:171-205`

```rust
fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
    let mut builder = self.builder;

    // For any embedded language (Glimmer, Vue, Svelte, Astro), scan templates
    if let Some(file_source) = ctx.services.get_service::<JsFileSource>() {
        let embedding_kind = file_source.as_embedding_kind();

        // Check if this is any embedded language
        if embedding_kind.is_glimmer() || embedding_kind.is_vue()
           || embedding_kind.is_svelte() || embedding_kind.is_astro() {

            // Try to get the original source from services
            if let Some(original_source) = ctx.services.get_service::<OriginalSourceText>() {
                let source_text = original_source.text().to_string();

                // Scan templates based on language
                if embedding_kind.is_glimmer() {
                    add_template_references(&mut builder, &source_text);
                }
                // TODO: Add Vue, Svelte, Astro scanning here
            }
        }
    }

    let model = builder.build();
    ctx.services.insert_service(model);
}
```

### 3. What Needs to Be Done (NOT YET IMPLEMENTED)

To complete this solution, we need to:

#### A. Add `original_source_text` to `LintParams`

**File**: `crates/biome_service/src/file_handlers/mod.rs`

```rust
pub(crate) struct LintParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) settings: &'a Settings,
    // ... existing fields ...

    /// Original untransformed source text for embedded languages
    /// (Glimmer, Vue, Svelte, Astro). This is needed because the parse tree
    /// contains transformed text with templates replaced by markers.
    pub(crate) original_source_text: Option<String>,
}
```

#### B. Populate it in Glimmer Handler

**File**: `crates/biome_service/src/file_handlers/glimmer.rs:257`

```rust
fn lint(mut params: LintParams) -> LintResults {
    // Capture the original source before delegating to JavaScript linter
    // The parse tree was built from TRANSFORMED text (templates → markers)
    // but we need the ORIGINAL text for template scanning
    if let Some(text) = params.parse.text() {  // Get text from parse somehow
        params.original_source_text = Some(text.to_string());
    }

    javascript::lint(params)
}
```

#### C. Create Service in JavaScript Handler

**File**: `crates/biome_service/src/file_handlers/javascript.rs:763-770`

```rust
let mut services =
    JsAnalyzerServices::from((params.module_graph, params.project_layout, file_source));

// If original source text is provided (for embedded languages), add it to services
if let Some(original_text) = params.original_source_text {
    use biome_js_analyze::services::semantic::OriginalSourceText;
    services.insert(OriginalSourceText::new(original_text));
}

let (_, analyze_diagnostics) = analyze(
    &tree,
    filter,
    &analyzer_options,
    &params.plugins,
    services,
    |signal| process_lint.process_signal(signal),
);
```

## Benefits of This Approach

### ✅ Generic
- Works for ALL embedded languages (Glimmer, Vue, Svelte, Astro)
- No language-specific hacks

### ✅ Opt-In
- `Option<String>` in LintParams means existing code isn't affected
- Only embedded language handlers populate it

### ✅ Future-Proof
- When Vue/Svelte/Astro need template scanning, they just:
  1. Populate `original_source_text` in their lint function
  2. Add their scanning logic to `finish()` method
  3. Done!

### ✅ Minimal Changes
- One field added to `LintParams`
- One service type created
- Conditional logic in semantic model builder

## How Other Languages Can Use This

### Vue Example (Future)

```rust
// In crates/biome_service/src/file_handlers/vue.rs
fn lint(mut params: LintParams) -> LintResults {
    if let Some(text) = get_original_text(&params.parse) {
        params.original_source_text = Some(text.to_string());
    }
    javascript::lint(params)
}

// In crates/biome_js_analyze/src/services/semantic.rs
if embedding_kind.is_vue() {
    add_vue_template_references(&mut builder, &source_text);
}
```

### Svelte Example (Future)

```rust
// Similar pattern for Svelte
if embedding_kind.is_svelte() {
    add_svelte_template_references(&mut builder, &source_text);
}
```

## Current Status

### ✅ Completed
- Generic `OriginalSourceText` service created
- Semantic model builder checks for service
- Extensible architecture for all embedded languages

### ❌ NOT Completed (Blocked by missing original text access)
- Need to add `original_source_text` to `LintParams`
- Need to populate it in glimmer handler
- Need to inject service in javascript handler

### 🤔 Challenge
The parse tree doesn't store the original text, only the transformed AST.
Need to find how to access original text in `lint()` function.

## Alternative: Store in Parse Metadata

Another approach would be to store the original text as metadata in the `AnyParse` structure:

```rust
// When parsing in glimmer.rs
let mut parse = parse_js_with_cache(...);
parse.store_metadata("original_source", text);  // Hypothetical API

// Later in lint
if let Some(original) = params.parse.get_metadata("original_source") {
    params.original_source_text = Some(original);
}
```

This would require modifications to the parser's metadata system.

## Recommendation

The cleanest solution is **adding `original_source_text` to `LintParams`** because:
1. It's explicit and clear
2. Doesn't require parser changes
3. Easy to trace through the codebase
4. Works for all embedded languages

The challenge is getting access to the original text in the `lint()` function, since the `parse` parameter only has the transformed AST.

## Next Steps

1. Investigate how to access original source text in `lint()` function
2. Add `original_source_text` field to `LintParams`
3. Update glimmer handler to populate it
4. Update javascript handler to create service from it
5. Test end-to-end
6. Document pattern for Vue/Svelte/Astro maintainers
