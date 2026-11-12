---
"@biomejs/biome": patch
---

Fixed [#13102](https://github.com/biomejs/biome/issues/13102): `biome ci` no longer forces ANSI colors when running outside a CI environment, so output is no longer colored when piped, redirected, or when `NO_COLOR` is set. Colors are still forced in non-GitHub CI environments when no `--colors` option is provided.
