---
"@biomejs/biome": patch
---

Fixed type inference for the intersection of a primitive type and one of its literal types, such as `string & "a"`, which Biome inferred as `never` instead of `"a"`. Type-aware rules such as [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) and [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now see the literal, so the following switch is reported for missing the `"a"` case.

```ts
type Kind = ("a" & string) | "b";

function handle(kind: Kind) {
  switch (kind) {
    case "b":
      break;
  }
}
```
