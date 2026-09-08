---
name: testing-codegen
description: Use this skill when selecting or running Biome test fixtures, quick tests, `insta` snapshot workflows, expectation comments, orphan checks, or required code generators. Do not use for subsystem implementation design.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Testing and Code Generation

Choose the narrowest test that exercises the changed behavior, then broaden only when shared infrastructure or integration risk justifies it.

## Test Selection

| Change | Start with |
| --- | --- |
| Lint rule | `just test-lintrule <ruleName>` |
| One crate | `cargo test -p <crate>` or the crate's focused test target |
| Parser or formatter investigation | `just qt <package>` |
| CLI migration | focused `biome_cli` migration tests |
| Documentation code | `just test-doc` |

Use `-- --show-output` or `--nocapture` only when the test's diagnostic output is needed.

Quick tests are scratch space for inspecting CST, formatter IR, or one analyzer query. Persistent behavior belongs in the subsystem's normal fixture directory before finishing.

## Snapshot Workflow

Run the focused test to produce snapshots, then review every changed section:

```shell
cargo insta review
```

Use `cargo insta accept` or `cargo insta reject` only after inspecting the pending changes. A passing snapshot test proves output matches the checked-in snapshot, not that the snapshot describes correct behavior.

### Orphaned Snapshots

Do not delete suspected orphan snapshots manually. Deletion is safe only after running the complete workspace snapshot suite without package, target, or test filters:

```shell
cargo insta test --workspace --unreferenced delete
```

For a scoped run, use `--unreferenced warn` or `--unreferenced reject`; incomplete test selection cannot prove that a snapshot is orphaned. Inspect every deletion from a complete run.

## Analyzer Fixtures

Place rule fixtures under the language analyzer's current `tests/specs/<group>/<rule>/` hierarchy. The directory group must match the rule declaration.

Use focused source files for parser-dependent cases. Use `.jsonc` arrays when multiple independent script snippets share the same configuration and module semantics are not required. Use `options.json` in a subdirectory when tests need different rule configuration.

### Diagnostic Expectation Comments

The test utilities recognize these marker texts in source comments:

```text
should generate diagnostics
should not generate diagnostics
```

Current enforcement:

- a filename containing `valid` but not `invalid` must contain one of the expectation markers;
- when either marker is present, actual diagnostics must match it;
- an `invalid` or neutral filename without a marker is accepted, but adding the correct marker makes intent explicit;
- `.snap`, `.json`, `.jsonc`, and `.md` are exempt from the mandatory valid-file marker check;
- HTML-family workspace fixtures are checked from raw file content and should use a top-level HTML comment.

Put the marker at the top of a primary fixture. Sidecar files with neutral names do not need a marker unless they are independently analyzed as cases.

## Parser Fixtures

Use the parser crate's established `ok/` and `error/` directories. A recovery regression should include valid syntax after the malformed construct to prove the parser resumes at the intended boundary.

Use `just qt <parser-package>` to inspect a CST during development; do not leave the quick test as the only regression coverage.

## Formatter Fixtures

Use internal specs for behavior introduced or fixed by the change. External Prettier snapshots record comparison results but do not replace focused internal coverage.

The formatter harness performs its idempotency reformat during one test invocation for eligible files. Inspect both formatted output and any IR shown for a mismatch.

## Required Code Generation

| Changed source | Command |
| --- | --- |
| `.ungram` grammar | `just gen-grammar <lang>` |
| Formatter source | `just gen-formatter <lang>` |
| Lint rule or assist | `just gen-rules` and `just gen-configuration` |
| Bindings needed locally | `just gen-bindings` |

Root `AGENTS.md` is canonical for which artifacts must be committed and which full outputs CI Autofix may provide.

Do not run `just ready` in a dirty working tree: the recipe checks for a clean diff before and after its full verification sequence. Use the focused commands required by the current task, then `just f` and `just l`.

## Completion Checklist

- A code change has focused persistent coverage.
- A bug fixture fails without the fix.
- Snapshot contents were inspected.
- Expectation comments match fixture intent.
- Orphan snapshots were pruned through `insta`.
- Required generated artifacts are present.
- Narrow tests pass before broader checks run.

## References

- Main test guide: `CONTRIBUTING.md#testing`
- Analyzer guide: `crates/biome_analyze/CONTRIBUTING.md`
- Expectation enforcement: `crates/biome_test_utils/src/lib.rs`
- Formatter harness: `crates/biome_formatter_test/src/spec.rs`
- Generator recipes: `justfile`
