---
"@biomejs/biome": patch
---

Added a new nursery rule `useSortedInterfaceMembers` that enforces an ordering for TypeScript interface members.

The rule sorts interface members for readability. It includes an autofix.

Invalid example.

```ts,expect_diagnostic
interface MixedMembers {
  z: string;
  a: number;
  (): void;
  y: boolean;
}
```

Valid example (after the fix).

```ts
interface MixedMembers {
  a: number;
  y: boolean;
  z: string;
  (): void;
}
```
