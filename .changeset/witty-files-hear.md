---
"@biomejs/biome": patch
---

Fixed [#9625](https://github.com/biomejs/biome/issues/9625): `experimentalEmbeddedSnippetsEnabled` no longer crashes when a file mixes formatable CSS-in-JS templates with tagged templates that the embedded formatter can't currently delegate, such as a styled-components interpolation returning `css```.
