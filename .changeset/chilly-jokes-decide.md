---
"@biomejs/biome": minor
---

Added a new assist action `useSortedInterfaceMembers` that sorts TypeScript interface members, for readability.

It includes an autofix.

Invalid example.

```ts,expect_diagnostic
interface MixedMembers {
  z: string;
  a: number;
  (): void;
  y: boolean;
}
```

Valid example (after using the assist).

```ts
interface MixedMembers {
  a: number;
  y: boolean;
  z: string;
  (): void;
}
```
