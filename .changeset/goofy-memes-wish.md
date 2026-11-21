---
"@biomejs/biome": patch
---

Fixed [#13251](https://github.com/biomejs/biome/issues/13251): parse diagnostics are now reported in source order, including mismatched nested JSX tags such as `<some><nested></some></nested>;`.

