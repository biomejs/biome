---
"@biomejs/biome": patch
---

Fixed [#10113](https://github.com/biomejs/biome/issues/10113): `--suppress` with `--only` no longer ignores overrides that disable a rule.

Previously, running `biome lint --suppress --only=lint/suspicious/noDebugger` would add suppression comments to files where the rule was disabled by an override. Now, overrides that set a rule to `"off"` are always respected, regardless of whether `--only` is active.
