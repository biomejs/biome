---
"@biomejs/biome": patch
---

Fixed [#6229](https://github.com/biomejs/biome/issues/6229) where the fix of `noUnusedImports` emitted an invalid syntax. Now the following case emits a code fix that is syntactically correct:

```js
import Used, { NotUsed } from 'foo';

Used();
```
