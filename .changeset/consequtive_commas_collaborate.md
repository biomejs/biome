---
"@biomejs/biome": minor
---

Type inference is now able to handle the sequence operator (`,`), as well as
post- and pre-update operators: `++`.


## Examples

```ts
let x = 5;

// We now infer that `x++` resolves to a number, while the expression as a whole
// becomes a Promise:
x++, new Promise((resolve) => resolve('comma'));
```
