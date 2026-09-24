---
"@biomejs/biome": patch
---

Fixed [#11893](https://github.com/biomejs/biome/issues/11893), where [`noDeprecatedImports`](https://biomejs.dev/linter/rules/no-deprecated-imports/) reported an import when only some function overloads were marked deprecated. The rule now reports overloaded imports only when every overload is deprecated.
