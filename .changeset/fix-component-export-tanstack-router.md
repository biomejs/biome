---
"@biomejs/biome": patch
---

Fixed [#8628](https://github.com/biomejs/biome/issues/8628): [`useComponentExportOnlyModules`](https://biomejs.dev/linter/rules/use-component-export-only-modules/) now allows components referenced as object property values in exported expressions. This fixes false positives for TanStack Router patterns.

```jsx
export const Route = createFileRoute('/')({
  component: HomeComponent,
})

function HomeComponent() { ... } // no longer reported as "should be exported"
```
