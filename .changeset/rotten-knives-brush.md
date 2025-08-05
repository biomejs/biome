---
"@biomejs/biome": patch
---

Fixed [#7101](https://github.com/biomejs/biome/issues/7101): [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) now handles members declared as part of constructor arguments:

1. If a class member defined in a constructor argument is only used within the constructor, it removes the `private` modifier and makes it a plain method argument.
1. If it is not used at all, it will prefix it with an underscore, similar to `noUnusedFunctionParameter`.
