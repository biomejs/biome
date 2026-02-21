---
"@biomejs/biome": patch
---

Added the nursery rule `useVueScopedStyles` for Vue SFCs. This rule enforces that `<style>` blocks have the `scoped` attribute (or `module` for CSS Modules), preventing style leakage and conflicts between components.
