---
"@biomejs/biome": patch
---

Fixed [#8345](https://github.com/biomejs/biome/issues/8345): [`useAdjacentOverloadSignatures`](https://biomejs.dev/linter/rules/use-adjacent-overload-signatures/) no longer reports false positives for static and instance methods with the same name. Static methods and instance methods are now treated as separate overload groups.

```ts
class Kek {
  static kek(): number { return 0 }
  another(): string { return '' }
  kek(): number { return 1 }  // no longer reported as non-adjacent
}
```
