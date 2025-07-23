---
"@biomejs/biome": patch
---

Fixed [#6692](https://github.com/biomejs/biome/issues/6692): The rules `noUnusedVariables` and `noUnusedFunctionParameters` no longer cause an infinite loop when the suggested name is not applicable (e.g. the suggested name is already declared in the scope).
