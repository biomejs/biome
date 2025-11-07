# TDD Workflow Document: Implementing Ember Lint Rules in Biome

## Overview

This document provides a step-by-step Test-Driven Development (TDD) workflow for implementing Ember lint rules in Biome. Follow this process for every rule to ensure quality, prevent regressions, and maintain clear documentation.

---

## The TDD Cycle

```
RED → GREEN → REFACTOR
 ↓      ↓        ↓
Test → Code → Improve → Repeat
```

1. **RED**: Write failing tests first
2. **GREEN**: Write minimal code to pass tests
3. **REFACTOR**: Improve code while keeping tests green

---

## Phase 1: Setup (One-time per rule)

### Step 1.1: Choose Your Rule

Pick a rule from the prioritized list. Start with Tier 1 (easy) rules.

**Example**: `noMixins` from eslint-plugin-ember

### Step 1.2: Research the Original Rule

```bash
# Open the original implementation
open https://github.com/ember-cli/eslint-plugin-ember/blob/master/lib/rules/no-mixins.js

# Review:
# - What patterns does it detect?
# - What are edge cases?
# - Does it have auto-fixes?
# - What are the test cases?
```

**Document findings**:
```markdown
## noMixins Research

**Purpose**: Prevent usage of Ember mixins (deprecated pattern)

**Detects**:
- `import Mixin from '@ember/object/mixin'`
- `import { Mixin } from '@ember/object/mixin'`
- `Mixin.create()` calls

**Edge Cases**:
- Renamed imports: `import MyMixin from '@ember/object/mixin'`
- Destructured imports
- Namespace imports: `import * as EmberObject from '@ember/object/mixin'`

**Auto-fix**: No (requires manual refactoring)

**Severity**: Warning (recommended rule)
```

---

## Phase 2: RED - Write Failing Tests

### Step 2.1: Create Test Directory

```bash
# Create test directory structure
mkdir -p crates/biome_js_analyze/tests/specs/nursery/noMixins
cd crates/biome_js_analyze/tests/specs/nursery/noMixins
```

### Step 2.2: Write Invalid Cases (Should Trigger Rule)

Create test files for patterns that SHOULD be flagged:

```bash
# File: invalid.js
cat > invalid.js << 'EOF'
import Mixin from '@ember/object/mixin';

export default Mixin.create({
  // This should trigger noMixins
});
EOF
```

```bash
# File: invalidDestructured.js
cat > invalidDestructured.js << 'EOF'
import { Mixin } from '@ember/object/mixin';

export default Mixin.create({});
EOF
```

```bash
# File: invalidRenamed.js
cat > invalidRenamed.js << 'EOF'
import MyMixin from '@ember/object/mixin';

export default MyMixin.create({});
EOF
```

```bash
# File: invalidNamespace.js
cat > invalidNamespace.js << 'EOF'
import * as EmberMixin from '@ember/object/mixin';

export default EmberMixin.Mixin.create({});
EOF
```

### Step 2.3: Write Valid Cases (Should NOT Trigger Rule)

Create test files for patterns that should be ALLOWED:

```bash
# File: valid.js
cat > valid.js << 'EOF'
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  // Modern component - no mixins
}
EOF
```

```bash
# File: validComposition.js
cat > validComposition.js << 'EOF'
// Using composition instead of mixins
import { action } from '@ember/object';

export default class MyComponent {
  @action
  handleClick() {}
}
EOF
```

```bash
# File: validThirdParty.js
cat > validThirdParty.js << 'EOF'
// Third-party library named 'mixin' (not Ember)
import mixin from 'lodash/mixin';

export default mixin({}, { a: 1 });
EOF
```

### Step 2.4: Run Tests (Expect Failure)

```bash
# Run the test suite
cargo test no_mixins

# Expected output:
# Error: Rule 'noMixins' not found
# or: Test failed - rule not implemented
```

**✅ Checkpoint**: Tests fail because rule doesn't exist yet. This is CORRECT.

---

## Phase 3: GREEN - Implement the Rule

### Step 3.1: Create Rule File

```bash
# Create the rule file
touch crates/biome_js_analyze/src/lint/nursery/no_mixins.rs
```

### Step 3.2: Write Rule Declaration

