---
"@biomejs/biome": minor
---

This PR ports the `useValidAnchor` rule to HTML. As it's not possible at the moment to detect, whether or not a given variable is `null` or `undefined`, those cases are not covered.
