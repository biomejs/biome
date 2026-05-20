---
"@biomejs/biome": patch
---

The `useUniqueElementIds` rule now ignores elements inside an SVG context. SVG elements use `id` attributes for local references (gradients, patterns, clip-paths) and don't need globally unique IDs. The rule walks up the JSX tree to detect `<svg>` ancestors and skips the check when found. HTML elements outside SVG contexts continue to be flagged as expected.

Closes #6206
