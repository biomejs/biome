---
"@biomejs/biome": patch
---

Fixed [#6029](https://github.com/biomejs/biome/issues/6029): A new line before the semicolon in the previous statement is now kept after formatting.

For example, the following code:

```js
const foo = 3

;[1, 2, 3].map(x => x * 2)
```

when `javascript.formatter.semicolons` is `always`, it becomes:

```js
const foo = 3;

[1, 2, 3].map(x => x * 2);
```

when `javascript.formatter.semicolons` is `asNeeded`, the original code is considered as already formatted.
