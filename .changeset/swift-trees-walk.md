---
"@biomejs/biome": patch
---

Added `node.descendants(kinds)` and `node.walk(kinds, enter, exit)` to JavaScript plugins. They find the nodes inside a node whose kind is in a set created with `nodeKinds()`, and are much faster than visiting every node with `node.children()`.

```js
const CALLS = nodeKinds("JS_CALL_EXPRESSION");

for (const call of root.descendants(CALLS)) {
  registerDiagnostic(call, "information", "Found a call.");
}
```
