---
"@biomejs/biome": patch
---

Added the [`useConsistentTypeDefinitions`](https://biomejs.dev/rules/use-consistent-type-definitions) rule.

This rule enforces consistent usage of either `interface` or `type` for object type definitions in TypeScript.

The rule accepts an option to specify the preferred style:
- `interface` (default): Prefer using `interface` for object type definitions
- `type`: Prefer using `type` for object type definitions

Examples:

```ts
// With default option (interface)
// ❌ Invalid
type Point = { x: number; y: number; };

// ✅ Valid  
interface Point {
  x: number;
  y: number;
}

// With option { style: "type" }
// ❌ Invalid
interface Point {
  x: number;
  y: number;
}

// ✅ Valid
type Point = { x: number; y: number; };
```

The rule will automatically fix simple cases where conversion is straightforward.

