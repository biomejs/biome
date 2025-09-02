---
"@biomejs/biome": patch
---

Support dynamic `import defer` and `import source`. The syntax looks like:

```ts
import.source("foo");
import.source("x", { with: { attr: "val" } });
import.defer("foo");
import.defer("x", { with: { attr: "val" } });
```
