---
"@biomejs/biome": patch
---

Fixed [#9506](https://github.com/biomejs/biome/issues/9506) and [#9479](https://github.com/biomejs/biome/issues/9479): Biome no longer reports false parse errors on `<script type="speculationrules">` and `<script type="application/ld+json">` tags. These script types contain non-JavaScript content and are now correctly skipped by the embedded language detector.