```rust
// File: crates/biome_js_analyze/src/lint/nursery/no_mixins.rs

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsImport;
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow the use of Ember mixins.
    ///
    /// Mixins are deprecated in modern Ember. Use composition patterns instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import Mixin from '@ember/object/mixin';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import Component from '@glimmer/component';
    /// ```
    ///
    pub NoMixins {
        version: "next",
        name: "noMixins",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintEmber("no-mixins")],
    }
}

impl Rule for NoMixins {
    type Query = Ast<JsImport>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();

        // Get import source
        let source = import.import_clause()?.source().ok()?;
        let source_text = source.inner_string_text().ok()?;

        // Check if importing from @ember/object/mixin
        if source_text.text() == "@ember/object/mixin" {
            return Some(import.range());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Using mixins is deprecated in modern Ember."
                },
            )
            .note(markup! {
                "Mixins can be replaced with composition patterns, utility functions, or native JavaScript features."
            })
            .note(markup! {
                "See the Ember Octane migration guide for more information."
            })
        )
    }
}
```

### Step 3.3: Register the Rule

```rust
// File: crates/biome_js_analyze/src/lint/nursery.rs

// Add to the declare_group! macro:
declare_group! {
    pub Nursery {
        name: "nursery",
        rules: [
            // ... existing rules ...
            self::no_mixins::NoMixins,
        ]
    }
}

// Add module declaration:
pub mod no_mixins;
```

### Step 3.4: Run Tests Again

```bash
# Run tests
cargo test no_mixins

# First run will create snapshots
# Output:
# test specs::no_mixins::invalid ... ok
# test specs::no_mixins::valid ... ok
```

### Step 3.5: Review Snapshots

```bash
# Review generated snapshots
cargo insta review

# Interactive prompt will show:
# - The source code
# - The diagnostic message
# - The formatting

# For each snapshot:
# - [a]ccept if correct
# - [r]eject if incorrect
# - [s]kip to decide later
```

**Example snapshot review**:
```
Reviewing 1/4 snapshots

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
invalid.js
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Input
import Mixin from '@ember/object/mixin';

export default Mixin.create({});

# Diagnostics
invalid.js:1:1 lint/nursery/noMixins ━━━━━━━━━━━━━━

  ✖ Using mixins is deprecated in modern Ember.

  > 1 │ import Mixin from '@ember/object/mixin';
      │ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    2 │
    3 │ export default Mixin.create({});

  ℹ Mixins can be replaced with composition patterns.

  ℹ See the Ember Octane migration guide.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[a]ccept [r]eject [s]kip [q]uit
```

**Press `a` to accept if the diagnostic is correct.**

### Step 3.6: Verify All Tests Pass

```bash
# Run tests again to confirm
cargo test no_mixins

# Expected output:
# test result: ok. X passed; 0 failed
```

**✅ Checkpoint**: All tests pass. Rule is working!

---

## Phase 4: REFACTOR - Improve & Extend

### Step 4.1: Add Edge Cases

Identify scenarios you haven't tested yet:

```bash
# File: invalidMultipleImports.js
cat > invalidMultipleImports.js << 'EOF'
import Component from '@glimmer/component';
import Mixin from '@ember/object/mixin';

export default class MyComponent extends Component {}
EOF
```

```bash
# File: invalidWithOtherImports.js
cat > invalidWithOtherImports.js << 'EOF'
import { computed } from '@ember/object';
import { Mixin } from '@ember/object/mixin';
import Service from '@ember/service';

export default Mixin.create({});
EOF
```

### Step 4.2: Run Tests for Edge Cases

```bash
cargo test no_mixins

# Review new snapshots
cargo insta review
```

### Step 4.3: Refactor Implementation (If Needed)

If tests reveal issues, refactor the implementation:

```rust
// Example: Improve to handle more cases
fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    let import = ctx.query();

    let source = import.import_clause()?.source().ok()?;
    let source_text = source.inner_string_text().ok()?;
    let text = source_text.text();

    // More flexible matching
    if text == "@ember/object/mixin" || text.starts_with("@ember/object/mixin/") {
        return Some(import.range());
    }

    None
}
```

### Step 4.4: Ensure Tests Still Pass

```bash
# After refactoring, always run tests
cargo test no_mixins

