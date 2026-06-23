---
"@biomejs/biome": patch
---

Fixed [#10716](https://github.com/biomejs/biome/issues/10716): `biome migrate --write` now correctly preserves the `recommended` preset when migrating the deprecated `linter.rules.recommended: true` boolean to `linter.rules.preset`. The mapping was previously broken for inputs where whitespace was attached as token trivia, causing `true` to be rewritten to `preset: "none"` and silently disabling all rules.
