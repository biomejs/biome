---
"@biomejs/biome": patch
---

Fixed [`noThenProperty`](https://biomejs.dev/linter/rules/no-then-property/) failing to detect `Object.fromEntries`, `Object.defineProperty`, and `Reflect.defineProperty` calls with comments between their tokens.
