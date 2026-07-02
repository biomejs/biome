---
"@biomejs/biome": patch
---

Fixed [#10698](https://github.com/biomejs/biome/issues/10698): The [`noUnsafeOptionalChaining`](https://biomejs.dev/linter/rules/no-unsafe-optional-chaining/) rule now reports unsafe optional chains wrapped in TypeScript `as`, `satisfies`, type assertion, and instantiation expressions, such as `new (value?.constructor as Constructor)()`.
