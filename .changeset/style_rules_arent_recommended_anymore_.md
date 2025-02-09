---
"@biomejs/biome": major
---

The `style` rules aren't recommended anymore.

Linting rules that belong to the group `style` aren't recommended anymore. Here's the list of rules that aren't recommended anymore:

- `useNumberNamespace`
- `noNonnullAssertion`
- `useAsConstAssertion`
- `noParameterAssign`
- `noInferrableTypes`
- `useNodejsImportProtocol`
- `useExportType`
- `useDefaultParameterLast`
- `noUnusedTemplateLiteral`
- `useExponentiationOperator`
- `useEnumInitializers`
- `useShorthandFunctionType`
- `useLiteralEnumMembers`
- `noVar`
- `noUselessElse`
- `useNumericLiterals`
- `noCommaOperator`
- `useConst`
- `noArguments`
- `useSelfClosingElements`
- `useImportType`
- `useTemplate`
- `useSingleVarDeclarator`
- `useWhile`

Use `biome migrate` to enable these rules, to avoid breaking changes.
