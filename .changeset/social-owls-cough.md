---
"@biomejs/biome": minor
---

Added `project/frameworks` informational diagnostics when Biome can't determine all the global stylesheets or modules provided by a framework. These diagnostics explain when [`noUndeclaredClasses`](https://biomejs.dev/linter/rules/no-undeclared-classes/), [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/), [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/), or [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) skip checks and what to configure or install before Biome can run them. For example, Biome now reports the following guidance.

```text
i Biome can't find all the stylesheets that the frameworks of this package add to every page.
i List these stylesheets in the css.globals option of the Biome configuration to enable the rules.
```

