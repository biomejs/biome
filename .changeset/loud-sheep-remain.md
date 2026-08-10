---
"@biomejs/biome": patch
---

Added the nursery rule `useAstroClientOnlyDirectiveValue`, which reports Astro `client:only` directives without an initializer.

For example, `<Component client:only />` triggers the rule.
