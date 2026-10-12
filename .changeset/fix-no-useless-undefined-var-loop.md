---
"@biomejs/biome": patch
---

Fixed a false positive in [`noUselessUndefinedInitialization`](https://biomejs.dev/linter/rules/no-useless-undefined-initialization/) where `var` declarations initialized to `undefined` inside loops, in redeclarations, or after prior reassignments were incorrectly flagged and safe-fixed.
