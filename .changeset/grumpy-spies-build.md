---
"@biomejs/biome": patch
---

Added the nursery rule [`noAstroDeprecatedResolve`](https://biomejs.dev/linter/rules/no-astro-deprecated-resolve/), which reports uses of the deprecated `Astro.resolve()` API in Astro files.

```astro
<img src={Astro.resolve("../images/penguin.png")} />
```
