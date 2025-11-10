# Ember Linting Rules Implementation Progress

## Project Overview

Porting Ember linting rules from eslint-plugin-ember and ember-template-lint to Biome's analyzer framework. Goal is to provide comprehensive Ember.js support in Biome.

**Total Scope**: ~218 rules (97 from eslint-plugin-ember + 130 from ember-template-lint, minus 9 formatting rules)

## Current Status

### Completed: 13 Rules (6% complete)

All rules are in the `nursery` lint group and use the `noEmber*` naming convention.

#### Batch 1 (Commit: d054f04188)
1. **noEmberPauseTest** - Disallow `pauseTest()` in tests
   - Type: Semantic analysis (checks imports from `@ember/test-helpers`)
   - Query: `Semantic<JsCallExpression>`
   - Tests: 5 test files

2. **noEmberGlobalJquery** - Disallow global `$` and `jQuery`
   - Type: Semantic analysis (checks for global references)
   - Query: `Semantic<AnyJsExpression>`
   - Tests: 6 test files

3. **noEmberAccesskeyAttribute** - Disallow `accesskey` attribute in templates
   - Type: Template parsing (HTML parser with Glimmer mode)
   - Query: `Ast<JsModule>`
   - Tests: 4 .gjs test files

#### Batch 2 (Commit: 295f92fddc)
4. **noEmberOldShims** - Disallow deprecated shim imports
   - Type: Import path checking
   - Query: `Ast<JsImport>`
   - Tests: 8 test files

5. **noEmberGetWithDefault** - Disallow `getWithDefault()` method
   - Type: Semantic analysis (checks imports from `@ember/object`)
   - Query: `Semantic<JsCallExpression>`
   - Tests: 5 test files

6. **noEmberAutofocus** - Disallow `autofocus` attribute in templates
   - Type: Template parsing (HTML parser with Glimmer mode)
   - Query: `Ast<JsModule>`
   - Tests: 4 .gjs test files

#### Batch 3 (Commit: a4f157a6ac)
7. **noEmberClassicClasses** - Disallow `.extend()` pattern
   - Type: Semantic analysis (checks for classic class patterns)
   - Query: `Semantic<JsCallExpression>`
   - Tests: 8 test files

8. **noEmberRequireComputedPropertyDependencies** - Require computed deps
   - Type: Semantic analysis (checks `computed()` calls)
   - Query: `Semantic<JsCallExpression>`
   - Tests: 7 test files

9. **noEmberInlineStyles** - Disallow inline `style` attributes
   - Type: Template parsing (HTML parser with Glimmer mode)
   - Query: `Ast<JsModule>`
   - Tests: 4 .gjs test files

#### Batch 4 (Commit: 6f4b067c41) ✨ LATEST
10. **noEmberActionsHash** - Disallow `actions: {}` hash pattern
    - Type: AST analysis (checks object properties)
    - Query: `Ast<JsPropertyObjectMember>`
    - Tests: 5 test files
    - Known limitation: Only detects object properties, not class field initializers

11. **noEmberGet** - Disallow `get()` helper function
    - Type: Semantic analysis (checks imports from `@ember/object`)
    - Query: `Semantic<JsCallExpression>`
    - Tests: 5 test files
    - Conservative: Only flags imported `get()`, not method calls

12. **noEmberPositiveTabindex** - Disallow positive tabindex values
    - Type: Template parsing with value checking
    - Query: `Ast<JsModule>`
    - Tests: 4 .gjs test files
    - Parses attribute values numerically

13. **noEmberMixins** - Disallow imports from `/mixins/` directories
    - Type: Import path checking
    - Query: `Ast<JsImport>`
    - Foundation rule (implemented first)

## Established Workflow

### TDD Process (RED → GREEN → REFACTOR)

1. **Plan** (if starting new batch):
   - Use TodoWrite to track 3 rules in parallel
   - Launch 3 agents using Task tool with sonnet model

2. **Implement** (each rule):
   - Create rule file: `crates/biome_js_analyze/src/lint/nursery/no_ember_*.rs`
   - Write comprehensive tests with multiple cases
   - Use snapshot testing with `cargo insta`
   - Add diagnostic messages with helpful notes

