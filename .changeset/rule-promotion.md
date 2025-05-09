---
"@biomejs/biome": major
---

Promote nursery rules and update the recommended rule set.

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

- [a11y/noStaticElementInteractions](https://next.biomejs.dev/linter/rules/no-static-element-interactions)
- [a11y/useAriaPropsSupportedByRole](https://next.biomejs.dev/linter/rules/use-aria-props-supported-by-role)(recommended)
- [a11y/useValidAutocomplete](https://next.biomejs.dev/linter/rules/use-valid-autocomplete)
- [complexity/noUselessEscapeInRegex](https://next.biomejs.dev/linter/rules/no-useless-escape-in-regex)
- [complexity/noUselessStringRaw](https://next.biomejs.dev/linter/rules/no-useless-string-raw)
- [performance/noDynamicNamespaceImportAccess](https://next.biomejs.dev/linter/rules/no-dynamic-namespace-import-access)
- [performance/noImgElement](https://next.biomejs.dev/linter/rules/no-img-element)
- [style/noCommonJs](https://next.biomejs.dev/linter/rules/no-common-js)
- [style/noEnum](https://next.biomejs.dev/linter/rules/no-enum)
- [style/noExportedImports](https://next.biomejs.dev/linter/rules/no-exported-imports)
- [style/noHeadElement](https://next.biomejs.dev/linter/rules/no-head-element)
- [style/noNestedTernary](https://next.biomejs.dev/linter/rules/no-nested-ternary)
- [style/noProcessEnv](https://next.biomejs.dev/linter/rules/no-process-env)
- [style/noRestrictedImports](https://next.biomejs.dev/linter/rules/no-restricted-imports)
- [style/noRestrictedTypes](https://next.biomejs.dev/linter/rules/no-restricted-types)
- [style/noSubstr](https://next.biomejs.dev/linter/rules/no-substr)
- [style/useAtIndex](https://next.biomejs.dev/linter/rules/use-at-index)
- [style/useCollapsedIf](https://next.biomejs.dev/linter/rules/use-collapsed-if)
- [style/useComponentExportOnlyModules](https://next.biomejs.dev/linter/rules/use-component-export-only-modules)
- [style/useConsistentCurlyBraces](https://next.biomejs.dev/linter/rules/use-consistent-curly-braces)
- [style/useConsistentMemberAccessibility](https://next.biomejs.dev/linter/rules/use-consistent-member-accessibility)
- [style/useTrimStartEnd](https://next.biomejs.dev/linter/rules/use-trim-start-end)
- [suspicious/noDocumentCookie](https://next.biomejs.dev/linter/rules/no-document-cookie)
- [suspicious/noDocumentImportInPage](https://next.biomejs.dev/linter/rules/no-document-import-in-page)
- [suspicious/noDuplicateElseIf](https://next.biomejs.dev/linter/rules/no-duplicate-else-if)
- [suspicious/noHeadImportInDocument](https://next.biomejs.dev/linter/rules/no-head-import-in-document)
- [suspicious/noIrregularWhitespace](https://next.biomejs.dev/linter/rules/no-irregular-whitespace)
- [suspicious/noOctalEscape](https://next.biomejs.dev/linter/rules/no-octal-escape)
- [suspicious/noTemplateCurlyInString](https://next.biomejs.dev/linter/rules/no-template-curly-in-string)
- [suspicious/useAdjacentOverloadSignatures](https://next.biomejs.dev/linter/rules/use-adjacent-overload-signatures)
- [suspicious/useGoogleFontDisplay](https://next.biomejs.dev/linter/rules/use-google-font-display)
- [suspicious/useGuardForIn](https://next.biomejs.dev/linter/rules/use-guard-for-in)
- [suspicious/useStrictMode](https://next.biomejs.dev/linter/rules/use-strict-mode)

Moreover, the following JavaScript rules are now recommended:

- [complexity/noUselessUndefinedInitialization](https://next.biomejs.dev/linter/rules/no-useless-undefined-initialization)
- [complexity/useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals)
- [correctness/noConstantMathMinMaxClamp](https://next.biomejs.dev/linter/rules/no-constant-math-min-max-clamp)
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
- [style/useConst](https://biomejs.dev/linter/rules/use-const)
- [style/useSelfClosingElements](https://biomejs.dev/linter/rules/use-self-closing-elements)
- [style/useSingleVarDeclarator](https://biomejs.dev/linter/rules/use-single-var-declarator)
