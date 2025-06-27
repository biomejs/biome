---
"@biomejs/biome": patch
---

Added a new nursery rule `noAlert` that disallows the use of `alert`, `confirm` and `prompt`.

The following code is deemed incorrect:

```js
alert("here!");
```
