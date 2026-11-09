---
"@biomejs/biome": patch
---

Added type information to JavaScript plugins. Rules can now request inferred types with the `types` option of `ast()` and `grit()`, and query them through `context.types`.

For example:

```ts
export const rule = defineRule({
  query: ast("JS_CALL_EXPRESSION", { types: true }),
  run(call, { types }) {
    if (types.typeOf(call.callee)?.isPromise() !== true) return;
    registerDiagnostic(call, "warning", "Await this promise.");
  },
});
```

The `types` option also provides the semantic model. Type information requires the `types` domain to be enabled, for example with `linter.domains.types` set to `"all"` in the configuration.
