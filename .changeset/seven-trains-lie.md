---
"@biomejs/biome": minor
---

The `--reporter=summary` has been greatly enhanced. It now shows the list of files that contains violations, the files shown are clickable and can be opened from the editor.

Below an example of the new version:

```
reporter/parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  i The following files have parsing errors.

  - index.css

reporter/format ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  i The following files needs to be formatted.

  - index.css
  - index.ts
  - main.ts

reporter/violations ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  i Some lint rules or assist actions reported some violations.

  Rule Name                                        Diagnostics

  lint/correctness/noUnknownFunction               14 (2 error(s), 12 warning(s), 0 info(s))
  lint/suspicious/noImplicitAnyLet                 16 (12 error(s), 4 warning(s), 0 info(s))
  lint/suspicious/noDoubleEquals                   8 (8 error(s), 0 warning(s), 0 info(s))
  assist/source/organizeImports                    2 (2 error(s), 0 warning(s), 0 info(s))
  lint/suspicious/noRedeclare                      12 (12 error(s), 0 warning(s), 0 info(s))
  lint/suspicious/noDebugger                       8 (8 error(s), 0 warning(s), 0 info(s))

```
