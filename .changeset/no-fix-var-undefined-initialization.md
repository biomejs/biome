---
"@biomejs/biome": patch
---

Fixed [#11938](https://github.com/biomejs/biome/issues/11938): [`noUselessUndefinedInitialization`](https://biomejs.dev/linter/rules/no-useless-undefined-initialization/) no longer offers a fix for `var` declarations. Removing `= undefined` from a `var` declaration inside a loop or from a redeclaration changed the behavior of the program, because `var x;` doesn't reset the variable.
