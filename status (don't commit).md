## Findings

- `low/tests` `crates/biome_service/src/file_handlers/mod.rs:1324` - The `is_diagnostic_error` change is untested. This path drives the error count (and exit code) of `lint --write` / `check --write` through `ProcessFixAll`, but every new CLI test runs without `--write`. Add a fixture with `minimumSeverity: "error"`, an info-level rule whose fix is unsafe (so it is not applied), and `--write`, asserting the command fails.

- `low/documentation` `.changeset/linter-minimum-severity.md:5` - New user-facing configuration option without a linked website docs PR. CONTRIBUTING requires a docs PR against the website `next` branch for new configuration options; link it from this PR.

- `optional/maintainability` `crates/biome_service/src/settings.rs:2387` - Redundant inheritance in `to_override_settings`. `as_linter_minimum_severity_by_indices` already starts from the base value and only replaces it when an override sets one, so falling back to `current_settings.linter.minimum_severity` here is dead logic and diverges from how `rules` and `domains` are stored. Store `linter.minimum_severity` directly.

- `optional/correctness` `crates/biome_configuration/src/analyzer/mod.rs:648` - `MinimumSeverity::Info.raise(Severity::Hint)` yields `Information`, so the default is not literally a no-op as the docs and changeset claim. No built-in rule emits `Hint` and plugin diagnostics are excluded, so it is unreachable today; consider short-circuiting `Info` to return the input unchanged so the invariant does not depend on that.

## Questions

- Plugin diagnostics use the `plugin` category, not `lint/`, so `minimumSeverity` never raises them, while the CLI summary reporter groups them with lint diagnostics. The changeset says "every lint rule". Is excluding Grit/JS plugin rules intended? If so, say so in the changeset and rustdoc; if not, extend `is_lint_rule` to `plugin`.

## Review Status

Scope: `de1ad4c4c5` (origin/next) through `0779d48a7a` / clean working tree, 14 files, no untracked files.
Branch target: `next` is correct (minor feature).
Changeset: present and correct (`@biomejs/biome: minor`).
Brief: independent.
Validation: Static review only; no project code was run.
Fetch: updated `origin/next`.
