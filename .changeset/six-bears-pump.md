---
"@biomejs/biome": patch
---

Type inference accuracy has been improved significantly. More global types, like `Array`, `Map`, `Set`, etc., are now fully defined. This should decrease false positives on all typed rules, since less types will be unknown.
