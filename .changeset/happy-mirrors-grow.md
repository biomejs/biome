---
"@biomejs/biome": patch
---

Added the [`noMultiStr`](https://biomejs.dev/linter/rules/no-multi-str) rule, which disallows creating multiline strings by escaping newlines.

**Invalid:**

```js
const foo =
	"Line 1\n\
Line 2";
```

**Valid:**

```js
const foo = "Line 1\nLine 2";
const bar = `Line 1
Line 2`;
```