# Should still pass
```

**✅ Checkpoint**: Tests pass after refactoring. Code is improved.

---

## Phase 5: Document & Finalize

### Step 5.1: Review Documentation

Ensure inline docs are complete and accurate:

```rust
declare_lint_rule! {
    /// Disallow the use of Ember mixins.
    ///
    /// Mixins are a deprecated pattern in Ember. They have been replaced by:
    /// - Composition patterns
    /// - Utility functions
    /// - Native JavaScript classes with inheritance
    /// - Decorators
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import Mixin from '@ember/object/mixin';
    ///
    /// export default Mixin.create({
    ///   mixinMethod() {}
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import Component from '@glimmer/component';
    ///
    /// export default class MyComponent extends Component {
    ///   componentMethod() {}
    /// }
    /// ```
    ///
    pub NoMixins {
        version: "next",
        name: "noMixins",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintEmber("no-mixins")],
    }
}
```

### Step 5.2: Run Full Test Suite

```bash
# Run ALL tests to ensure no regressions
cargo test

# Or run just the analyzer tests
cargo test -p biome_js_analyze
```

### Step 5.3: Format and Lint

```bash
# Format Rust code
just f

# Lint Rust code
just l

# Generate any needed code
just gen-analyzer
```

### Step 5.4: Create a Commit

```bash
git add crates/biome_js_analyze/src/lint/nursery/no_mixins.rs
git add crates/biome_js_analyze/src/lint/nursery.rs
git add crates/biome_js_analyze/tests/specs/nursery/noMixins/

git commit -m "feat(analyzer): implement noMixins rule

Disallow the use of Ember mixins, which are deprecated in modern Ember.

Test cases:
- Basic mixin import
- Destructured import
- Renamed import
- Namespace import
- Valid alternatives

Closes #XXX"
```

---

## Complete TDD Checklist Template

Use this checklist for EVERY rule:

```markdown
## Rule: [RULE_NAME]

### Phase 1: Setup ⬜
- [ ] Rule selected from priority list
- [ ] Original implementation reviewed
- [ ] Research notes documented
- [ ] Patterns and edge cases identified

### Phase 2: RED - Write Tests ⬜
- [ ] Test directory created
- [ ] 3-5 invalid test cases written
- [ ] 3-5 valid test cases written
- [ ] Edge cases identified
- [ ] Tests run and fail as expected

### Phase 3: GREEN - Implement ⬜
- [ ] Rule file created
- [ ] declare_lint_rule! written
- [ ] Rule type definitions added
- [ ] run() method implemented
- [ ] diagnostic() method implemented
- [ ] action() method implemented (if applicable)
- [ ] Rule registered in module
- [ ] Tests run and pass
- [ ] Snapshots reviewed and accepted

### Phase 4: REFACTOR - Improve ⬜
- [ ] Additional edge cases added
- [ ] Implementation refined
- [ ] All tests still pass
- [ ] Code is clean and well-documented

### Phase 5: Finalize ⬜
- [ ] Documentation complete
- [ ] Full test suite passes
- [ ] Code formatted (just f)
- [ ] Code linted (just l)
- [ ] Analyzer generated (just gen-analyzer)
- [ ] Committed with clear message

### Quality Checks ✅
- [ ] No false positives
- [ ] No false negatives
- [ ] Clear diagnostic messages
- [ ] Helpful notes/suggestions
- [ ] Examples in documentation
- [ ] Links to migration guides
```

---

## Daily TDD Workflow

### Morning Routine

```bash
# 1. Pick your rule for the day
# Check the priority list

# 2. Create a branch
git checkout -b feat/no-mixins

# 3. Research (30 minutes)
# Review original rule, document findings

# 4. Write tests (1-2 hours)
# Create all test cases FIRST
# Run tests - they should FAIL

# Lunch break ☕
```

### Afternoon Routine

```bash
# 5. Implement rule (2-3 hours)
# Write minimal code to pass tests
# Review snapshots
# Iterate until all tests pass

# 6. Refactor (1 hour)
# Add edge cases
# Improve code quality
# Ensure tests still pass

# 7. Finalize (30 minutes)
# Documentation, formatting, commit

