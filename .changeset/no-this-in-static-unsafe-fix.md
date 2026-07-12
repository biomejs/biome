---
"@biomejs/biome": patch
---

Fixed [#10278](https://github.com/biomejs/biome/issues/10278): The fix of the [`noThisInStatic`](https://biomejs.dev/linter/rules/no-this-in-static/) rule is now unsafe by default. Replacing `this` with the class name in a static context isn't always safe, for example when a static `[Symbol.hasInstance]` method relies on `this` to resolve the current class through the prototype chain.
