---
"@biomejs/biome": patch
---

Fixed [#6656](https://github.com/biomejs/biome/issues/6656), where Biome incorrectly formatted HTML void elements such as `<meta>` when they contained the self-closing slash.

```diff
- <meta foo="bar" />
+ <meta foo="bar">
```
