---
"@biomejs/biome": patch
---

Fixed [#11949](https://github.com/biomejs/biome/issues/11949): [`noSelfAssign`](https://biomejs.dev/linter/rules/no-self-assign/) now reports assignments whose sides name the same property through dot, string, or numeric syntax, such as `a.b = a["b"]` and `a[0] = a["0"]`.
