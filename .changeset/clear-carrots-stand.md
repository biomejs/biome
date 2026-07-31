---
"@biomejs/biome": patch
---

Fixed [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/): Biome now chooses the same function overload as TypeScript when an argument is a callback, so conditions that were previously missed are reported.

The following code is now invalid, because a parameter typed `() => void` accepts an `async` callback and `schedule` therefore returns `string`:

```ts
declare function schedule(handler: () => void): string;
declare function schedule(handler: () => Promise<void>): string | undefined;

schedule(async () => {}) ?? "fallback";
```

The following code is also now invalid, because `map(() => 42)` returns `42`:

```ts
type Mapper<T> = () => T;
declare function map<T>(mapper: Mapper<T>): T;

map(() => 42) || flag;
```
