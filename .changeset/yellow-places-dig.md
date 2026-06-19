---
"@biomejs/biome": patch
---

Fixed [#10612](https://github.com/biomejs/biome/issues/10612). The Biome parser now correctly parses processing instructions. The following SVG doesn't throw errors anymore:

```svg
<?xml version="1.0" encoding="UTF-8" ?>

<svg></svg>
```
