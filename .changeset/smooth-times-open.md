---
"@biomejs/biome": patch
---

Fixed [#11352](https://github.com/biomejs/biome/issues/11352): [`useExplicitLengthCheck`](https://biomejs.dev/linter/rules/use-explicit-length-check/) no longer reports `length`-like properties used as value-producing `||` fallbacks or optional chains, and it no longer offers fixes for value-producing `&&` checks or unsafe negations.
