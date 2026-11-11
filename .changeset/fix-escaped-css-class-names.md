---
"@biomejs/biome": patch
---

Fixed [#13078](https://github.com/biomejs/biome/issues/13078): [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/) and [`noUndeclaredClasses`](https://biomejs.dev/linter/rules/no-undeclared-classes/) now decode CSS escapes in class selectors. A selector such as `.foo\+bar` now matches `class="foo+bar"`, so neither rule reports it anymore. `noUndeclaredClasses` also no longer misses classes defined in separate `<style>` blocks at the same position within each block.
