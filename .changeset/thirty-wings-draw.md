---
"@biomejs/biome": patch
---

Support parse `import defer`(which is a [stage3 proposal](https://github.com/tc39/proposal-defer-import-eval)). The syntax look like this:

```ts
import defer * as foo from "<specifier>";
```
