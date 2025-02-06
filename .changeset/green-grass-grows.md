---
"@biomejs/biome": patch
---

Fix [#342](https://github.com/biomejs/biome/issues/342), js parser is no longer progressing for an invalid object member name:

```js
({
  params: { [paramName: string]: number } = {}
})
```
