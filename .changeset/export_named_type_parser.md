---
cli: patch
biome_js_parser: patch
---

# Export Named Type support `default` parser

The following code:

```ts
export { type A as default } from './b.ts';
```

Should be parsed successfully.