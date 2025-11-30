---
"@biomejs/biome": patch
---

feat(useExplicitType): Relax rule for trivially inferrable types

Allow type annotations to be omitted when types are trivially inferrable from:
- Binary expressions (`const sum = 1 + 1`)
- Comparison expressions (`const isEqual = 'a' === 'b'`, `const isTest = process.env.NODE_ENV === 'test'`)
- Logical expressions (`const and = true && false`)
- Class instantiation (`const date = new Date()`)
- Array literals (`const arr = [1, 2, 3]`)
- Conditional expressions (`const val = true ? 'yes' : 'no'`)
- Function calls (`const num = Math.random()`)
- Parameter defaults - any expression is now allowed (`const fn = (max = MAX_ATTEMPTS) => ...`)

Comparison expressions always return `boolean`, so any operands are now allowed
(including property access like `process.env.NODE_ENV`).

Parameters with default values no longer require type annotations, as TypeScript
can infer the type from the default value (even when referencing variables).

Also removed the redundant `any` type validation from this rule. The `any` type 
is now only validated by the dedicated `noExplicitAny` rule, following the 
Single Responsibility Principle.
