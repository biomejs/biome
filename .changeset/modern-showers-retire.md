---
"@biomejs/biome": patch
---

Fixed [#6974](https://github.com/biomejs/biome/issues/6974), where [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) incorrectly reported TypeScript private constructor properties read through object destructuring from `this` as unused.
