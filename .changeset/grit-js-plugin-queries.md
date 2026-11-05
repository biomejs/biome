---
"@biomejs/biome": patch
---

Added `grit()` queries to JavaScript plugins. A rule can now use a GritQL pattern to select the nodes passed to `run()` instead of listing syntax kinds with `ast()` or `semantic()`.

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

Grit sequence patterns call `run()` for each matched node; they support leaf tokens, and they reject patterns containing rewrites. Plugins using GritQL patterns should report diagnostics from `run()` as usual.

Plugins using Grit queries can still request the semantic model as well:

```ts
export const rule = defineRule({
  query: grit("JsReferenceIdentifier()", { semantic: true }),
  run(node, { model }) {
    // You can use `model` here similar to a query specified using `semantic()`.
  },
});
```
