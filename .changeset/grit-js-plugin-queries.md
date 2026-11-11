---
"@biomejs/biome": patch
---

Added `grit()` queries to JavaScript plugins. A rule can now use a GritQL pattern to select the nodes passed to `run()` instead of listing syntax kinds with `ast()`.

For example:

```ts
export const noConsole = defineRule({
  query: grit("`console.log($args)`"),
  run(element) {
    if (element.kind !== "JS_CALL_EXPRESSION") return;
    registerDiagnostic(element, "warning", "Unexpected console.log.");
  },
});
```

Usage of Grit patterns can be particularly useful when it would take several checks to verify the shape of a snippet that involves multiple AST nodes.

Grit sequence patterns call `run()` for each matched node. Matching on leaf tokens is also supported for patterns, while patterns containing rewrites are rejected. Plugins using GritQL patterns should report diagnostics from `run()` as usual.

Plugins using Grit queries can still request the semantic model as well:

```ts
export const rule = defineRule({
  query: grit("function $fn($args) { $body }", { semantic: true }),
  run(node, { model }) {
    // You can use `model` here similar to a query specified using `ast(..., { semantic: true })`.
  },
});
```
