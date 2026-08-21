---
name: lint-rule-development
description: Create or modify Biome lint rules and assists, including analyzer queries, semantic bindings, rule state, code actions, fix safety, options, registration, and end-user rule rustdoc. Use for rule behavior and implementation; use diagnostics-development for substantial message/advice design and testing-codegen for fixture or snapshot mechanics.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Lint Rule Development

Follow the current analyzer architecture in `crates/biome_analyze/CONTRIBUTING.md`. Read only the sections relevant to the rule rather than loading the entire guide by default.

## Workflow

1. Find two or three current rules in the same language and group with a similar query or action.
2. Generate the matching lint-rule or assist scaffolding when adding analyzer behavior.
3. Implement the narrowest query and state needed to decide whether to signal.
4. Add a diagnostic and, when safe, an action.
5. Add focused valid and invalid fixtures, then inspect snapshots.
6. Run required analyzer codegen and the narrowest tests.

Scaffolding commands:

```shell
just new-js-lintrule useMyRule
just new-css-lintrule useMyRule
just new-json-lintrule useMyRule
just new-graphql-lintrule useMyRule
```

New lint rules start in `nursery`. They are patch changes targeting `main`, because nursery rules do not follow normal feature versioning. Load `changeset` for the release entry.

For a new assist, use the language's `new-*-assistrule` recipe, for example:

```shell
just new-js-assistrule useMyAction
just new-json-assistrule useMyAction
```

The generator places assists under `src/assist/source/`; they do not use lint groups or the nursery policy. A new assist is a user-facing feature and normally requires a minor changeset targeting `next`. A bug fix to an existing assist follows normal bug-fix policy. Check `justfile` for the languages with assist scaffolding.

## Query and State

Choose the least expensive query that answers the rule:

- `Ast<Node>` for syntax-local checks;
- `Semantic<Node>` when bindings, references, scopes, or globals are required;
- a service query only when the fact is owned by that service;
- type inference only when syntax and the semantic model cannot answer the question.

The analyzer guide's query and service sections are canonical for available APIs.

`run()` should decide whether to emit a signal. Keep action-only work in `action()` so it is not performed for every candidate node.

Prefer ranges, syntax nodes, tokens, and compact enums in `State`. A `String`, `Box<str>`, or collection built from syntax text often indicates avoidable allocation; load `syntax-text-handling` before owning source text.

For rules matching a global identifier, prove the reference resolves to the global rather than a local shadow. Cover every relevant member of node unions and framework-specific syntax families.

## Diagnostics

Every diagnostic answers three separate questions:

1. What condition was found?
2. Why is it a problem?
3. What should the user do?

The message answers the first question. Advice answers the second and, when no action exists, the third. A code action and its label normally answer the third when an automated fix exists.

Load `diagnostics-development` for message structure, markup, details, advice, categories, and standalone `Diagnostic` types. Do not duplicate its guidance in the rule implementation.

## Actions

Build mutations in `action()`. Set `FixKind::Safe` only when no reachable input changes behavior. If safety depends on assumptions the rule cannot prove, use `Unsafe`.

Test that applying the action:

- produces valid syntax;
- preserves comments and trivia;
- does not trigger the same rule again;
- preserves semantics for a safe fix;
- handles every syntax variant accepted by `run()`.

Use existing rules with the same mutation shape as API examples.

## Options

Add options only for a real semantic mode or established conflicting preference. Do not add speculative flexibility.

Use the rule's generated or existing file under `crates/biome_rule_options/src/` as the source of truth. Follow the analyzer guide sections **Rule Options**, **Merge**, and **Documenting Options** for current derives, merge behavior, configuration examples, and rustdoc requirements.

Check these integration points:

- the rule's `type Options`;
- defaults preserve behavior when the option is absent;
- extended configurations merge according to the option type's contract;
- fixture directories with `options.json` cover each behavior;
- rule rustdoc states every option's default and shows it in use;
- `just gen-rules` and `just gen-configuration` update registrations and configuration.

If the source ESLint rule has options that `biome migrate eslint` should preserve, load `eslint-migrate-options`.

## Rule Documentation

Rustdoc inside `declare_lint_rule!` and `declare_assist_rule!` is end-user website content, not internal API documentation.

- Start with a single-line behavior summary.
- Explain why the reported pattern is problematic.
- Put `### Invalid` examples before `### Valid` examples.
- Mark examples according to the current rules-check syntax.
- For every option, state the default, show configuration, and show an applied example.
- Do not use `ignore` merely to avoid validation.

Inspect neighboring rule documentation and the analyzer guide before writing examples.

## Testing and Generation

Load `testing-codegen` for fixture naming, expectation comments, `.jsonc` cases, snapshot review, and pruning.

Typical focused commands:

```shell
just test-lintrule useMyRule
just gen-rules
just gen-configuration
```

Run `just f` and `just l` as required by `AGENTS.md`.

## Review Checklist

- The query is no broader than needed.
- Globals and shadowing are handled through the semantic model.
- `State` avoids unnecessary owned source text.
- Action-only work is deferred to `action()`.
- Safe fixes preserve semantics.
- Defaults preserve existing behavior.
- Valid, invalid, option, suppression, and fix cases cover the changed branches.
- Required generated artifacts are present.
- User-facing diagnostics and rustdoc explain behavior rather than implementation.

## References

- Analyzer guide: `crates/biome_analyze/CONTRIBUTING.md`
- Rule implementations: `crates/biome_*_analyze/src/lint/`
- Rule options: `crates/biome_rule_options/src/`
- Test fixtures: `crates/biome_*_analyze/tests/specs/`
