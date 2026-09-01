---
"@biomejs/biome": patch
---

Fixed an apostrophe or quote in the text of a JSX element inside an Astro expression ending the expression early, such as `{items.map((i) => <li>it's {i}</li>)}`.
