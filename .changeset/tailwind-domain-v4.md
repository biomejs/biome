---
"@biomejs/biome": minor
---

Changed the dependency that enables the [`tailwind` domain](https://biomejs.dev/linter/domains/#tailwind) automatically. The domain is now enabled when `package.json` declares `tailwindcss` v4 or later, instead of v3 or later, because we are currently targeting Tailwind v4 support. The domain can always be enabled manually.
