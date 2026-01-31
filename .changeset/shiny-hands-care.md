---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentMethodSignatures`](https://biomejs.dev/linter/rules/use-consistent-method-signatures/). \
Inspired by the similarly named version from [`typescript-eslint`](https://typescript-eslint.io/rules/method-signature-style/), this rule aims to enforce a consistent style for methods used inside object types and interfaces.

### Examples

Invalid code with `style` set to `"property"` (the default):

```ts,expect_diagnostic
interface Foo {
  method(a: string): void;
}
```

Invalid code with `style` set to `"method"`:

```ts,expect_diagnostic
type Bar = {
  prop: (a: string) => void;
}
```
