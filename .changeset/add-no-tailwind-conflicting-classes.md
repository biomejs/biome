---
"@biomejs/biome": patch
---

Added the nursery rule `noTailwindConflictingClasses` in the `tailwind` domain. It reports Tailwind utility classes that conflict by targeting the same CSS property in the same variant context.
