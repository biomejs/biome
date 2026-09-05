---
"@biomejs/biome": patch
---

Reduced unnecessary type inference when type-aware lint rules inspect members of namespace imports from libraries such as Zod. Fixed type inference so blanket re-exports do not expose default exports.
