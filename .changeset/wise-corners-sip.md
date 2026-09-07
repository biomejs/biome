---
"@biomejs/biome": patch
---

Fixed [#8629](https://github.com/biomejs/biome/issues/8629), where [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) incorrectly reported used private TypeScript method overload signatures as unused.
