---
"@biomejs/js-api": patch
---

Fixed a bug where `formatContent()` and the other content APIs did not match `overrides` for paths that use Windows separators, such as `src\a.js`. The workspace runs inside WebAssembly, where a backslash is an ordinary character, so a path produced by `path.relative()` on Windows was compared against `src/**` as a single component and never matched. Reported in [#13303](https://github.com/biomejs/biome/issues/13303).
