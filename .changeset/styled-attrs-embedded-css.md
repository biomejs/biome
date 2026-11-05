---
"@biomejs/biome": patch
---

Fixed [#12826](https://github.com/biomejs/biome/issues/12826): Biome now detects CSS in styled-components templates that use `.attrs()`, such as `` styled.button.attrs({ type: "button" })`...` `` and `` styled(Component).attrs(() => ({}))`...` ``, so they are formatted and linted like `` styled.button`...` ``.
