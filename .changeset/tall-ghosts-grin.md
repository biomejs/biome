---
"@biomejs/biome": patch
---

Fixed handling of `biome-ignore format` suppression comments on TypeScript declared class properties with string literal names.

```ts
class A {
	declare /* biome-ignore format: exercise suppression checking */ 'a-b': 0;
}
```
