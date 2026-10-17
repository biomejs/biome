---
"@biomejs/biome": patch
---

Fixed type inference for class methods with their own type parameters. [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now detects unhandled promises returned through generic class methods, such as `factory.create<Promise<void>>()`.
