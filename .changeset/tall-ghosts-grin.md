---
"@biomejs/biome": patch
---

Fixed handling of `biome-ignore format` suppression comments on TypeScript declared class properties with string literal names.

```ts
declare /* biome-ignore format: reason */ "role-admin": Array<number>;
```
