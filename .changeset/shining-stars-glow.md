---
"@biomejs/biome": patch
---

Fix [#4533](https://github.com/biomejs/biome/issues/4533), don't throw error when pseudo class after a webkit scrollbar pseudo element.

The following code will not report:

```css
::-webkit-scrollbar-thumb:hover {}
```
