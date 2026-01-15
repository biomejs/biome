---
"@biomejs/biome": patch
---

Added the rule [`useUnicodeRegex`](https://biomejs.dev/linter/rules/use-unicode-regex/).

The rule enforces the use of the `u` or `v` flag for regular expressions. This ensures proper handling of Unicode characters like emoji.

```js
// Invalid
/foo/;
new RegExp("foo", "gi");

// Valid
/foo/u;
new RegExp("foo", "giu");
```
