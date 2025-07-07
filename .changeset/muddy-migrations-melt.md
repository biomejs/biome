---
"@biomejs/biome": minor
---

`biome migrate` no longer enables style rules that were recommended in v1,
because that would be undesirable for users upgrading from 2.0.

Users who are upgrading from Biome 1.x are therefore advised to first upgrade to
Biome 2.0, and run the migration, before continuing to Biome 2.1 or later.
