---
"@biomejs/biome": minor
---

Enhanced the command `migrate eslint`. Now the command shows which ESLint rules were migrated,
and which rules aren't supported yet.

```
/eslint.config.js migrate ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ℹ 50% (3/6) of the rules have been migrated.

  ℹ Migrated rules:

  - getter-return
  - prefer-const
  - @typescript-eslint/require-await

  ℹ Rules that can be migrated to an inspired rule using --include-inspired:

  - @typescript-eslint/parameter-properties

  ℹ Rules that can be migrated to a nursery rule using --include-nursery:

  - @typescript-eslint/switch-exhaustiveness-check

  ℹ Unsupported rules:

  - semi

configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ℹ Migration results:

  - /biome.json: configuration successfully migrated.
```
