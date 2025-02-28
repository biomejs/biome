---
"@biomejs/biome": patch
---

Fix CSS parser case error, `@-moz-document url-prefix(https://example.com)` and `@-moz-document domain(example.com)` are now valid.
