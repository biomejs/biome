---
cli: minor
---

# Introduce the `domains` linter feature

The Biome linter now has a new way to opt-in rules, with a concept called `domains`.

Domains can be seen as concepts shared by different rules.

You can enable and disable multiple rules that belong to a domain. When you assign `true`, Biome will enable the rules, when you assign `false`, Biome will disable the rules.

```json5
// biome.jsonc
{
  "linter": {
    "domains": {
      "test": false, // rules around testing are disabled
      "next": true // rules around Next.js are disabled
    }
  }
}
```

New domains introduced:

- `test`: it will enable rules:
  - `noExportsInTest`
  - `noExcessiveNestedTestSuites`
  - `noDuplicateTestHooks`
  - `noFocusedTests`
- `next`: it will enable rules for Next.js:
  - `noImgElement`
  - `useExhaustiveDependencies`
  - `useHookAtTopLevel`
  - `noHeadImportInDocument`
  - `noHeadImportInDocument`
- `react`: it will enable rules for React.
  - `useExhaustiveDependencies`
  - `useHookAtTopLevel`
- `solid`: it will enable rules for Solid.
  - `noReactSpecificProps`

