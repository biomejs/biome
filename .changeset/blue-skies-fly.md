---
"@biomejs/biome": patch
---

Fix [#4317](https://github.com/biomejs/biome/issues/4317), setter parameter can contain a trailing comma, the following example will now parsed correctly:

```ts
export class DummyClass {
  set input(
    value: string,
  ) {}
}
```
