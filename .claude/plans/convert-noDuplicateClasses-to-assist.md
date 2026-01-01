# Plan: Convert `noDuplicateClasses` from Lint Rule to Assist Action

## Background

The maintainer (@ematipico) indicated that `noDuplicateClasses` should be an assist action rather than a lint rule. The reasoning:
- "Always safe fix" + "no valid use case" + "objectively problematic" = should be automatically fixed, not reported as a problem
- Assist actions are for mechanical transformations that are always beneficial
- Lint rules are for things that *might* be bugs requiring user attention

## Current State

| Language | Location | Status |
|----------|----------|--------|
| JS/JSX | `crates/biome_js_analyze/src/lint/nursery/no_duplicate_classes.rs` | Lint rule |
| HTML | `crates/biome_html_analyze/src/lint/nursery/no_duplicate_classes.rs` | Lint rule |

## Target State

| Language | Location | Status |
|----------|----------|--------|
| JS/JSX | `crates/biome_js_analyze/src/assist/source/no_duplicate_classes.rs` | Assist action |
| HTML | `crates/biome_html_analyze/src/assist/source/no_duplicate_classes.rs` | Assist action |

---

# Part 1: Convert JS Rule to Assist Action

## Step 1.1: Create the Assist Rule File

**File:** `crates/biome_js_analyze/src/assist/source/no_duplicate_classes.rs`

Key changes from lint rule:

```rust
// BEFORE (lint rule)
use biome_analyze::{declare_lint_rule, ...};
declare_lint_rule! {
    pub NoDuplicateClasses {
        version: "next",
        name: "noDuplicateClasses",
        language: "jsx",
        sources: &[RuleSource::EslintBetterTailwindcss("no-duplicate-classes")],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

// Diagnostic uses:
category!("lint/nursery/noDuplicateClasses")
ctx.metadata().action_category(ctx.category(), ctx.group())

// AFTER (assist action)
use biome_analyze::{declare_source_rule, ...};
use biome_diagnostics::{Applicability, category};

declare_source_rule! {
    pub NoDuplicateClasses {
        version: "next",
        name: "noDuplicateClasses",
        language: "jsx",
        sources: &[RuleSource::EslintBetterTailwindcss("no-duplicate-classes").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

// Diagnostic uses:
category!("assist/source/noDuplicateClasses")
rule_action_category!()
```

**Full implementation pattern** (based on `use_sorted_keys.rs`):

```rust
use crate::lint::nursery::use_sorted_classes::any_class_string_like::AnyClassStringLike;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource,
    context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_js_factory::make::{...};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;
use rustc_hash::FxHashSet;

use crate::JsRuleAction;

declare_source_rule! {
    /// Remove duplicate CSS classes.
    ///
    /// Duplicate CSS classes are redundant and may indicate copy-paste errors
    /// or merge conflict artifacts. This action automatically removes them,
    /// keeping only the first occurrence of each class.
    ///
    /// ## Examples
    ///
    /// ```js,expect_diff
    /// clsx("flex flex p-4 p-4 m-2");
    /// ```
    ///
    /// ```jsx,expect_diff
    /// <div class="flex flex p-4 p-4 m-2" />;
    /// ```
    ///
    pub NoDuplicateClasses {
        version: "next",
        name: "noDuplicateClasses",
        language: "jsx",
        sources: &[RuleSource::EslintBetterTailwindcss("no-duplicate-classes").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoDuplicateClasses {
    type Query = Ast<AnyClassStringLike>;
    type State = DuplicateClassState;
    type Signals = Option<Self::State>;
    type Options = UseSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        // ... same detection logic ...
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/noDuplicateClasses"),  // Changed category
            ctx.query().range(),
            markup! {
                "This class attribute contains duplicate classes."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // ... same fix logic ...

        Some(RuleAction::new(
            rule_action_category!(),  // Changed from ctx.metadata().action_category(...)
            Applicability::Always,
            markup! { "Remove duplicate classes." },
            mutation,
        ))
    }
}
```

## Step 1.2: Update Diagnostic Category

**File:** `crates/biome_diagnostics_categories/src/categories.rs`

```rust
// Remove:
"lint/nursery/noDuplicateClasses": "https://biomejs.dev/linter/rules/no-duplicate-classes",

// Add:
"assist/source/noDuplicateClasses": "https://biomejs.dev/assist/actions/no-duplicate-classes",
```

## Step 1.3: Remove from Lint Nursery

**Delete file:** `crates/biome_js_analyze/src/lint/nursery/no_duplicate_classes.rs`

The `just gen-analyzer` command will automatically update `lint/nursery.rs` to remove the reference.

## Step 1.4: Move Tests

**Move from:**
- `crates/biome_js_analyze/tests/specs/nursery/noDuplicateClasses/`

**Move to:**
- `crates/biome_js_analyze/tests/specs/source/noDuplicateClasses/`

Note: Test file naming convention for assists uses `expect_diff` in doctest examples rather than `expect_diagnostic`.

## Step 1.5: Run Codegen

```bash
just gen-analyzer
```

This will automatically:
- Update `assist/source.rs` to include the new rule
- Update `lint/nursery.rs` to remove the old rule
- Update `registry.rs` if needed

---

# Part 2: Create HTML Assist Infrastructure

HTML currently has **no assist infrastructure**. We need to create it.

## Step 2.1: Update Codegen to Include HTML Assist

**File:** `xtask/codegen/src/generate_analyzer.rs`

```rust
// BEFORE (line 53-58):
fn generate_html_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_html_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("lint", &mut analyzers, &base_path)?;

    update_html_registry_builder(analyzers)
}

// AFTER:
fn generate_html_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_html_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("lint", &mut analyzers, &base_path)?;
    generate_category("assist", &mut analyzers, &base_path)?;  // Add this line

    update_html_registry_builder(analyzers)
}
```

## Step 2.2: Create Directory Structure

```bash
mkdir -p crates/biome_html_analyze/src/assist/source
```

## Step 2.3: Create the HTML Assist Rule

**File:** `crates/biome_html_analyze/src/assist/source/no_duplicate_classes.rs`

```rust
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic,
    context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_html_factory::make::html_string;
