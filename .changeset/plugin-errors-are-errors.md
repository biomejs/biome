---
"@biomejs/biome": patch
---

Fixed [#12851](https://github.com/biomejs/biome/issues/12851): a plugin that throws or fails to run now reports an error diagnostic instead of an info diagnostic, so `biome lint` and `biome check` exit with a non-zero code.
