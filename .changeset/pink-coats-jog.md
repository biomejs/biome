---
"@biomejs/biome": minor
---

Added the new nursery rule `noUselessCatchBinding`. This rule disallows unnecessary catch bindings.

```diff
try {
    // Do something
- } catch (unused) {}
+ } catch {}
```
