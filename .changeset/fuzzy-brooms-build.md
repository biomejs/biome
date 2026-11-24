---
"@biomejs/biome": patch
---

[#13384](https://github.com/biomejs/biome/issues/13384): Fixed Svelte lint diagnostics for shorthand attributes and event handlers: `{id}` can duplicate an explicit `id`, and `{onclick}` is recognized as a click handler. Accessibility checks no longer report an empty link or heading solely because its content comes from a child component.
