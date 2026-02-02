---
"@biomejs/biome": patch
---

Fixed [#8354](https://github.com/biomejs/biome/issues/8354): Don't remove quotes when type memeber is new.

```ts
// Input:
type X = {
    'new'(): string;
    'foo'(): string;
};

// Format Output:
type X = {
    'new()': string;
    foo(): string;
}
```
