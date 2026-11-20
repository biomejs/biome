---
"@biomejs/biome": patch
---

Fixed [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), which now detects unhandled promises returned through generic class methods, such as `factory.create<Promise<void>>()`.
