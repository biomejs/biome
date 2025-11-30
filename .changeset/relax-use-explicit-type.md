---
"@biomejs/biome": patch
---

feat(useExplicitType): Relax rule for trivially inferrable types

Allow type annotations to be omitted when types are trivially inferrable from:
- Binary expressions (`const sum = 1 + 1`)
- Comparison expressions (`const isEqual = 'a' === 'b'`)
- Logical expressions (`const and = true && false`)
- Class instantiation (`const date = new Date()`)
- Array literals (`const arr = [1, 2, 3]`)
- Conditional expressions (`const val = true ? 'yes' : 'no'`)
- Function calls (`const num = Math.random()`)
- Parameter defaults (`const fn = (word = 'hello') => word`)
