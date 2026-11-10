---
"@biomejs/biome": patch
---

Removed the `semantic()` query function from the JavaScript plugin API. Pass `{ semantic: true }` to `ast()` instead to provide the semantic model to a rule.

```ts
export const rule = defineRule({
  query: semantic("JS_REFERENCE_IDENTIFIER"),
  run(node, { model }) {
    // Use `model` here.
  },
});
```

Becomes:

```ts
export const rule = defineRule({
  query: ast("JS_REFERENCE_IDENTIFIER", { semantic: true }),
  run(node, { model }) {
    // Use `model` here.
  },
});
```

This behaviour matches the `grit()` query function, which also accepts the `{ semantic: true }` option.