3. **Verify**:
   - Run code generator: `cargo run -p xtask_codegen -- analyzer`
   - Verify compilation: `cargo check -p biome_js_analyze`
   - Accept snapshots: `cargo insta review`

4. **Commit**:
   - Add all files (rules, tests, generated files)
   - Write detailed commit message documenting each rule
   - Include AI disclosure: "This PR was written primarily by Claude Code"

### Key Commands

```bash
# Run code generator (updates registry and generates options)
cargo run -p xtask_codegen -- analyzer

# Verify compilation
cargo check -p biome_js_analyze

# Run specific rule tests
cargo test -p biome_js_analyze --test spec_tests nursery/noEmberRuleName

# Review/accept snapshots
cargo insta review
cargo insta accept

# Full test suite
cargo test -p biome_js_analyze
```

## Technical Patterns

### Pattern 1: Semantic Analysis (Import Checking)

Used for rules that need to verify imports from specific Ember packages.

```rust
impl Rule for NoEmberExample {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?;

        // Check function name
        let function_name = get_callee_name(&callee)?;
        if function_name != "targetFunction" {
            return None;
        }

        // Use semantic model to verify import
        if let Some(ident) = callee.as_js_identifier_expression() {
            let model = ctx.model();
            let binding = model.binding(&ident)?;

            // Check if imported from correct package
            let import_source = binding.import_source()?;
            if import_source != "@ember/package" {
                return None;
            }
        }

        Some(call.range())
    }
}
```

**Examples**: noEmberPauseTest, noEmberGetWithDefault, noEmberGet, noEmberClassicClasses

### Pattern 2: Simple Import Path Checking

Used for rules that just need to check import paths.

```rust
impl Rule for NoEmberExample {
    type Query = Ast<JsImport>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

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

**Examples**: noEmberMixins, noEmberOldShims

### Pattern 3: Template Parsing (Glimmer)

Used for rules that need to analyze HTML in `<template>` blocks.

```rust
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use regex::Regex;
use std::sync::LazyLock;

static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid regex")
});

impl Rule for NoEmberExample {
    type Query = Ast<JsModule>;
    type State = Vec<Violation>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query();

        // Check if this is a Glimmer file
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.as_embedding_kind().is_glimmer() {
            return None;
        }

        let source = module.syntax().text_with_trivia().to_string();
        let violations = find_violations(&source);

        if violations.is_empty() { None } else { Some(violations) }
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

        let root = parse.tree();
        let root_node = root.syntax();

        // Traverse elements
        for node in root_node.descendants() {
            if let Some(element) = AnyHtmlElement::cast(node) {
                if let Some(attr) = element.find_attribute_by_name("target-attr") {
                    // Calculate absolute position
                    let attr_range = attr.range();
                    let absolute_start: u32 =
                        (template_start + usize::from(attr_range.start()))
                        .try_into().unwrap();
                    let absolute_end: u32 =
                        (template_start + usize::from(attr_range.end()))
                        .try_into().unwrap();

                    violations.push(Violation {
                        range: TextRange::new(absolute_start.into(), absolute_end.into()),
                    });
                }
            }
        }
    }

    violations
}
```

**Examples**: noEmberAccesskeyAttribute, noEmberAutofocus, noEmberInlineStyles, noEmberPositiveTabindex

### Pattern 4: Global Reference Checking

Used to detect usage of global variables.

```rust
impl Rule for NoEmberExample {
    type Query = Semantic<AnyJsExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expr = ctx.query();
        let ident = expr.as_js_identifier_expression()?;
        let name = ident.name().ok()?;
        let name_text = name.text();

