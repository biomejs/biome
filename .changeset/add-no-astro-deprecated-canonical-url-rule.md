---
"@biomejs/biome": patch
---

Added the nursery rule [`noAstroDeprecatedCanonicalUrl`](https://biomejs.dev/linter/rules/no-astro-deprecated-canonical-url/), which reports reads of `Astro.canonicalURL` in Astro files. Astro 2.0 removed this property, so use `Astro.url` to build the canonical URL instead.

```astro
---
const canonicalURL = Astro.canonicalURL;
---
```
