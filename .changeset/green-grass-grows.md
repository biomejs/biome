---
"@biomejs/biome": patch
---

Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now correctly handles invalid object member names, such as:

```js
({
  params: { [paramName: string]: number } = {}
})
```
