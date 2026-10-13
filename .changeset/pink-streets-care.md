---
"@biomejs/biome": patch
---

Fixed false positives in [`noProto`](https://biomejs.dev/linter/rules/no-proto/) for computed identifiers, property names containing quotes, and tagged templates. Expressions such as `obj[__proto__]` and `` obj[`"__proto__"`] `` no longer trigger the rule.

Fixed missed diagnostics for `__proto__` accesses through parentheses, TypeScript wrappers, and comma expressions, such as `obj[('__proto__')]`.
