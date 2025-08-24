---
"@biomejs/biome": minor
---

Fixed #6476: Added the new rule `noUselessCatchBindings`. This rule disallows unnecessary catch bindings.

```diff
try {
    // Do something
- } catch (unused) {}
+ } catch {}
```
