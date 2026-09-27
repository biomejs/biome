---
"@biomejs/biome": patch
---

Fixed [#11903](https://github.com/biomejs/biome/issues/11903): [`noUselessReturn`](https://biomejs.dev/linter/rules/no-useless-return/) no longer reports the trailing `return;` of a TypeScript function when TypeScript's type checker requires it. The rule now keeps the statement when the enclosing function declares a return type other than `void`, `undefined` or `any`, or when another `return` statement of the same function returns a value, so that enabling `noImplicitReturns` no longer turns a compiling file into a `TS7030` error after applying the safe fix.


