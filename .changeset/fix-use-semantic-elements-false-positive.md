---
"@biomejs/biome": patch
---

Fixed [#5212](https://github.com/biomejs/biome/issues/5212): [`useSemanticElements`](https://biomejs.dev/linter/rules/use-semantic-elements/) no longer reports a diagnostic when a semantic element already has its corresponding role attribute (e.g. `<nav role="navigation">`, `<footer role="contentinfo">`). These cases are now correctly left to [`noRedundantRoles`](https://biomejs.dev/linter/rules/no-redundant-roles/).
