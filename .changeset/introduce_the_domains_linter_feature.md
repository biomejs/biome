---
"@biomejs/biome": minor
---

Introduce the `domains` linter feature. The Biome linter now has a new way to opt-in rules, with a concept called `domains`.

Domains can be seen as concepts shared by different rules.

You can enable and disable multiple rules that belong to a domain. When you assign `"all"`, Biome will enable all the rules, when you assign `"none"`, Biome will disable the rules, when you assign "recommended", Biome will enable all rules of the domain that are recommended.

```json5
// biome.jsonc
{
  "linter": {
    "domains": {
      "test": "all", // all rules that belong to this domain are enabled
      "react": "recommended", // only the recommended rules from this domain are enabled
      "solid": "none" // rules related to Solid are disabled
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
  And it will inject the following globals:
   - `after`
   - `afterAll`
   - `afterEach`
   - `before`
   - `beforeEach`
   - `beforeAll`
   - `describe`
   - `it`
   - `expect`
   - `test`
- `next`: it will enable rules for Next.js projects:
  - `useExhaustiveDependencies`
  - `useHookAtTopLevel`
  - `noImgElement`
  - `noHeadImportInDocument`
  - `noHeadImportInDocument`
- `react`: it will enable rules for React projects:
  - `useExhaustiveDependencies`
  - `useHookAtTopLevel`
- `solid`: it will enable rules for Solid projects:
  - `noReactSpecificProps`

For more information regarding how Biome enables rules via domains, please refer to the documentation page of each rule.
