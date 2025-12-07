---
"@biomejs/biome": minor
---

Added 2 options from `typescript-eslint` (`ignoreDifferentlyNamedParameters` and `ignoreDifferentJsDoc`) to [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures/).

Each will cause the rule to ignore overload signatures whose parameter names or JSDoc comments differ.

Example with `ignoreDifferentlyNamedParameters` set to `true`:

```ts
// These overloads would normally trigger diagnostics despite signifying completely different quantities 
// with different parameter names.
// With the option enabled, they will be ignored.
function cook(type: FoodType.BURGER, meat: "beef" | "chicken"): void;
function cook(type: FoodType.CAKE, flavour: string): void;
function cook(type: FoodType.SHRIMP_COCKTAIL, sauces: string[]): void;
function cook(type: FoodType, ...params: unknown[]): void { }
```

Example for `ignoreDifferentJsDoc` set to `true`:

```ts
/** Does objs have "cow" inside it? */
function hasCow(objs: string[]): boolean;
/** @deprecated - convert to array */
function hasCow(objs: string): boolean;
```
