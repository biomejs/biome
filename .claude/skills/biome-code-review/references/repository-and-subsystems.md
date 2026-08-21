# Repository and Subsystem Review

Read only the sections matching the diff. Load the corresponding implementation skill for implementation contracts; this reference adds reviewer-specific checks.

## Repository Map

| Area | Source of truth | Common counterparts |
| --- | --- | --- |
| Grammar | `xtask/codegen/<lang>.ungram` | `biome_<lang>_syntax/src/generated/`, `biome_<lang>_factory/src/generated/` |
| Lint rule | `biome_<lang>_analyze/src/lint/<group>/<rule>.rs` | group module, registry, configuration rules, diagnostic categories |
| Assist | `biome_<lang>_analyze/src/assist/source/<action>.rs` | source module, registry, assist configuration, categories |
| Rule options | `biome_rule_options/src/<rule>.rs` | rule `type Options`, rustdoc, option fixtures |
| Formatter | `biome_<lang>_formatter/src/**` | internal specs and Prettier snapshots |
| Parser | `biome_<lang>_parser/src/**` | `ok/` and `error/` fixtures |
| Types/module graph | `biome_js_type_info/`, `biome_module_graph/` | database and service consumers |
| Changeset | `.changeset/*.md` | user-visible behavior in the diff |

Generated files identify themselves. Review the human-authored source and confirm required checked-in counterparts are present without running generators.

## Lint Rules and Assists

Load `lint-rule-development`. Prioritize these reviewer checks:

- A rule matching a known global must reject locally shadowed bindings.
- Cover every relevant AST union member and framework-specific value shape.
- Verify suppression and option defaults preserve existing behavior.
- `FixKind::Safe` must not change semantics in any reachable case.
- Applying an action must produce code the rule no longer reports.
- New rules belong in `nursery`; established rules remain in their declared group.
- An option needs an actual conflicting preference or semantic mode, not speculative flexibility.
- A group/path mismatch can make a test silently exercise no diagnostics.

## Parser

Load `parser-development`.

- A presence test must not consume input before returning `Absent`.
- Recovery must produce a `BOGUS_*` node allowed by the grammar at that position.
- Recovery sets should stop at the relevant separator or enclosing boundary without swallowing following valid syntax.
- Parser behavior changes need both successful and malformed fixtures when recovery is involved.

## Formatter

Load `formatter-development`.

- Dropped comments or tokens and idempotency failures are high severity.
- Formatting composition belongs in a type implementing `Format`; a cluster of free functions passing `&mut Formatter` has no clear owner for layout invariants.
- A multi-way layout should be selected once and represented explicitly rather than recomputed at write sites.
- `fmt_fields` should destructure every generated field explicitly. `..` can hide a newly added token, and a bound field that is never written drops source content.
- A Prettier snapshot disappearing proves agreement only for that input. Behavior changes still need a focused internal spec.

## Diagnostics

Load `diagnostics-development`.

- The message identifies the problem rather than restating a construct or rule name.
- Advice explains why the condition matters and gives a concrete next action when no code action exists.
- The highlighted range points to the smallest useful source span.
- Category and link registration should come from the supported generator or scaffolding workflow, not manual edits to generated registries.

## Types and Module Graph

Load `type-inference`.

- `Unknown` means the system cannot prove the type; treating it as a negative answer creates false positives.
- Start from the narrowest query that can answer the caller instead of inferring an entire module by default.
- A Salsa query must track every input that can change its result.
- Cross-module data must remain reference-based so edits cannot leave copied data stale.
- Trace changed resolution paths from caller result through each query, reference, import, fallback, and owning module.
- Require semantic tests for the type shapes actually traversed. Require query-event tests only for claims about dependency or inference scope.

## Code Generation and Registration

- Verify artifacts, not claims that a command was run.
- A grammar change requires matching generated syntax and factory changes.
- A formatter source change requires formatter generation for that language.
- A lint-rule change requires generated rule registration and configuration.
- Do not report bindings or other full analyzer outputs that `AGENTS.md` explicitly leaves to CI Autofix.
- Search parallel registration, serialization, migration, preset, and documentation sites for new enum variants, options, and manifest fields.

## Tests and Snapshots

Load `testing-codegen`.

- Read snapshot diffs as behavior, not generated noise.
- Verify a regression fixture reaches the changed branch and fails without the fix.
- Confirm rule fixture paths match the declared group.
- A manually deleted snapshot is suspicious; the supported pruning workflow determines whether it is orphaned.
- Name the exact untested scenario and defect it would catch instead of requesting generic additional coverage.
