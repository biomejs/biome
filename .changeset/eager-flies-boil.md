---
"@biomejs/biome": patch
---

Fixed [#7192](https://github.com/biomejs/biome/issues/7192): [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) now considers compound assignments such as `??=` to read and use private class members.
