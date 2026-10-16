---
"@biomejs/biome": patch
---

Fixed [#12373](https://github.com/biomejs/biome/issues/12373): `biome migrate eslint` no longer migrates `one-var` to [`useSingleVarDeclarator`](https://biomejs.dev/linter/rules/use-single-var-declarator/) unless `--include-inspired` is passed.
