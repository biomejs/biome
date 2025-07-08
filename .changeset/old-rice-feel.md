---
"@biomejs/biome": patch
---

Fixed [#6656](https://github.com/biomejs/biome/issues/6656): Biome now correctly formats HTML void elements such as `<meta>` when they contain a self-closing slash.

```diff
- <meta foo="bar" />
+ <meta foo="bar">
```
