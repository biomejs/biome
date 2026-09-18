---
"@biomejs/biome": patch
---

Fixed [#10697](https://github.com/biomejs/biome/issues/10697): The formatter no longer removes the parentheses around an `await` or `yield` expression used as the target of a TypeScript instantiation expression. For example, `(await makeFactory)<Value>` is no longer reformatted to `await makeFactory<Value>`, which would change the meaning of the code.
