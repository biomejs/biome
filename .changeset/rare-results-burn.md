---
"@biomejs/biome": patch
---

Fixed false positive in the rule [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function) where the [`tech`](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face/src#tech) function was incorrectly flagged as an unknown function.
