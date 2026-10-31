---
"@biomejs/biome": minor
---

Added the `css.globals` option, which lists the stylesheets that apply to every page without an import, such as the ones that a framework or a build tool adds, for example `"css": { "globals": ["src/styles/theme.css"] }`. The rules [`noUndeclaredClasses`](https://biomejs.dev/linter/rules/no-undeclared-classes/), [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/), and [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/) treat their classes and custom properties as available to every file, and they report again in packages where Biome can't find all the stylesheets that a framework adds. Biome now also shows an information diagnostic when it can't find these stylesheets, or the modules that a framework provides.
