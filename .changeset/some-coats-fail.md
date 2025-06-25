---
"@biomejs/biome": patch
---

Added the new nursery rule [`useUnifiedTypeSignature`](https://biomejs.dev/linter/rules/use-unified-type-signature), which disallows overload signatures that can be unified into a single signature.

Overload signatures that can be merged into a single signature are redundant and should be avoided. This rule helps simplify function signatures by combining overloads by making parameters optional and/or using type unions.

**Example (Invalid): Overload signatures that can be unified:**

```ts
function f(a: number): void;
function f(a: string): void;
```

```ts
interface I {
    a(): void;
    a(x: number): void;
}
```

**Example (Valid): Unified signatures:**

```ts
function f(a: number | string): void {}
```

```ts
interface I {
    a(x?: number): void;
}
```

**Example (Valid): Different return types cannot be merged:**

```ts
interface I {
    f(): void;
    f(x: number): number;
}
```
