---
"@biomejs/biome": patch
---

Fixed [#7495](https://github.com/biomejs/biome/issues/7495): [`noUselessConstructor`](https://biomejs.dev/linter/rules/no-useless-constructor/) now ignores TypeScript constructors that forward at least one argument to `super`, preserving constructors that narrow the subclass's accepted parameter types. The exemption also applies when the parent and child signatures are identical; JavaScript and zero-argument forwarding behavior are unchanged.
