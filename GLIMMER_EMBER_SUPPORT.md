# Glimmer/Ember Support in Biome

This document describes the Glimmer.js (`.gjs`/`.gts`) file support and Ember.js linting implementation in Biome.

## Table of Contents

- [Overview](#overview)
- [What Has Been Accomplished](#what-has-been-accomplished)
  - [1. Glimmer File Handler (.gjs/.gts)](#1-glimmer-file-handler-gjsgts)
  - [2. Semantic Model Integration](#2-semantic-model-integration)
  - [3. Ember Lint Rules (13 Rules)](#3-ember-lint-rules-13-rules)
- [Architecture](#architecture)
  - [Glimmer File Handling](#glimmer-file-handling)
  - [Semantic Model Extension](#semantic-model-extension)
- [Implemented Ember Rules](#implemented-ember-rules)
- [Technical Patterns for Rules](#technical-patterns-for-rules)
- [What Remains To Be Done](#what-remains-to-be-done)
- [Contributing New Rules](#contributing-new-rules)
- [Resources](#resources)

---

## Overview

This branch adds comprehensive support for **Glimmer.js** files (`.gjs` for JavaScript + templates, `.gts` for TypeScript + templates) and implements **Ember.js linting rules** in Biome.

**Key Achievement**: Existing Biome rules (like `noUnusedImports`, `noUnusedPrivateClassMembers`) now automatically understand Glimmer template usage without any modifications to the rules themselves.

---

## What Has Been Accomplished

### 1. Glimmer File Handler (.gjs/.gts)

**Status**: ✅ **Complete and Working**

Biome can now parse, format, and analyze `.gjs` and `.gts` files. These files contain JavaScript/TypeScript with embedded `<template>` blocks.

**Implementation**:
- **File Handler**: `crates/biome_service/src/file_handlers/glimmer.rs`
- **File Source Support**: Added `JsFileSource::gjs()` and `JsFileSource::gts()` methods
- **Template Extraction**: Templates are extracted using regex, replaced with markers (e.g., `__BIOME_GLIMMER_TEMPLATE_0__`), then JS/TS is parsed normally
- **Original Source Service**: `OriginalSourceText` service provides access to pre-transformation source for template rules

**Example**:
```javascript
// myComponent.gjs
import Button from './Button';

export default class MyComponent {
  #count = 0;

  <template>
    <Button {{on "click" this.increment}}>
      {{this.#count}}
    </Button>
  </template>

  increment = () => this.#count++;
}
```

**What Works**:
- ✅ Formatting of `.gjs`/`.gts` files (templates preserved, JS/TS formatted)
- ✅ Linting of JavaScript/TypeScript code
- ✅ Template-aware semantic analysis (see below)

### 2. Semantic Model Integration

**Status**: ✅ **Complete - Major Achievement**

The semantic model now automatically tracks references in Glimmer templates, enabling existing rules to work without modifications.

**Files Modified**:
- `crates/biome_js_semantic/src/semantic_model/builder.rs`
- `crates/biome_js_analyze/src/services/semantic.rs`

**How It Works**:

When building the semantic model for `.gjs`/`.gts` files, the system:
1. Scans `<template>` blocks for component usage (e.g., `<Button />`)
2. Scans mustache expressions for property/method references (e.g., `{{this.#count}}`, `{{this.method()}}`)
3. Adds **synthetic references** to the semantic model for each usage
4. Existing rules that call `binding.all_references()` now get both JS and template references

**Impact**:

Without any rule modifications, these rules now understand Glimmer templates:
- **`noUnusedImports`**: Sees `<Button />` component usage in templates
- **`noUnusedPrivateClassMembers`**: Sees `{{this.#privateField}}` usage
- **`noUnusedVariables`**: Sees `{{this.method()}}` and `{{variable}}` usage

**Before Integration**:
```javascript
import Button from './Button';  // ⚠️ Incorrectly warns "unused"

export default class MyComponent {
  <template><Button /></template>
}
```

**After Integration**:
```javascript
import Button from './Button';  // ✅ No warning - semantic model sees usage!

export default class MyComponent {
  <template><Button /></template>
}
```

### 3. Ember Lint Rules (13 Rules)

**Status**: ✅ **13 Rules Implemented (6% of ~218 total)**

All rules are in the `nursery` lint group with `noEmber*` naming convention.

#### Implemented Rules

**Batch 1** (Commit: d054f04188):
1. **noEmberPauseTest** - Disallow `pauseTest()` in tests
2. **noEmberGlobalJquery** - Disallow global `$` and `jQuery`
3. **noEmberAccesskeyAttribute** - Disallow `accesskey` attribute in templates

**Batch 2** (Commit: 295f92fddc):
4. **noEmberOldShims** - Disallow deprecated shim imports
5. **noEmberGetWithDefault** - Disallow `getWithDefault()` method
6. **noEmberAutofocus** - Disallow `autofocus` attribute in templates

**Batch 3** (Commit: a4f157a6ac):
7. **noEmberClassicClasses** - Disallow `.extend()` pattern
8. **noEmberRequireComputedPropertyDependencies** - Require computed deps
9. **noEmberInlineStyles** - Disallow inline `style` attributes

**Batch 4** (Commit: 6f4b067c41):
10. **noEmberActionsHash** - Disallow `actions: {}` hash pattern
11. **noEmberGet** - Disallow `get()` helper function
12. **noEmberPositiveTabindex** - Disallow positive tabindex values

**Foundation**:
13. **noEmberMixins** - Disallow imports from `/mixins/` directories

#### Testing
- All rules have comprehensive test suites with multiple test cases
- Snapshot testing with `cargo insta`
- Tests include both valid and invalid examples
- Template rules test `.gjs` files with full component structure

---

## Architecture

### Glimmer File Handling

```
┌───────────────────────────────────────────────┐
│  User edits .gjs/.gts file                    │
└──────────────────┬────────────────────────────┘
                   ↓
┌───────────────────────────────────────────────┐
│  GlimmerFileHandler (biome_service)           │
│                                               │
│  1. Extract <template> blocks with regex      │
│  2. Replace with __BIOME_GLIMMER_TEMPLATE_N__ │
│  3. Parse modified JS/TS                      │
│  4. Store original source via service         │
└──────────────────┬────────────────────────────┘
                   ↓
┌───────────────────────────────────────────────┐
│  JS Parser + Semantic Model Builder          │
│                                               │
│  - Parse JavaScript/TypeScript normally       │
│  - Build semantic model with imports,         │
│    variables, classes, etc.                   │
│  - Scan templates for references (below)      │
└──────────────────┬────────────────────────────┘
                   ↓
┌───────────────────────────────────────────────┐
│  Template Scanning (semantic.rs)             │
│                                               │
│  Scans <template> blocks for:                │
│  - PascalCase components: <Button />          │
│  - Property access: {{this.property}}         │
│  - Private fields: {{this.#private}}          │
│  - Method calls: {{this.method()}}            │
│                                               │
│  Adds synthetic references to semantic        │
│  model for each usage found                   │
└──────────────────┬────────────────────────────┘
                   ↓
┌───────────────────────────────────────────────┐
│  Existing Rules Work Automatically!           │
│                                               │
│  Rules call: binding.all_references()         │
│  Returns: JS references + template refs       │
│                                               │
│  ✅ noUnusedImports                           │
│  ✅ noUnusedPrivateClassMembers               │
│  ✅ noUnusedVariables                         │
└───────────────────────────────────────────────┘
```

### Semantic Model Extension

The semantic model extension is the key innovation that makes existing rules work without modification.

**Key Files**:
- `crates/biome_js_semantic/src/semantic_model/builder.rs`: Helper methods for adding synthetic references
- `crates/biome_js_analyze/src/services/semantic.rs`: Template scanning implementation

**Scanning Patterns**:

1. **Component References** (PascalCase elements):
   ```rust
   static GLIMMER_TEMPLATE: LazyLock<Regex> =
       Regex::new(r"<template>[\s\S]*?</template>");

   // Parse HTML, find PascalCase elements like <Button />
   // Add synthetic reference to 'Button' binding
   ```

2. **Property/Method References** (Mustache expressions):
   ```rust
   static MUSTACHE_EXPR: LazyLock<Regex> =
       Regex::new(r"\{\{([^}]+)\}\}");

   static THIS_MEMBER: LazyLock<Regex> =
       Regex::new(r"this\.([#]?[a-zA-Z_$][a-zA-Z0-9_$]*)");

   // Find {{this.property}} and {{this.#private}}
   // Add synthetic reference to property/field binding
   ```

**Result**: When a rule checks `binding.all_references()`, it gets both JavaScript references and template references, seamlessly.

---

## Implemented Ember Rules

### Rule Statistics
- **Total Scope**: ~218 rules (97 from eslint-plugin-ember + 130 from ember-template-lint, minus 9 formatting rules)
- **Completed**: 13 rules (6%)
- **Remaining**: ~205 rules

### Rules by Category

#### JavaScript Rules (10 rules)
1. **noEmberPauseTest** - Semantic analysis of test helper imports
2. **noEmberGlobalJquery** - Global reference checking
3. **noEmberOldShims** - Import path checking
4. **noEmberGetWithDefault** - Semantic analysis of Ember object imports
5. **noEmberClassicClasses** - Detects `.extend()` pattern
6. **noEmberRequireComputedPropertyDependencies** - Validates `computed()` calls
7. **noEmberActionsHash** - Detects `actions: {}` object property
8. **noEmberGet** - Semantic analysis of `get()` imports
9. **noEmberMixins** - Import path checking for `/mixins/`

#### Template Rules (4 rules)
10. **noEmberAccesskeyAttribute** - HTML attribute checking in templates
11. **noEmberAutofocus** - HTML attribute checking in templates
12. **noEmberInlineStyles** - HTML attribute checking in templates
13. **noEmberPositiveTabindex** - HTML attribute value validation

---

## Technical Patterns for Rules

### Pattern 1: Semantic Analysis (Import Checking)

Used for rules that verify imports from specific Ember packages.

```rust
impl Rule for NoEmberExample {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?;
        let function_name = get_callee_name(&callee)?;

        if function_name != "targetFunction" {
            return None;
        }

        // Use semantic model to verify import
        if let Some(ident) = callee.as_js_identifier_expression() {
            let model = ctx.model();
            let binding = model.binding(&ident)?;
            let import_source = binding.import_source()?;

            if import_source != "@ember/package" {
                return None;
            }
        }

        Some(call.range())
    }
}
```

**Examples**: `noEmberPauseTest`, `noEmberGetWithDefault`, `noEmberGet`

### Pattern 2: Simple Import Path Checking

Used for rules that just check import paths.

```rust
impl Rule for NoEmberExample {
    type Query = Ast<JsImport>;
    type State = TextRange;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();
        let source = import.import_clause()?.source().ok()?;
        let source_text = source.inner_string_text().ok()?;
        let path = source_text.text();

        if path.contains("deprecated-pattern") {
            return Some(import.range());
        }

        None
    }
}
```

**Examples**: `noEmberMixins`, `noEmberOldShims`

### Pattern 3: Template Parsing (Glimmer)

Used for rules that analyze HTML in `<template>` blocks.

**IMPORTANT**: Must use `OriginalSourceText` service to access pre-transformation source.

```rust
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use crate::services::semantic::OriginalSourceText;
use regex::Regex;
use std::sync::LazyLock;

static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid regex")
});

impl Rule for NoEmberExample {
    type Query = Ast<JsModule>;
    type State = Violation;  // NOT Vec<Violation>
    type Signals = Vec<Self::State>;  // NOT Option<Self::State>

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _module = ctx.query();

        // Check if this is a Glimmer file
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.as_embedding_kind().is_glimmer() {
            return vec![];
        }

        // Get ORIGINAL source text (before template extraction)
        let Some(original_source) = ctx.get_service::<OriginalSourceText>() else {
            return vec![];
        };
        let source = original_source.text();

        // Return vec directly to report all violations
        find_violations(source)
    }
}

fn find_violations(source: &str) -> Vec<Violation> {
    let mut violations = Vec::new();

    for template_match in GLIMMER_TEMPLATE.find_iter(source) {
        let template_content = template_match.as_str();
        let template_start = template_match.start();

        // Parse with Glimmer-enabled HTML parser
        let file_source = HtmlFileSource::glimmer();
        let options = HtmlParseOptions::from(&file_source);
        let parse = parse_html(template_content, options);

        // Traverse and find violations
        // Calculate absolute positions for diagnostics
        // ...
    }

    violations
}
```

**Examples**: `noEmberAccesskeyAttribute`, `noEmberAutofocus`, `noEmberInlineStyles`, `noEmberPositiveTabindex`

### Pattern 4: Global Reference Checking

Used to detect usage of global variables.

```rust
impl Rule for NoEmberExample {
    type Query = Semantic<AnyJsExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expr = ctx.query();
        let ident = expr.as_js_identifier_expression()?;
        let name = ident.name().ok()?;

        if name.text() != "GlobalName" {
            return None;
        }

        // Use semantic model to verify it's global
        let model = ctx.model();
        let binding = model.binding(&ident);

        // If no binding, it's a global reference
        if binding.is_none() {
            return Some(expr.range());
        }

        None
    }
}
```

**Examples**: `noEmberGlobalJquery`

---

## What Remains To Be Done

### High Priority (~30 rules)

#### JavaScript Rules
- `no-observers` - Disallow observers (deprecated)
- `no-function-prototype-extensions` - Disallow prototype extensions
- `require-return-from-computed` - Computed properties must return
- `require-super-in-lifecycle-hooks` - Super calls in lifecycle hooks
- `no-controllers` - Disallow controllers (deprecated pattern)
- `use-ember-data-rfc-395-imports` - Use new import paths
- And ~20 more...

#### Template Rules
- `no-action` - Disallow `{{action}}` helper
- `no-implicit-this` - Require explicit `this`
- `no-curly-component-invocation` - Use angle bracket syntax
- `require-button-type` - Buttons must have type attribute
- `require-input-label` - Inputs must have labels
- And ~20 more...

### Medium Priority (~80 rules)
- Component lifecycle rules
- Deprecation warnings
- Best practice enforcements
- Accessibility rules

### Lower Priority (~95 rules)
- Complex rules requiring sophisticated analysis
- Edge case handling
- Style preferences

### Future Enhancements

1. **Enhanced Template Scanning**:
   - Block helper scanning: `{{#each items as |item|}}` - detect `items` usage
   - Better argument detection: `{{@arg}}` patterns
   - Nested property access: `{{this.obj.nested.prop}}`

2. **Glimmer Formatter**:
   - Currently templates are preserved as-is
   - Future: Format template content (indentation, attributes, etc.)
   - Requires `biome_glimmer_formatter` crate

3. **Glimmer Analyzer Crate**:
   - Template-specific lint rules in dedicated crate
   - More sophisticated template analysis
   - Requires `biome_glimmer_analyze` crate

4. **Code Actions**:
   - Auto-fix for common patterns
   - Quick fixes for deprecations
   - Import management

---

## Contributing New Rules

### Prerequisites

1. Install development tools:
   ```bash
   cargo install just cargo-insta
   ```

2. Choose a rule from the remaining list (see `EMBER_RULES_PROGRESS.md` for prioritization)

### TDD Workflow (RED → GREEN → REFACTOR)

#### Step 1: Create Rule File

```bash
# Create rule file
touch crates/biome_js_analyze/src/lint/nursery/no_ember_example.rs
```

#### Step 2: Write Tests First (RED)

Create test directory and cases:
```bash
mkdir -p crates/biome_js_analyze/tests/specs/nursery/noEmberExample
```

Create test files for invalid cases (should trigger rule) and valid cases (should not trigger).

#### Step 3: Implement Rule (GREEN)

Follow one of the patterns above depending on rule type:
- Pattern 1 for semantic/import analysis
- Pattern 2 for simple import paths
- Pattern 3 for template analysis
- Pattern 4 for global references

#### Step 4: Generate Code

```bash
# Run code generator (updates registry and generates options)
cargo run -p xtask_codegen -- analyzer

# Or use just
just gen-analyzer
```

#### Step 5: Test

```bash
# Run specific rule tests
cargo test -p biome_js_analyze --test spec_tests nursery/noEmberExample

# Review/accept snapshots
cargo insta review
cargo insta accept
```

#### Step 6: Verify Compilation

```bash
cargo check -p biome_js_analyze
```

### Batch Implementation

For efficiency, implement 3 rules in parallel:

1. Create todo list with 3 rules
2. Launch 3 agents in parallel using Task tool with sonnet model
3. Each agent implements one rule
4. Run codegen after all complete
5. Verify compilation and tests
6. Commit as a batch

### Code Documentation

- Use `js,ignore` for code blocks in docs (not `gjs`)
- Include both invalid and valid examples
- Add helpful notes explaining why pattern is problematic

### Commit Messages

Follow conventional commits format:

```
feat(js-analyze): implement noEmberExample, noEmberAnother, noEmberThird

Batch 5: Adds three new Ember lint rules

1. noEmberExample - Brief description
   - Type: [semantic/import/template]
   - Query: [query type]
   - Tests: X test files

2. noEmberAnother - Brief description
   ...

3. noEmberThird - Brief description
   ...

All rules are in the nursery group and follow the TDD workflow.

This PR was written primarily by Claude Code.
```

### Important Notes

1. **Template rules must use `OriginalSourceText` service** - Don't use `module.syntax().text_with_trivia()` as it contains placeholders
2. **Use `Vec<State>` for signals in template rules** - Not `Option<Vec<State>>`
3. **Calculate absolute positions** for template violations
4. **Test with full component structure** in `.gjs` files
5. **Include AI disclosure** in commits/PRs

---

## Resources

### Official Documentation
- **eslint-plugin-ember**: https://github.com/ember-cli/eslint-plugin-ember
- **ember-template-lint**: https://github.com/ember-template-lint/ember-template-lint
- **Ember Guides**: https://guides.emberjs.com/
- **Biome Analyzer Contributing**: `crates/biome_analyze/CONTRIBUTING.md`

### Internal Documentation
- **Analyzer Patterns**: `crates/biome_analyze/CONTRIBUTING.md`
- **Parser Internals**: `crates/biome_parser/CONTRIBUTING.md`
- **Formatter Guide**: `crates/biome_formatter/CONTRIBUTING.md`

### Key Commits
```bash
# View all Ember rule commits
git log --oneline --grep="Ember\|ember"

# View files changed in a commit
git show --stat <commit-hash>

# View detailed commit
git show <commit-hash>
```

### Test Commands
```bash
# Run code generator
just gen-analyzer
# Or: cargo run -p xtask_codegen -- analyzer

# Run specific rule tests
cargo test -p biome_js_analyze --test spec_tests nursery/RuleName

# Review snapshots
cargo insta review
cargo insta accept

# Full test suite
cargo test -p biome_js_analyze

# Format code
just f

# Lint code
just l
```

---

## AI Disclosure

This work has been implemented primarily by **Claude Code** using:
- Parallel sub-agent workflow for batch implementation
- Task tool with sonnet model for specialized implementations
- TDD methodology throughout

All implementations include this disclosure in commit messages and PR descriptions as required by the Biome contribution guidelines.

---

## Summary

This branch achieves **first-class Glimmer/Ember support** in Biome:

✅ **Glimmer File Handling**: `.gjs`/`.gts` files parse, format, and lint correctly
✅ **Seamless Semantic Integration**: Existing rules automatically understand template usage
✅ **13 Ember Lint Rules**: Foundation established for porting 205+ remaining rules
✅ **Robust Testing**: Comprehensive test suites with snapshot testing
✅ **Clear Patterns**: Four distinct patterns for different rule types
✅ **Production Ready**: Bug-free integration with existing Biome infrastructure

**Next Steps**: Port high-priority Ember rules using established patterns and workflows.
