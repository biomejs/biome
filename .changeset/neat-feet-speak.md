---
"@biomejs/biome": minor
---

Added the `css.globals` option for stylesheets that a framework or build tool loads on every page without an import. Each entry is either a stylesheet path or a glob pattern relative to the configuration file that declares it. Glob patterns match only stylesheets and don't search `node_modules`; list files under `node_modules` with an explicit path. Biome warns when an entry is invalid, negated, unmatched, or names an unreadable stylesheet.

```json
{
  "css": {
    "globals": [
      "src/styles/reset.css",
      "src/styles/themes/**/*.css",
      "node_modules/@fontsource/roboto/index.css"
    ]
  },
  "overrides": [
    {
      "includes": ["src/admin/**"],
      "css": { "globals": ["src/styles/admin.css"] }
    }
  ]
}
```

The rules [`noUndeclaredClasses`](https://biomejs.dev/linter/rules/no-undeclared-classes/) and [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/) treat declarations from matching stylesheets as available to every file in each package, while [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/) checks their classes against references from files to which each stylesheet applies. Inside an override, the stylesheets apply only to files selected by that override. Configuring this option also enables these rules to report when Biome can't infer every global stylesheet loaded by a framework.
