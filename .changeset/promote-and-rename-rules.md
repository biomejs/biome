---
"@biomejs/biome": minor
---

Promoted multiple lint rules from nursery to stable groups and renamed several rules for consistency.

#### Promoted rules

The following rules have been promoted from nursery to stable groups:

##### CSS

- Promoted [`noImportantStyles`](https://biomejs.dev/linter/rules/no-important-styles) to the `complexity` group.
- Promoted [`noUnknownAtRules`](https://biomejs.dev/linter/rules/no-unknown-at-rules) to the `suspicious` group.
##### GraphQL

- Promoted [`useGraphqlNamedOperations`](https://biomejs.dev/linter/rules/use-graphql-named-operations) to the `correctness` group.
- Promoted [`useGraphqlNamingConvention`](https://biomejs.dev/linter/rules/use-graphql-naming-convention) to the `style` group.
##### JavaScript/TypeScript

- Promoted [`noExcessiveLinesPerFunction`](https://biomejs.dev/linter/rules/no-excessive-lines-per-function) to the `complexity` group.
- Promoted [`noImplicitCoercions`](https://biomejs.dev/linter/rules/no-implicit-coercions) to the `complexity` group.
- Promoted [`useIndexOf`](https://biomejs.dev/linter/rules/use-index-of) to the `complexity` group.
- Promoted [`noGlobalDirnameFilename`](https://biomejs.dev/linter/rules/no-global-dirname-filename) to the `correctness` group.
- Promoted [`noNestedComponentDefinitions`](https://biomejs.dev/linter/rules/no-nested-component-definitions) to the `correctness` group.
- Promoted [`noProcessGlobal`](https://biomejs.dev/linter/rules/no-process-global) to the `correctness` group.
- Promoted [`noReactPropAssignments`](https://biomejs.dev/linter/rules/no-react-prop-assignments) to the `correctness` group.
- Promoted [`noRestrictedElements`](https://biomejs.dev/linter/rules/no-restricted-elements) to the `correctness` group.
- Promoted [`noSolidDestructuredProps`](https://biomejs.dev/linter/rules/no-solid-destructured-props) to the `correctness` group.
- Promoted [`useJsonImportAttributes`](https://biomejs.dev/linter/rules/use-json-import-attributes) to the `correctness` group.
- Promoted [`useParseIntRadix`](https://biomejs.dev/linter/rules/use-parse-int-radix) to the `correctness` group.
- Promoted [`useSingleJsDocAsterisk`](https://biomejs.dev/linter/rules/use-single-js-doc-asterisk) to the `correctness` group.
- Promoted [`useUniqueElementIds`](https://biomejs.dev/linter/rules/use-unique-element-ids) to the `correctness` group.
- Promoted [`noAwaitInLoops`](https://biomejs.dev/linter/rules/no-await-in-loops) to the `performance` group.
- Promoted [`noUnwantedPolyfillio`](https://biomejs.dev/linter/rules/no-unwanted-polyfillio) to the `performance` group.
- Promoted [`useGoogleFontPreconnect`](https://biomejs.dev/linter/rules/use-google-font-preconnect) to the `performance` group.
- Promoted [`useSolidForComponent`](https://biomejs.dev/linter/rules/use-solid-for-component) to the `performance` group.
- Promoted [`noMagicNumbers`](https://biomejs.dev/linter/rules/no-magic-numbers) to the `style` group.
- Promoted [`useConsistentObjectDefinitions`](https://biomejs.dev/linter/rules/use-consistent-object-definitions) to the `style` group.
- Promoted [`useExportsLast`](https://biomejs.dev/linter/rules/use-exports-last) to the `style` group.
- Promoted [`useGroupedAccessorPairs`](https://biomejs.dev/linter/rules/use-grouped-accessor-pairs) to the `style` group.
- Promoted [`useNumericSeparators`](https://biomejs.dev/linter/rules/use-numeric-separators) to the `style` group.
- Promoted [`useObjectSpread`](https://biomejs.dev/linter/rules/use-object-spread) to the `style` group.
- Promoted [`useReadonlyClassProperties`](https://biomejs.dev/linter/rules/use-readonly-class-properties) to the `style` group.
- Promoted [`useSymbolDescription`](https://biomejs.dev/linter/rules/use-symbol-description) to the `style` group.
- Promoted [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures) to the `style` group.
- Promoted [`noBitwiseOperators`](https://biomejs.dev/linter/rules/no-bitwise-operators) to the `suspicious` group.
- Promoted [`noConstantBinaryExpressions`](https://biomejs.dev/linter/rules/no-constant-binary-expressions) to the `suspicious` group.
- Promoted [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore) to the `suspicious` group.
- Promoted [`noUnassignedVariables`](https://biomejs.dev/linter/rules/no-unassigned-variables) to the `suspicious` group.
- Promoted [`noUselessRegexBackrefs`](https://biomejs.dev/linter/rules/no-useless-regex-backrefs) to the `suspicious` group.
- Promoted [`noUselessStringEscapes`](https://biomejs.dev/linter/rules/no-useless-string-escapes) to the `suspicious` group.
- Promoted [`useConsistentIterableCallbackReturnValues`](https://biomejs.dev/linter/rules/use-consistent-iterable-callback-return-values) to the `suspicious` group.
- Promoted [`useStaticResponseMethods`](https://biomejs.dev/linter/rules/use-static-response-methods) to the `suspicious` group.

#### Renamed rules

The following rules have been renamed during promotion. The migration tool will automatically update your configuration:

- Renamed `noAwaitInLoop` to [`noAwaitInLoops`](https://biomejs.dev/linter/rules/no-await-in-loops).
- Renamed `noConstantBinaryExpression` to [`noConstantBinaryExpressions`](https://biomejs.dev/linter/rules/no-constant-binary-expressions).
- Renamed `noDestructuredProps` to [`noSolidDestructuredProps`](https://biomejs.dev/linter/rules/no-solid-destructured-props).
- Renamed `noImplicitCoercion` to [`noImplicitCoercions`](https://biomejs.dev/linter/rules/no-implicit-coercions).
- Renamed `noReactPropAssign` to [`noReactPropAssignments`](https://biomejs.dev/linter/rules/no-react-prop-assignments).
- Renamed `noUnknownAtRule` to [`noUnknownAtRules`](https://biomejs.dev/linter/rules/no-unknown-at-rules).
- Renamed `noUselessBackrefInRegex` to [`noUselessRegexBackrefs`](https://biomejs.dev/linter/rules/no-useless-regex-backrefs).
- Renamed `useAdjacentGetterSetter` to [`useGroupedAccessorPairs`](https://biomejs.dev/linter/rules/use-grouped-accessor-pairs).
- Renamed `useConsistentObjectDefinition` to [`useConsistentObjectDefinitions`](https://biomejs.dev/linter/rules/use-consistent-object-definitions).
- Renamed `useConsistentResponse` to [`useStaticResponseMethods`](https://biomejs.dev/linter/rules/use-static-response-methods).
- Renamed `useForComponent` to [`useSolidForComponent`](https://biomejs.dev/linter/rules/use-solid-for-component).
- Renamed `useJsonImportAttribute` to [`useJsonImportAttributes`](https://biomejs.dev/linter/rules/use-json-import-attributes).
- Renamed `useNamedOperation` to [`useGraphqlNamedOperations`](https://biomejs.dev/linter/rules/use-graphql-named-operations).
- Renamed `useNamingConvention` to [`useGraphqlNamingConvention`](https://biomejs.dev/linter/rules/use-graphql-naming-convention).
- Renamed `useUnifiedTypeSignature` to [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures).

Configuration files using the old rule names will need to be updated. Use the migration tool to automatically update your configuration:

```bash
biome migrate --write
```
