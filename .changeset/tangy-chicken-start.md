---
"@biomejs/biome": patch
---

Added support in the JS parser for `import source`(a [stage3 proposal](https://github.com/tc39/proposal-source-phase-imports)). The syntax looks like:

```ts
import source foo from "<specifier>";
```
