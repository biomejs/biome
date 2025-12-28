---
"@biomejs/biome": patch
---

Hardened union static-member type flattening in edge cases (e.g. unions containing `unknown` or inferred expression types). This keeps inference conservative and avoids unstable type growth in `node = node.parent`-style loops.