use biome_html_syntax::{HtmlAttribute, HtmlString, inner_string_text};
use biome_rowan::{AstNode, BatchMutationExt};
use rustc_hash::FxHashSet;

use crate::HtmlRuleAction;

declare_source_rule! {
    /// Remove duplicate CSS classes from HTML class attributes.
    ///
    /// ## Examples
    ///
    /// ```html,expect_diff
    /// <div class="flex flex p-4 p-4 m-2"></div>
    /// ```
    ///
    pub NoDuplicateClasses {
        version: "next",
        name: "noDuplicateClasses",
        language: "html",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

// ... same pattern as JS, adapted for HTML AST ...
```

## Step 2.4: Update HTML lib.rs

**File:** `crates/biome_html_analyze/src/lib.rs`

Add the assist module declaration:

```rust
mod a11y;
mod assist;  // Add this line
mod lint;
pub mod options;
mod registry;
mod suppression_action;
```

## Step 2.5: Move HTML Tests

**Move from:**
- `crates/biome_html_analyze/tests/specs/nursery/noDuplicateClasses/`

**Move to:**
- `crates/biome_html_analyze/tests/specs/source/noDuplicateClasses/`

## Step 2.6: Run Codegen

```bash
just gen-analyzer
```

This will automatically generate:
- `crates/biome_html_analyze/src/assist.rs`
- `crates/biome_html_analyze/src/assist/source.rs`
- Update `crates/biome_html_analyze/src/registry.rs`

---

# Part 3: Update Changeset and Documentation

## Step 3.1: Update Changeset

**File:** `.changeset/no-duplicate-classes-rule.md`

```markdown
---
"@biomejs/biome": patch
---

Added the new assist action `noDuplicateClasses`.

Automatically removes duplicate CSS classes from class attributes and utility function calls.
Works in both JSX/JS and HTML files.

```jsx
// Before
<div class="flex flex p-4 p-4 m-2" />;

// After (auto-fixed)
<div class="flex p-4 m-2" />;
```
```

## Step 3.2: Update Doctest Examples

Change from `expect_diagnostic` to `expect_diff`:

```rust
// BEFORE (lint rule)
/// ```jsx,expect_diagnostic
/// <div class="flex flex" />;
/// ```

// AFTER (assist action)
/// ```jsx,expect_diff
/// <div class="flex flex" />;
/// ```
```

---

# Implementation Checklist

## JS Assist Conversion
- [ ] Create `crates/biome_js_analyze/src/assist/source/no_duplicate_classes.rs`
- [ ] Update diagnostic category in `categories.rs`
- [ ] Delete `crates/biome_js_analyze/src/lint/nursery/no_duplicate_classes.rs`
- [ ] Move tests from `specs/nursery/` to `specs/source/`
- [ ] Update doctest examples to use `expect_diff`

## HTML Assist Infrastructure
- [ ] Modify `xtask/codegen/src/generate_analyzer.rs` to add HTML assist
- [ ] Create `crates/biome_html_analyze/src/assist/source/` directory
- [ ] Create `crates/biome_html_analyze/src/assist/source/no_duplicate_classes.rs`
- [ ] Add `mod assist;` to `crates/biome_html_analyze/src/lib.rs`
- [ ] Move tests from `specs/nursery/` to `specs/source/`

## Final Steps
- [ ] Run `just gen-analyzer`
- [ ] Run `just f` (format)
- [ ] Run `cargo test -p biome_js_analyze -- no_duplicate_classes`
- [ ] Run `cargo test -p biome_html_analyze -- no_duplicate_classes`
- [ ] Update changeset
- [ ] Run `cargo clippy`

---

# Risk Assessment

| Risk | Mitigation |
|------|------------|
| HTML assist infrastructure doesn't exist | We create it following CSS/JSON patterns |
| Codegen may fail | Test incrementally; run codegen after each major step |
| Test structure may differ | Follow existing `source/` test patterns |
| Maintainer may want different approach | Ask for clarification before starting major work |

---

# Alternative: Ask Maintainer First

Before doing this work, consider asking:

> @ematipico Thanks for the guidance on making this an assist action!
>
> Before I refactor, I want to confirm:
> 1. Should this go in `assist/source/` (like `useSortedKeys`)?
> 2. HTML currently has no assist infrastructure - should I create it, or is there a preferred approach?
> 3. Should the `sources` field be kept (with `.inspired()`) or removed entirely?

This could save significant rework if the maintainer has a different vision.
