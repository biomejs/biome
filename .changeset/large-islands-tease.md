---
"@biomejs/biome": patch
---

Added the rule [`noNonNullAssertedOptionalChain`](https://biomejs.dev/linter/rules/no-non-null-asserted-optional-chain).

This rule prevents the use of non-null assertions (`!`) immediately after optional chaining expressions (`?.`). Optional chaining is designed to safely handle nullable values by returning `undefined` when the chain encounters `null` or `undefined`. Using a non-null assertion defeats this purpose and can lead to runtime errors.

```ts
// Invalid - non-null assertion after optional chaining
obj?.prop!;
obj?.method()!;
obj?.[key]!;
(obj?.prop)!;

// Valid - proper optional chaining usage
obj?.prop;
obj?.method();
obj?.prop ?? defaultValue;
obj!.prop?.method();