        if name_text != "GlobalName" {
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

**Examples**: noEmberGlobalJquery

## Important Notes

### Code Documentation

- Use `js,ignore` for code blocks in docs (not `gjs`) to avoid parser errors
- Include both invalid and valid examples
- Add helpful notes explaining why the pattern is problematic

### Testing

- .gjs files need full component structure, not just templates
- Tests should cover edge cases and nested structures
- Use snapshot testing for consistent output
- Test file structure: `crates/biome_js_analyze/tests/specs/nursery/RuleName/testCase.js`

### Parallel Agent Workflow

- Launch 3 agents in parallel using Task tool with sonnet model
- Agents may temporarily see incomplete files from other agents (acceptable)
- All agents typically complete successfully despite interference
- Verify compilation before committing to catch any issues

### Code Generation

- Always run after adding new rules: `cargo run -p xtask_codegen -- analyzer`
- Updates multiple files:
  - `crates/biome_js_analyze/src/lint/nursery.rs` (module declarations)
  - `crates/biome_configuration/src/analyzer/linter/rules.rs` (rule registry)
  - `crates/biome_diagnostics_categories/src/categories.rs` (diagnostic categories)
  - `crates/biome_rule_options/src/*.rs` (options files)
  - JSON schemas and TypeScript definitions

### Known Warnings

- `element_name` field unused warnings in template rules (intentional - kept for future use)
- These warnings are safe to ignore

## Next Steps

### Remaining Ember Rules to Port

From eslint-plugin-ember (~84 remaining):
- no-attrs-in-components
- no-attrs-snapshot
- no-capital-letters-in-routes
- no-component-lifecycle-hooks
- no-controllers
- no-current-route-name
- no-deeply-nested-dependent-keys-with-each
- no-duplicate-dependent-keys
- no-ember-super-in-es-classes
- no-empty-glimmer-component-classes
- no-function-prototype-extensions
- no-get
- no-implicit-injections
- no-incorrect-calls-with-inline-anonymous-functions
- no-invalid-debug-function-arguments
- no-observers
- no-on-calls-in-components
- no-private-routing-service
- no-unnecessary-index-route
- no-unnecessary-route-path-option
- no-unnecessary-service-injection-argument
- require-computed-macros
- require-computed-property-dependencies
- require-return-from-computed
- require-super-in-lifecycle-hooks
- route-path-style
- use-ember-data-rfc-395-imports
- And many more...

From ember-template-lint (~130 template rules):
- no-action
- no-args-paths
- no-at-ember-render-modifiers
- no-capital-letters
- no-class-bindings
- no-curly-component-invocation
- no-element-event-actions
- no-implicit-this
- no-invalid-interactive
- no-nested-interactive
- no-passed-in-event-handlers
- no-positional-data-test-selectors
- no-quoteless-attributes
- no-triple-curlies
- no-unbound
- require-button-type
- require-context-role
- require-input-label
- require-valid-alt-text
- And many more...

### Recommended Implementation Order

1. **High-priority JS rules** (commonly used, easy to implement):
   - no-observers
   - no-get (duplicate of noEmberGet, may need adjustment)
   - no-function-prototype-extensions
   - require-return-from-computed
   - require-super-in-lifecycle-hooks

2. **High-priority template rules** (accessibility and modern patterns):
   - no-action
   - no-implicit-this
   - no-curly-component-invocation
   - require-button-type
   - require-input-label

3. **Medium-priority rules** (deprecations and best practices):
   - no-controllers
   - no-observers
   - no-component-lifecycle-hooks
   - use-ember-data-rfc-395-imports

4. **Complex rules** (require more sophisticated analysis):
   - no-capital-letters-in-routes
   - route-path-style
   - no-deeply-nested-dependent-keys-with-each

### Batch Planning

Continue with batches of 3 rules each:
- **Batch 5**: Pick 3 high-priority JS rules
- **Batch 6**: Pick 3 high-priority template rules
- **Batch 7**: Mix of JS and template rules
- Continue pattern...

## Resources

- eslint-plugin-ember: https://github.com/ember-cli/eslint-plugin-ember
- ember-template-lint: https://github.com/ember-template-lint/ember-template-lint
- Ember Guides: https://guides.emberjs.com/
- Biome Analyzer Contribution Guide: `crates/biome_analyze/CONTRIBUTING.md`

## Git History

```bash
# View all Ember rule commits
git log --oneline --grep="Ember\|ember"

# View files changed in a batch
git show --stat <commit-hash>

# View detailed commit
git show <commit-hash>
```

## AI Disclosure

This work has been implemented primarily by Claude Code using:
- Parallel sub-agent workflow for batch implementation
- Task tool with sonnet model for specialized implementations
- TDD methodology throughout

All implementations should include this disclosure in commit messages and PR descriptions.
