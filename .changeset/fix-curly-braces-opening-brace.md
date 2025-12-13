---
"@biomejs/biome": patch
---

Fixed [#8011](https://github.com/biomejs/biome/issues/8011): [`useConsistentCurlyBraces`](https://biomejs.dev/linter/rules/use-consistent-curly-braces/) no longer suggests removing curly braces from JSX expression children containing characters that would cause parsing issues or semantic changes when converted to plain JSX text (`{`, `}`, `<`, `>`, `&`).
