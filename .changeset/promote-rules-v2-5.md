---
"@biomejs/biome": minor
---

Promoted 73 nursery rules to stable groups.

Four rules were renamed as part of the promotion:

- `noFloatingClasses` is now [`noUnusedInstantiation`](https://biomejs.dev/linter/rules/no-unused-instantiation/), because the rule checks any discarded `new` expression, not only classes.
- `noMultiStr` is now [`noMultilineString`](https://biomejs.dev/linter/rules/no-multiline-string/).
- `useFind` is now [`useArrayFind`](https://biomejs.dev/linter/rules/use-array-find/).
- `useSpread` is now [`useSpreadOverApply`](https://biomejs.dev/linter/rules/use-spread-over-apply/), because the rule enforces spread call arguments over `Function.apply()`, not array or object spread.

#### Correctness

Promoted the following rules to the `correctness` group:

- [`noBeforeInteractiveScriptOutsideDocument`](https://biomejs.dev/linter/rules/no-before-interactive-script-outside-document/)
- [`noUnusedInstantiation`](https://biomejs.dev/linter/rules/no-unused-instantiation/)
- [`useInlineScriptId`](https://biomejs.dev/linter/rules/use-inline-script-id/) (recommended, Next.js domain)
- [`noVueVIfWithVFor`](https://biomejs.dev/linter/rules/no-vue-v-if-with-v-for/) (recommended, Vue domain)
- [`useVueValidVBind`](https://biomejs.dev/linter/rules/use-vue-valid-v-bind/) (recommended, Vue domain)
- [`useVueValidVElse`](https://biomejs.dev/linter/rules/use-vue-valid-v-else/) (recommended, Vue domain)
- [`useVueValidVElseIf`](https://biomejs.dev/linter/rules/use-vue-valid-v-else-if/) (recommended, Vue domain)
- [`useVueValidVHtml`](https://biomejs.dev/linter/rules/use-vue-valid-v-html/) (recommended, Vue domain)
- [`useVueValidVIf`](https://biomejs.dev/linter/rules/use-vue-valid-v-if/) (recommended, Vue domain)
- [`useVueValidVOn`](https://biomejs.dev/linter/rules/use-vue-valid-v-on/) (recommended, Vue domain)
- [`useVueValidVText`](https://biomejs.dev/linter/rules/use-vue-valid-v-text/) (recommended, Vue domain)
- [`useVueValidTemplateRoot`](https://biomejs.dev/linter/rules/use-vue-valid-template-root/) (recommended, Vue domain)
- [`useVueValidVCloak`](https://biomejs.dev/linter/rules/use-vue-valid-v-cloak/) (recommended, Vue domain)
- [`useVueValidVOnce`](https://biomejs.dev/linter/rules/use-vue-valid-v-once/) (recommended, Vue domain)
- [`useVueValidVPre`](https://biomejs.dev/linter/rules/use-vue-valid-v-pre/) (recommended, Vue domain)
- [`useVueVForKey`](https://biomejs.dev/linter/rules/use-vue-v-for-key/) (recommended, Vue domain)
- [`noDuplicateAttributes`](https://biomejs.dev/linter/rules/no-duplicate-attributes/) (recommended)
- [`noDuplicateArgumentNames`](https://biomejs.dev/linter/rules/no-duplicate-argument-names/) (recommended)
- [`noDuplicateInputFieldNames`](https://biomejs.dev/linter/rules/no-duplicate-input-field-names/) (recommended)
- [`noDuplicateVariableNames`](https://biomejs.dev/linter/rules/no-duplicate-variable-names/) (recommended)
- [`noDuplicateEnumValueNames`](https://biomejs.dev/linter/rules/no-duplicate-enum-value-names/) (recommended)
- [`useLoneAnonymousOperation`](https://biomejs.dev/linter/rules/use-lone-anonymous-operation/) (recommended)

#### Suspicious

Promoted the following rules to the `suspicious` group:

- [`noShadow`](https://biomejs.dev/linter/rules/no-shadow/)
- [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/)
- [`noParametersOnlyUsedInRecursion`](https://biomejs.dev/linter/rules/no-parameters-only-used-in-recursion/)
- [`noUnknownAttribute`](https://biomejs.dev/linter/rules/no-unknown-attribute/)
- [`useArraySortCompare`](https://biomejs.dev/linter/rules/use-array-sort-compare/)
- [`noForIn`](https://biomejs.dev/linter/rules/no-for-in/)
- [`noDuplicatedSpreadProps`](https://biomejs.dev/linter/rules/no-duplicated-spread-props/)
- [`noEqualsToNull`](https://biomejs.dev/linter/rules/no-equals-to-null/)
- [`noProto`](https://biomejs.dev/linter/rules/no-proto/) (recommended)
- [`noUndeclaredEnvVars`](https://biomejs.dev/linter/rules/no-undeclared-env-vars/) (recommended, Turborepo domain)
- [`noReturnAssign`](https://biomejs.dev/linter/rules/no-return-assign/) (default severity: `error`)
- [`noDuplicateEnumValues`](https://biomejs.dev/linter/rules/no-duplicate-enum-values/) (recommended)
- [`noVueArrowFuncInWatch`](https://biomejs.dev/linter/rules/no-vue-arrow-func-in-watch/) (recommended, Vue domain)
- [`noNestedPromises`](https://biomejs.dev/linter/rules/no-nested-promises/)
- [`noLeakedRender`](https://biomejs.dev/linter/rules/no-leaked-render/)
- [`noDeprecatedMediaType`](https://biomejs.dev/linter/rules/no-deprecated-media-type/) (recommended)
- [`noDuplicateGraphqlOperationName`](https://biomejs.dev/linter/rules/no-duplicate-graphql-operation-name/)
- [`useRequiredScripts`](https://biomejs.dev/linter/rules/use-required-scripts/)

#### Style

Promoted the following rules to the `style` group:

- [`useVueMultiWordComponentNames`](https://biomejs.dev/linter/rules/use-vue-multi-word-component-names/) (recommended, Vue domain)
- [`useVueDefineMacrosOrder`](https://biomejs.dev/linter/rules/use-vue-define-macros-order/)
- [`noIncrementDecrement`](https://biomejs.dev/linter/rules/no-increment-decrement/)
- [`noContinue`](https://biomejs.dev/linter/rules/no-continue/)
- [`useSpreadOverApply`](https://biomejs.dev/linter/rules/use-spread-over-apply/)
- [`noTernary`](https://biomejs.dev/linter/rules/no-ternary/)
- [`noMultilineString`](https://biomejs.dev/linter/rules/no-multiline-string/)
- [`noMultiAssign`](https://biomejs.dev/linter/rules/no-multi-assign/)
- [`noExcessiveClassesPerFile`](https://biomejs.dev/linter/rules/no-excessive-classes-per-file/)
- [`noExcessiveLinesPerFile`](https://biomejs.dev/linter/rules/no-excessive-lines-per-file/)
- [`noVueOptionsApi`](https://biomejs.dev/linter/rules/no-vue-options-api/)
- [`useErrorCause`](https://biomejs.dev/linter/rules/use-error-cause/)
- [`useConsistentEnumValueType`](https://biomejs.dev/linter/rules/use-consistent-enum-value-type/)
- [`useConsistentMethodSignatures`](https://biomejs.dev/linter/rules/use-consistent-method-signatures/)
- [`useGlobalThis`](https://biomejs.dev/linter/rules/use-global-this/) (default severity: `warn`)
- [`useDestructuring`](https://biomejs.dev/linter/rules/use-destructuring/)
- [`useVueHyphenatedAttributes`](https://biomejs.dev/linter/rules/use-vue-hyphenated-attributes/) (recommended, Vue domain)
- [`useVueConsistentVBindStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-v-bind-style/) (recommended, Vue domain)
- [`useVueConsistentVOnStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-v-on-style/) (recommended, Vue domain)
- [`noHexColors`](https://biomejs.dev/linter/rules/no-hex-colors/)
- [`useConsistentGraphqlDescriptions`](https://biomejs.dev/linter/rules/use-consistent-graphql-descriptions/)
- [`noRootType`](https://biomejs.dev/linter/rules/no-root-type/)
- [`useLoneExecutableDefinition`](https://biomejs.dev/linter/rules/use-lone-executable-definition/)
- [`useInputName`](https://biomejs.dev/linter/rules/use-input-name/)

#### Complexity

Promoted the following rules to the `complexity` group:

- [`useArrayFind`](https://biomejs.dev/linter/rules/use-array-find/)
- [`noRedundantDefaultExport`](https://biomejs.dev/linter/rules/no-redundant-default-export/) (default severity: `warn`)
- [`noUselessReturn`](https://biomejs.dev/linter/rules/no-useless-return/)
- [`noDivRegex`](https://biomejs.dev/linter/rules/no-div-regex/)

#### Performance

Promoted the following rules to the `performance` group:

- [`noSyncScripts`](https://biomejs.dev/linter/rules/no-sync-scripts/)
- [`noJsxPropsBind`](https://biomejs.dev/linter/rules/no-jsx-props-bind/)
- [`useVueVapor`](https://biomejs.dev/linter/rules/use-vue-vapor/)

#### Security

Promoted the following rules to the `security` group:

- [`noScriptUrl`](https://biomejs.dev/linter/rules/no-script-url/) (recommended)

#### A11y

Promoted the following rules to the `a11y` group:

- [`noAmbiguousAnchorText`](https://biomejs.dev/linter/rules/no-ambiguous-anchor-text/) (recommended)
