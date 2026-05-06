---
"@biomejs/biome": patch
---

Fixed [#10271](https://github.com/biomejs/biome/issues/10271). The HTML parser no longer reports `Unexpected value or character` for text nodes that start with `of`. Other text-positioned keywords (`as`, `in`, `out`, etc.) were already handled by the same code path; the regression added in 2.4.14 left `of` (used by Vue's `v-for X of Y`) outside of it.
