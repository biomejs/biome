---
"@biomejs/biome": patch
---

Fixed [#7130](https://github.com/biomejs/biome/issues/7130). Removed the emission of a false-positive diagnostic. Biome no longer emits the following diagnostic:
```
lib/main.ts:1:5 suppressions/unused ━━━━━━━━━━━━━━━━━━━━━━━━━

  ⚠ Suppression comment has no effect because the tool is not enabled.

  > 1 │ /** biome-ignore-all assist/source/organizeImports: For the lib root file, we don't want to organize exports */
      │     ^^^^^^^^^^^^^^^^

```
