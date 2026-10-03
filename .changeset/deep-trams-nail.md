---
"@biomejs/biome": patch
---

Fixed [#12026](https://github.com/biomejs/biome/issues/12026): [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now reports missing cases when the union type is imported from a module that also exports a value with the same name.

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
