---
"@biomejs/biome": patch
---

Fixed [#12375](https://github.com/biomejs/biome/issues/12375): [`noSetterReturn`](https://biomejs.dev/linter/rules/no-setter-return/) now reports setters in property descriptors passed to `Object.defineProperty`, `Object.defineProperties`, `Object.create`, and `Reflect.defineProperty`, including an arrow function's implicit return.
