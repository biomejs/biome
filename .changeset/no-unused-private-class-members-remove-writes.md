---
"@biomejs/biome": patch
---

Fixed [#8185](https://github.com/biomejs/biome/issues/8185): the unsafe fix of [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) now also removes the statements that only write to the removed member, such as `this.#member = value;` and `this.#member++;`. Previously, these statements were left behind and referenced a member that no longer existed.
