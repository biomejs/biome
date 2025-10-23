---
"@biomejs/biome": minor
---

The following rules are now a part of the `react` domain, and they won't be enabled automatically unless you enabled the domain, or Biome detects `react` as a dependency of your closest `package.json`:

- [`lint/correctness/noChildrenProp`](https://biomejs.dev/linter/rules/no-children-prop/) (recommended)
- [`lint/correctness/noReactPropAssignments`](https://biomejs.dev/linter/rules/no-react-prop-assignments/)
- [`lint/security/noDangerouslySetInnerHtml`](https://biomejs.dev/linter/rules/no-dangerously-set-inner-html/) (recommended)
- [`lint/security/noDangerouslySetInnerHtmlWithChildren`](https://biomejs.dev/linter/rules/no-dangerously-set-inner-html-with-children/) (recommended)
- [`lint/style/useComponentExportOnlyModules`](https://biomejs.dev/linter/rules/use-component-export-only-modules/)
- [`lint/suspicious/noArrayIndexKey`](https://biomejs.dev/linter/rules/no-array-index-key/) (recommended)
