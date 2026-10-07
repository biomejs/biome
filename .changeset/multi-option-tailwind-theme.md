---
"@biomejs/biome": patch
---

Fixed parsing of Tailwind `@theme` at-rules with more than one option. Biome now accepts any number of options, including `prefix(...)`, such as the `@theme default inline reference { ... }` rule in Tailwind's own `tailwindcss/index.css`.
