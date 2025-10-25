---
"@biomejs/biome": patch
---

Fixed #7848: The css parser with `tailwindDirectives` enabled will now correctly parse tailwind's source exclude syntax: `@source not "foo.css";`
