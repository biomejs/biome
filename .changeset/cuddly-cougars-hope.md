---
"@biomejs/biome": patch
---

Added the nursery rule `noInvertedTernary` to detect inverted ternary expressions that reduce readability, such as `const x = a !== b ? first : second;`. The rule suggests using a direct condition by flipping the operator and branches (for example, `a === b ? second : first`).
