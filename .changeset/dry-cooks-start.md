---
"@biomejs/biome": patch
---

Fixed [#8174](https://github.com/biomejs/biome/issues/8174), where the HTML parser would parse 2 directives as a single directive because it would not reject whitespace in Vue directives. This would cause the formatter to erroneously merge the 2 directives into one, resulting in broken code.

```diff
- <Component v-else:property="123" />
+ <Component v-else :property="123" />
```
