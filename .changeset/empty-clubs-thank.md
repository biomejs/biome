---
"@biomejs/biome": patch
---

Fixed the Tailwind class parser rejecting arbitrary variants whose selector starts with an attribute selector. Classes like `[[data-variant=legend]+&]:-mt-1.5` and `group-has-[[data-slot=item-description]]/item:self-start` now parse correctly.
