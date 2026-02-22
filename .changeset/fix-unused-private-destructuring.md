---
"@biomejs/biome": patch
---

Fixed [#9008](https://github.com/biomejs/biome/issues/9008). The [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) rule no longer reports false positives when TypeScript `private` class members are accessed via destructuring from `this`. Patterns such as `const { myVar } = this;`, `const { myVar: renamed } = this;`, and `({ myVar } = this)` are now correctly recognized as usage.
