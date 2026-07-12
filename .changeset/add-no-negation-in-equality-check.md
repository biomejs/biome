---
"@biomejs/biome": patch
---

Added the new lint rule `suspicious/noNegationInEqualityCheck`. This rule detects confusing negations on the left-hand side of equality expressions, such as `!a === b`.
