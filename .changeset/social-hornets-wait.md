---
"@biomejs/biome": minor
---

Added the ability to show severity `Information` diagnostics in reporter outputs.

If one of more rules are triggered, and they are configured to emit an `Information` diagnostic, now their counted in the final output:

```bash
Checked 1 file in <TIME>. No fixes applied.
Found 1 info.
```
