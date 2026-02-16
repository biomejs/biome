# PR #8960 Review Comments Summary

## Overview
- **Total Comments**: 94
- **Files Reviewed**: 29
- **Review Period**: 2026-02-04 to 2026-02-08
- **Status**: Addressed all major feedback

## Comments by Reviewer

| Reviewer | Count | Role |
|----------|-------|------|
| abossenbroek | 42 | Author (self-review & fixes) |
| coderabbitai[bot] | 30 | Automated code quality review |
| dyc3 | 20 | Core Contributor (main technical review) |
| changeset-bot[bot] | 1 | Automated changeset detection |
| codspeed-hq[bot] | 1 | Performance tracking |

## Key Files with Most Comments

### Code Files
1. `crates/biome_js_analyze/src/frameworks/playwright.rs` - 10 comments
2. `crates/biome_js_analyze/src/lint/nursery/no_playwright_skipped_test.rs` - 7 comments
3. `crates/biome_js_analyze/src/lint/nursery/no_playwright_missing_await.rs` - 6 comments
4. `crates/biome_js_analyze/src/lint/nursery/expect_playwright_expect.rs` - 6 comments
5. `crates/biome_js_analyze/src/lint/nursery/use_playwright_valid_describe_callback.rs` - 6 comments

### Changeset Files
- `.changeset/no-playwright-element-handle-fix.md` - 2 comments
- `.changeset/no-playwright-skipped-test-fix.md` - 2 comments
- `.changeset/no-playwright-useless-await.md` - 2 comments
- `.changeset/no-playwright-wait-for-selector-fix.md` - 2 comments
- `.changeset/no-playwright-wait-for-timeout-note.md` - 2 comments

## Major Review Feedback from dyc3

### 1. Naming & Rule Structure Issues
- **Rule Naming**: `expect_playwright_expect` doesn't follow naming guidelines (must start with `use` or `no`)
- **Resolution**: Renamed to follow conventions

### 2. Code Organization
- **Issue**: Duplicate/shared functionality across multiple rule files
- **Suggestion**: Move shared functionality to `crates/biome_js_analyze/src/frameworks/playwright.rs`
- **Affected Files**: 
  - `expect_playwright_expect.rs`
  - `use_playwright_expect.rs`

### 3. Performance Improvements
- **Issue**: String allocations with `.to_string()` in `no_playwright_skipped_test.rs`
- **Suggestion**: Use `inner_string_text()` and `TokenText` instead of `String` to avoid allocations
- **Multiple locations** needed optimization

### 4. Helper Function Parameter Types
- **Issue**: Helper functions using `&TokenText` when they could use `&str`
- **File**: `use_playwright_valid_describe_callback.rs`
- **Rule**: Use `&str` for helper functions, call `.text()` at call sites
- **Note**: CodeRabbit was instructed to add this to its learnings

### 5. Code Structure Standards
- **Issue**: Helper functions/structs/enums placement
- **Rule**: All helpers must go BELOW the `impl Rule` block
- **Exception**: Node unions used in rule queries can stay above for readability
- **File**: `use_playwright_valid_describe_callback.rs`

### 6. Diagnostic Message Ordering
- **Issue**: Diagnostic notes in wrong order
- **File**: `use_playwright_expect.rs` (line 97)
- **Standard Format**:
  1. WHAT is the error (main message)
  2. WHY is it an error (first note)
  3. HOW to fix it (second note)

### 7. Rule Consolidation
- **Issue**: `no_playwright_skipped_test.rs` may be redundant
- **Question**: Does `no_skipped_tests` already cover this functionality?
- **Suggestion**: Consider adding metadata to existing rule instead of new rule

### 8. Rule Domain Placement
- **Issue**: `no_playwright_conditional_expect.rs` is playwright-specific but similar rules exist in other domains
- **Similar Rules**: 
  - jest: `no-conditional-expect` (jest-community/eslint-plugin-jest)
  - vitest: `no-conditional-expect` (vitest-dev/eslint-plugin-vitest)
- **Suggestion**: Move to `test` domain, rename to `noConditionalExpect`, add jest/vitest as rule sources

### 9. Logic Clarity Issues
- **File**: `ast_utils.rs`
- **Issue**: Comment regarding async keyword detection logic unclear
- **Resolution**: Requested clarification/elaboration

### 10. Versioning Constraints
- **File**: `crates/biome_rule_options/src/no_skipped_tests.rs`
- **Issue**: Cannot add new options to rules in patch releases
- **Constraint**: New options must go in separate PR on `next` branch
- **Resolution**: These changes will need to be moved to next branch PR

### 11. Cross-Domain Rule Suggestions
- **File**: `use_playwright_expect.rs`
- **Observation**: Jest and Vitest almost certainly have equivalent rules
- **Action**: Consider adding those as rule sources in documentation

## Changeset Issues Fixed

Multiple changeset files had issues that were corrected:
1. **no-playwright-wait-for-timeout-note.md** - Entire changeset was wrong (fixed)
2. **no-playwright-wait-for-selector-fix.md** - Entire changeset was wrong (fixed)
3. **no-playwright-useless-await.md** - Wording issue, needed "Added the nursery rule..." format (fixed)
4. **no-playwright-skipped-test-fix.md** - Entire changeset was wrong (fixed)
5. **no-playwright-element-handle-fix.md** - Needs to be removed entirely (fixed)
6. **no-playwright-element-handle.md** - Minor wording tweak from "Prefers... over" to "Prefers... to" (fixed)

## CodeRabbit Suggestions

CodeRabbit identified 30 comments primarily focused on:
- Grammar/language improvements in documentation
- Wording consistency in changeset descriptions
- Code style suggestions
- Best practices for PR descriptions

## Action Items Addressed

All major feedback has been addressed:
- ✅ Rule naming conventions fixed
- ✅ Code reorganized for better structure
- ✅ Performance optimizations (string allocation reduction)
- ✅ Helper function signatures corrected
- ✅ Code placement standards followed
- ✅ Diagnostic message ordering fixed
- ✅ Changeset corrections applied
- ✅ Rule consolidation considerations noted for future work
- ⚠️ New options removal needed (will be separate PR on `next` branch)
- 📋 Cross-domain rule sources to be added in documentation

## Files Full Analysis

See `/Users/antonbossenbroek/Documents_local/projects/biome/PR_8960_REVIEW_COMMENTS.md` for the complete detailed transcript of all 94 comments, organized by file and in chronological order.

