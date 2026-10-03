---
"@biomejs/biome": patch
---

Added the new nursery rule [`noAstroDeprecatedGetEntryBySlug`](https://biomejs.dev/linter/rules/no-astro-deprecated-get-entry-by-slug/), which reports imports of `getEntryBySlug()` from `astro:content`. Astro 6 removed this function in favor of `getEntry()`.

```js
import { getEntryBySlug } from "astro:content";
```
