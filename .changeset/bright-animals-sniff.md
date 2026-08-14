---
"@biomejs/biome": patch
---

Fixed [#11315](https://github.com/biomejs/biome/issues/11315): The CSS parser now recovers at declaration boundaries after bogus declarations, allowing subsequent valid declarations to be parsed.
