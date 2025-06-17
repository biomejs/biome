---
"@biomejs/biome": patch
---

Type exports now support renaming types to `default`.

The following code is now parsed successfully:

```ts
export { type A as default } from './b.ts';
```
