---
"@biomejs/biome": patch
---

Fixed [`noUselessReturn`](https://biomejs.dev/linter/rules/no-useless-return) in three cases. The rule no longer reports a `return;` that is the entire body of an unbraced `if` or `else`, since removing it alone would leave the `if` without a consequent and break parsing. The safe fix now preserves leading comments of the removed statement instead of deleting them with it. In TypeScript files, the rule no longer reports a trailing `return;` when another code path of the same function returns a value, because removing it breaks `tsc --noImplicitReturns` (TS7030).
