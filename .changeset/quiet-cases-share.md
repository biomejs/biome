---
"@biomejs/biome": patch
---

Fixed [#12475](https://github.com/biomejs/biome/issues/12475): [`noSwitchDeclarations`](https://biomejs.dev/linter/rules/no-switch-declarations/) no longer reports `var` declarations inside `switch` clauses, matching ESLint's `no-case-declarations`.
