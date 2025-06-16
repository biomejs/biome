---
"@biomejs/biome": minor
---

Enhanced the command `migrate eslint`. Now the command shows which ESLint rules were migrated,
and which rules aren't supported yet.

```
./eslint.config.js migrate ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ℹ 42% (3/7) of the rules have been migrated.

  ℹ Migrated rules:

  - getter-return
  - prefer-const
  - @typescript-eslint/require-await

  ℹ Rules that can be migrated to an inspired rule using --include-inspired:

  - @typescript-eslint/parameter-properties

  ℹ Rules that can be migrated to a nursery rule using --include-nursery:

  - @typescript-eslint/switch-exhaustiveness-check

  ℹ Stylistic rules that the formatter may support (manual migration required):
  
  - semi

  ℹ Unsupported rules:

  - block-scoped-var

configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ℹ Migration results:

  - ./biome.json: configuration successfully migrated.
```
