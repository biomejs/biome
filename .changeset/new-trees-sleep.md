---
"@biomejs/biome": patch
---

Fixed [#10897](https://github.com/biomejs/biome/issues/10897): [`noNegationInEqualityCheck`](https://biomejs.dev/linter/rules/no-negation-in-equality-check/) now properly unwraps nested parenthesized expressions before checking for negation. Previously, `(!foo) === bar` was not flagged when preceded by `!foo === bar` without a semicolon.
