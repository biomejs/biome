---
"@biomejs/biome": patch
---

Fixed [#7882](https://github.com/biomejs/biome/issues/7882): The HTML parser will now emit better diagnostics when it encounters a void element with a closing tag, such as `<br></br>`. Previously, the parser would emit multiple diagnostics with conflicting advice. Now it emits a single diagnostic that clearly states that void elements should not have closing tags.
