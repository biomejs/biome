---
"@biomejs/biome": minor
---

Added 2 options from `typescript-eslint` (`ignoreDifferentlyNamedParameters` and `ignoreDifferentJsDoc`) to [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures/).

Each will cause the rule to ignore overload signatures whose parameter names or JSDoc comments differ.

Example with `ignoreDifferentlyNamedParameters` set to `true`:

```ts
// With the option enabled, these won't trigger diagnostics due to differing names.
function cook(scoops: IceCreamScoop[]): void;
function cook(cakeType: string): void;
```

Example for `ignoreDifferentJsDoc` set to `true`:

```ts
// With the option enabled, these won't trigger diagnostics due to differing comment contents.
/** Does objs have "cow" inside it? */
function hasCow(objs: string[]): boolean;
/** @deprecated - convert to array */
function hasCow(objs: string): boolean;
```
