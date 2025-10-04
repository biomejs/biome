---
"@biomejs/biome": minor
---

Added the ability to show severity `Information` diagnostics in reporter outputs.

If one or more rules are triggered, and they are configured to emit an `Information` diagnostic, now they're counted in the final output:

```bash
Checked 1 file in <TIME>. No fixes applied.
Found 1 info.
```
