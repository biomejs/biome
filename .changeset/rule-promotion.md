---
"@biomejs/biome": major
---

Change the group of some rules, promote nursery rules and update the recommended rule set.

The following rules have been moved to a new group:

- [complexity/noArguments](https://biomejs.dev/linter/rules/no-arguments)
- [complexity/noCommaOperator](https://biomejs.dev/linter/rules/no-comma-operator)
- [complexity/noFlatMapIdentity](https://biomejs.dev/linter/rules/no-flat-map-identity)
- [complexity/noUselessContinue](https://biomejs.dev/linter/rules/no-useless-continue)
- [complexity/useNumericLiterals](https://biomejs.dev/linter/rules/use-numeric-literals)
- [correctness/useValidTypeof](https://biomejs.dev/linter/rules/use-valid-typeof)
- [performance/noNamespaceImport](https://biomejs.dev/linter/rules/no-namespace-import/)
- [style/useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals)
- [suspicious/noWith](https://biomejs.dev/linter/rules/no-with)

New rules are incubated in the nursery group.
Once stable, we promote them to a stable group.
Use the `biomem igrate` command to automatically migrate nursery rules that have been promoted.

The following CSS rules have been promoted:

- [correctness/noMissingVarFunction](https://biomejs.dev/linter/rules/no-missing-var-function)
- [correctness/noUnknownPseudoClass](https://biomejs.dev/linter/rules/no-unknown-pseudo-class)
- [correctness/noUnknownPseudoElement](https://biomejs.dev/linter/rules/no-unknown-pseudo-element)
- [correctness/noUnknownTypeSelector](https://biomejs.dev/linter/rules/no-unknown-type-selector)
- [style/noDescendingSpecificity](https://biomejs.dev/linter/rules/no-descending-specificity)
- [style/noValueAtRule](https://biomejs.dev/linter/rules/no-value-at-rule)
- [suspcious/noDuplicateCustomProperties](https://biomejs.dev/linter/rules/no-duplicate-custom-properties)
- [suspcious/noDuplicateProperties](https://biomejs.dev/linter/rules/no-duplicate-properties)

The following GraphQL rules have been promoted:

- [style/useDeprecatedReason](https://biomejs.dev/linter/rules/use-deprecated-reason)
- [suspicious/noDuplicatedFields](https://biomejs.dev/linter/rules/no-duplicated-fields)

The following JavaScript rules have been promoted:

- [a11y/noStaticElementInteractions](https://biomejs.dev/linter/rules/no-static-element-interactions)
- [a11y/useAriaPropsSupportedByRole](https://biomejs.dev/linter/rules/use-aria-props-supported-by-role)(recommended)
- [a11y/useValidAutocomplete](https://biomejs.dev/linter/rules/use-valid-autocomplete)
- [complexity/noUselessEscapeInRegex](https://biomejs.dev/linter/rules/no-useless-escape-in-regex)
- [complexity/noUselessStringRaw](https://biomejs.dev/linter/rules/no-useless-string-raw)
- [performance/noDynamicNamespaceImportAccess](https://biomejs.dev/linter/rules/no-dynamic-namespace-import-access)
- [performance/noImgElement](https://biomejs.dev/linter/rules/no-img-element)
- [style/noCommonJs](https://biomejs.dev/linter/rules/no-common-js)
- [style/noEnum](https://biomejs.dev/linter/rules/no-enum)
- [style/noExportedImports](https://biomejs.dev/linter/rules/no-exported-imports)
- [style/noHeadElement](https://biomejs.dev/linter/rules/no-head-element)
- [style/noNestedTernary](https://biomejs.dev/linter/rules/no-nested-ternary)
- [style/noProcessEnv](https://biomejs.dev/linter/rules/no-process-env)
- [style/noRestrictedImports](https://biomejs.dev/linter/rules/no-restricted-imports)
- [style/noRestrictedTypes](https://biomejs.dev/linter/rules/no-restricted-types)
- [style/noSubstr](https://biomejs.dev/linter/rules/no-substr)
- [style/useAtIndex](https://biomejs.dev/linter/rules/use-at-index)
- [style/useCollapsedIf](https://biomejs.dev/linter/rules/use-collapsed-if)
- [style/useComponentExportOnlyModules](https://biomejs.dev/linter/rules/use-component-export-only-modules)
- [style/useConsistentCurlyBraces](https://biomejs.dev/linter/rules/use-consistent-curly-braces)
- [style/useConsistentMemberAccessibility](https://biomejs.dev/linter/rules/use-consistent-member-accessibility)
- [style/useTrimStartEnd](https://biomejs.dev/linter/rules/use-trim-start-end)
- [suspicious/noDocumentCookie](https://biomejs.dev/linter/rules/no-document-cookie)
- [suspicious/noDocumentImportInPage](https://biomejs.dev/linter/rules/no-document-import-in-page)
- [suspicious/noDuplicateElseIf](https://biomejs.dev/linter/rules/no-duplicate-else-if)
- [suspicious/noHeadImportInDocument](https://biomejs.dev/linter/rules/no-head-import-in-document)
- [suspicious/noIrregularWhitespace](https://biomejs.dev/linter/rules/no-irregular-whitespace)
- [suspicious/noOctalEscape](https://biomejs.dev/linter/rules/no-octal-escape)
- [suspicious/noTemplateCurlyInString](https://biomejs.dev/linter/rules/no-template-curly-in-string)
- [suspicious/useAdjacentOverloadSignatures](https://biomejs.dev/linter/rules/use-adjacent-overload-signatures)
- [suspicious/useGoogleFontDisplay](https://biomejs.dev/linter/rules/use-google-font-display)
- [suspicious/useGuardForIn](https://biomejs.dev/linter/rules/use-guard-for-in)
- [suspicious/useStrictMode](https://biomejs.dev/linter/rules/use-strict-mode)

Moreover, the following JavaScript rules are now recommended:

- [complexity/noUselessUndefinedInitialization](https://biomejs.dev/linter/rules/no-useless-undefined-initialization)
- [complexity/useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals)
- [correctness/noConstantMathMinMaxClamp](https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp)
- [correctness/noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters) (recommended by ESLint)
- [correctness/noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports)
- [correctness/noUnusedPrivateClassMembers](https://biomejs.dev/linter/rules/no-unused-private-class-members) (recommended by ESLint)
- [correctness/noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) (recommended by ESLint)
- [complexity/useDateNow](https://biomejs.dev/linter/rules/use-date-now)

And the following style rules are no longer recommended:

- [style/useNumberNamespace](https://biomejs.dev/linter/rules/use-number-namespace)
- [style/useAsConstAssertion](https://biomejs.dev/linter/rules/use-as-const-assertion)
- [style/noParameterAssign](https://biomejs.dev/linter/rules/no-parameter-assign)
- [style/noInferrableTypes](https://biomejs.dev/linter/rules/no-inferrable-types)
- [style/useDefaultParameterLast](https://biomejs.dev/linter/rules/use-default-parameter-last)
- [style/noUnusedTemplateLiteral](https://biomejs.dev/linter/rules/no-unused-template-literal)
- [style/useEnumInitializers](https://biomejs.dev/linter/rules/use-enum-initializers)
- [style/noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
- [style/useSelfClosingElements](https://biomejs.dev/linter/rules/use-self-closing-elements)
- [style/useSingleVarDeclarator](https://biomejs.dev/linter/rules/use-single-var-declarator)
