---
"@biomejs/biome": patch
---

[`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now checks switch statements over object literal properties initialized with `as const`.

This switch is now reported because `status.kind` is inferred as the string literal `"ready"` but no case handles it:

```ts
const status = { kind: "ready" as const };
switch (status.kind) {}
```