# End of day: 1 rule complete! 🎉
```

---

## Weekly TDD Cadence

### Monday
- Plan week's rules (5 rules)
- Review test patterns from last week
- Set up branches

### Tuesday-Thursday
- Implement 1-2 rules per day
- Morning: Write tests
- Afternoon: Implement + refactor

### Friday
- Code review
- Update documentation
- Refactor common patterns
- Plan next week

---

## TDD Best Practices

### DO ✅

1. **Write tests first, always**
   - No exceptions
   - Tests are specifications

2. **Start with simple cases**
   - Basic invalid case
   - Basic valid case
   - Add complexity gradually

3. **Review snapshots carefully**
   - Check diagnostic messages
   - Verify line numbers
   - Ensure clarity

4. **Run tests frequently**
   - After every small change
   - Use `cargo test --watch` if available

5. **Commit test + implementation together**
   - Tests prove the rule works
   - Makes review easier

6. **Add tests for bug fixes**
   - Found a bug? Write a test first
   - Fix the bug
   - Test prevents regression

### DON'T ❌

1. **Don't implement before tests**
   - Defeats the purpose of TDD
   - Leads to undertested code

2. **Don't accept snapshots blindly**
   - Actually read them
   - Verify diagnostics are helpful

3. **Don't skip edge cases**
   - They cause bugs in production
   - Add them to your test suite

4. **Don't ignore failing tests**
   - Fix immediately
   - Never commit broken tests

5. **Don't forget to refactor**
   - Green doesn't mean done
   - Improve code quality

---

## TDD for Different Rule Types

### Simple Pattern Matching Rule

**Example**: `noOldShims`

```bash
# Tests (30 minutes)
# - Invalid: old shim imports
# - Valid: new module imports

# Implementation (1 hour)
# - Pattern match on import source
# - Simple string comparison

# Total: ~1.5 hours
```

### Semantic Analysis Rule

**Example**: `noUnusedServices`

```bash
# Tests (1 hour)
# - Invalid: injected but never used
# - Valid: injected and used in methods
# - Edge: used in templates

# Implementation (3 hours)
# - Query service decorators
# - Track references in class
# - Check template usage

# Total: ~4 hours
```

### Cross-Language Rule

**Example**: `requireTaglessComponents`

```bash
# Tests (1.5 hours)
# - Invalid: template with wrapper div
# - Valid: template without wrapper
# - Edge: template-only components

# Implementation (4 hours)
# - Detect component class
# - Parse template
# - Check root elements

# Total: ~5.5 hours
```

---

## Measuring TDD Success

### Rule Quality Metrics

```markdown
## Rule: noMixins

**Test Coverage**:
- Invalid cases: 6
- Valid cases: 4
- Edge cases: 3
- Total: 13 test cases ✅

**Quality**:
- All tests pass: ✅
- No false positives: ✅
- No false negatives: ✅
- Clear diagnostics: ✅
- Documentation complete: ✅

**Performance**:
- Test execution: 0.03s ✅
- Snapshot size: 2.1KB ✅
```

### Weekly Progress

```markdown
## Week 3 Summary

**Rules Completed**: 5
1. noMixins (13 tests) ✅
2. noOldShims (8 tests) ✅
3. noClassicClasses (11 tests) ✅
4. requireButtonType (9 tests) ✅
5. noAccesskey (6 tests) ✅

**Total Test Cases**: 47
**All Tests Passing**: ✅
**Code Coverage**: 100%
**Documentation**: Complete

**Blockers**: None
**Next Week Goal**: 5 more rules
```

---

## Troubleshooting TDD Issues

### Issue: Tests won't run

```bash
# Check rule is registered
grep -r "no_mixins" crates/biome_js_analyze/src/lint/

# Rebuild
cargo build

# Try again
cargo test no_mixins
```

### Issue: Snapshots look wrong

```bash
# Reject bad snapshots
cargo insta reject

# Fix implementation
# Run tests again
cargo test no_mixins

# Review new snapshots
cargo insta review
```

### Issue: False positives/negatives

```bash
# Add test case that demonstrates the issue
cat > tests/specs/nursery/noMixins/edge_case.js << 'EOF'
// Your problematic code
EOF

# Run tests - should fail
cargo test no_mixins

# Fix implementation
# Run tests - should pass
```

---

## Summary

**The TDD cycle for every rule**:

1. **RED**: Write failing tests (~1-2 hours)
2. **GREEN**: Implement to pass tests (~2-4 hours)
3. **REFACTOR**: Improve and extend (~1 hour)
4. **FINALIZE**: Document and commit (~30 min)

**Total per rule**: 4.5-7.5 hours depending on complexity

**Expected output**: 1 rule per day per developer

**Quality assurance**: Every rule has comprehensive tests

Follow this document for all 218 rules to ensure consistent quality!
