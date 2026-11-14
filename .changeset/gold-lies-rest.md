---
"@biomejs/biome": patch
---

Fixed [#10077](https://github.com/biomejs/biome/issues/10077): `biome lint` and `biome check` no longer fail on clean input passed through `--stdin-file-path` without `--write`. Stdin checks now report diagnostics and respect `--error-on-warnings`. Read-only checks detect formatting differences when formatting is enabled. With `--write`, linting and checking return a failing exit code if errors remain after applying fixes.
