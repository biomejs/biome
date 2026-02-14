---
"@biomejs/biome": minor
---

Added 2 options from `typescript-eslint` (`ignoreDifferentlyNamedParameters` and `ignoreDifferentJsDoc`) to [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures/).

Each option makes the rule ignore overload signatures whose parameter names or JSDoc comments differ.

#### Examples

Valid code with `ignoreDifferentlyNamedParameters` set to `true`:

```ts
function cook(scoops: IceCreamScoop[]): void;
function cook(cakeType: string): void;
```

Valid code with `ignoreDifferentJsDoc` set to `true`:

```ts
/** Does objs have "cow" inside it? */
function hasCow(objs: string[]): boolean;
/** @deprecated - convert to array */
function hasCow(objs: string): boolean;
```
