---
"@biomejs/biome": patch
---

Added the nursery rule [`useNamedCaptureGroup`](https://biomejs.dev/linter/rules/use-named-capture-group/).
The rule enforces using named capture groups in regular expressions instead of numbered ones. It supports both regex literals and `RegExp` constructor calls.

```js
// Invalid: unnamed capture group
/(foo)/;
new RegExp("(foo)");

// Valid: named capture group
/(?<id>foo)/;
new RegExp("(?<id>foo)");
```
