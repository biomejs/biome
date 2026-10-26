---
"@biomejs/biome": patch
---

Fixed [#12651](https://github.com/biomejs/biome/issues/12651): an imported name that the other module exports as both a type and a value now resolves to the type in type annotations, so [`noBaseToString`](https://biomejs.dev/linter/rules/no-base-to-string/) no longer reports `slug` below as an object.

```ts
// rule-slug.ts
export type RuleSlug = string;
export const RuleSlug = { create: (text: string): RuleSlug => text } as const;

// main.ts
import { RuleSlug } from "./rule-slug";
export const label = (slug: RuleSlug): string => `rule ${slug}`;
```
