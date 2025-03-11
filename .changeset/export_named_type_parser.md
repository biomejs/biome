---
"@biomejs/biome": patch
---

Export Named Type support `default` parser.

The following code is now parsed successfully:

```ts
export { type A as default } from './b.ts';
```
