---
"@biomejs/biome": patch
---

Fixed [#12026](https://github.com/biomejs/biome/issues/12026): when a module exports a `const`, `let`, `var`, or function and a type alias under the same name, with no namespace of that name, importers now use the type alias in type positions and the value under `typeof`, so [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) reports missing cases for the imported union. In type positions, such an alias built on `InstanceType` or `ReturnType` is treated as `any` until Biome evaluates those utility types.

The following `switch` is now reported, because it doesn't handle `"c"`:

```ts
// letters.ts
export const Letters = ["a", "b", "c"] as const;
export type Letters = (typeof Letters)[number];

// main.ts
import type { Letters } from "./letters";

function toNumber(letter: Letters): number {
  switch (letter) {
    case "a":
      return 1;
    case "b":
      return 2;
  }
  return 0;
}
```
