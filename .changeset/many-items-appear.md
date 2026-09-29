---
"@biomejs/biome": patch
---

Fixed [#11897](https://github.com/biomejs/biome/issues/11897): the safe fix for [`noUselessStringConcat`](https://biomejs.dev/linter/rules/no-useless-string-concat/) now escapes embedded double quotes when combining literals, preserving valid JavaScript and existing escape sequences.
