# Rome changelog

Biome is a fork of [Rome Tools](https://github.com/rome/tools).
This CHANGELOG describes all released of Rome Tools.

All releases of Biome are described in its own [CHANGELOG](./CHANGELOG.md).

## 12.1.3

### CLI

#### Other changes

- `rome lsp-proxy` should accept the global CLI options [#4505](https://github.com/rome/tools/issues/4505)
- Enhance help descriptions
- Accept the environment variable 'ROME_BINARY' to override the Rome binary

### Configuration

#### Other changes

- Fix an issue where all the `nursery` were enabled when the `"nursery": {}` object
was defined [#4479](https://github.com/rome/tools/issues/4479)

### Formatter

### Linter

#### Other changes

- Fix false positive diagnostics ([#4483](https://github.com/rome/tools/issues/4483)) that [`useHookAtTopLevel`](https://docs.rome.tools/lint/rules/usehookattoplevel/) caused to returning call expressions of a hook.
- Revert [#4359](https://github.com/rome/tools/issues/4359)

### Parser

#### Other changes

- Revert [#4359](https://github.com/rome/tools/issues/4359)


## 12.1.2

### Configuration

#### Other changes

- Fix regression where a specific combination of configuration didn't load
the correct rules to apply [#4502](https://github.com/rome/tools/issues/4502)

### Linter

#### New rules
- [`noUselessTypeConstraint`](https://docs.rome.tools/lint/rules/noUselessTypeConstraint/)

#### Other changes

- [`noInnerDeclarations`](https://docs.rome.tools/lint/rules/noinnerdeclarations/): allow function declarations in nested block inside an _ES module_ [#4492](https://github.com/rome/tools/issues/4492).
- [`noInvalidConstructorSuper`](https://docs.rome.tools/lint/rules/noinvalidconstructorsuper/): recognize `extends` clauses that use static member access such as `extends mod.C` [#4499](https://github.com/rome/tools/issues/4499)

## 12.1.1

### CLI

#### Other changes

- Fix regression where the command `lsp-proxy` was renamed `lsp_proxy` [#4489](https://github.com/rome/tools/issues/4489)


### Configuration

#### Other changes

- Fix an issue where Rome was loading incorrectly recommended rule [#4479](https://github.com/rome/tools/issues/4479) [#4488](https://github.com/rome/tools/issues/4488)

### Linter

#### Other changes

- Fix an issue where the [`noAssignInExpressions`](https://docs.rome.tools/lint/rules/noassigninexpressions/) rule replaced the operator with an invalid token, which caused other lint rules to crash. [#4464](https://github.com/rome/tools/issues/4464)
- Fix an issue that [`noUnusedVariables`](https://docs.rome.tools/lint/rules/nounusedvariables/) rule did not correctly detect exports when a variable and an `interface` had the same name [#4468](https://github.com/rome/tools/pull/4468)

## 12.1.0

### CLI

#### Other changes

- Refactored the underling argument parsing logic. Changed the look and feel of the help
output. [#4405](https://github.com/rome/tools/pull/4405).
- The command `rome check` can accept input from `stdin`.
- Add the argument `--stdin-file-path` to use when running `rome check` via `stdin`.
- Add the argument `--formatter-enabled` to the command `rome check` to control the formatter via CLI.
- Add the argument `--linter-enabled` to the command `rome check` to control the linter via CLI.
- Add the argument `--organize-imports-enabled` to the command `rome check` to control the import sorting via CLI.
- Add new command `rome migrate` the transform the configuration file `rome.json`
	when there are breaking changes.

### Configuration

- Add `vcs` property, to opt in the VCS integration:
  - `vcs.enabled`, to enable or not the integration;
  - `vcs.clientKind`, the supported clients;
  - `vcs.useIgnoreFile`, to ignore the files/paths inside the file;
  - `vcs.root`, an optional path to the root of the VCS;

### Editors

#### Other changes

- Fix an issue where the VSCode extension duplicates text when using VSCode git utilities [#4338](https://github.com/rome/tools/issues/4338)
- Remove code assists from being added to the code actions when apply fixes;
- When requesting code actions, ignored files should not throw errors. Fixes [#4434](https://github.com/rome/tools/issues/4434)

### Formatter

#### Other changes

- Fix an issue where formatting of JSX string literals property values were using incorrect quotes [#4054](https://github.com/rome/tools/issues/4054)
- Changed import assertion grammar to the new import attribute assertion
```diff
- import "module" assert {}
+ import "module" with {}
```
- Fix an issue where JSON formatter does not respect `lineWidth` for arrays [#4351](https://github.com/rome/tools/issues/4351)
- Add support for decorators

### Linter

#### New rules

- [`noConfusingArrow`](https://docs.rome.tools/lint/rules/noConfusingArrow/)
- [`noRedundantRoles`](https://docs.rome.tools/lint/rules/noRedundantRoles/)
- [`noNoninteractiveTabindex`](https://docs.rome.tools/lint/rules/noNoninteractiveTabindex/)
- [`noAriaUnsupportedElements`](https://docs.rome.tools/lint/rules/noAriaUnsupportedElements/)
- [`noConsoleLog`](https://docs.rome.tools/lint/rules/noConsoleLog/)
- [`noForEach`](https://docs.rome.tools/lint/rules/noForEach/)
- [`useLiteralKeys`](https://docs.rome.tools/lint/rules/useLiteralKeys/)
- [`noConstantCondition`](https://docs.rome.tools/lint/rules/noConstantCondition/)
- [`useGroupedTypeImport`](https://docs.rome.tools/lint/rules/useGroupedTypeImport/)
- [`noUselessConstructor`](https://docs.rome.tools/lint/rules/noUselessConstructor/)
- [`useLiteralEnumMembers`](https://docs.rome.tools/lint/rules/useLiteralEnumMembers/)
- [`useHeadingContent`](https://docs.rome.tools/lint/rules/useHeadingContent/)
- [`noAccumulatingSpread`](https://docs.rome.tools/lint/rules/noAccumulatingSpread/)
- [`useSimpleNumberKeys`](https://docs.rome.tools/lint/rules/useSimpleNumberKeys/)


#### Promoted rules

New rules are promoted, please check [#4431](https://github.com/rome/tools/pull/4431) for more details.

- [lint/a11y/noNoninteractiveElementToInteractiveRole](https://docs.rome.tools/lint/rules/noNoninteractiveElementToInteractiveRole)
- [lint/a11y/noRedundantAlt](https://docs.rome.tools/lint/rules/noRedundantAlt)
- [lint/a11y/noSvgWithoutTitle](https://docs.rome.tools/lint/rules/noSvgWithoutTitle)
- [lint/a11y/useAriaPropsForRole](https://docs.rome.tools/lint/rules/useAriaPropsForRole)
- [lint/a11y/useIframeTitle](https://docs.rome.tools/lint/rules/useIframeTitle)
- [lint/a11y/useMediaCaption](https://docs.rome.tools/lint/rules/useMediaCaption)
- [lint/a11y/useValidAriaProps](https://docs.rome.tools/lint/rules/useValidAriaProps)
- [lint/a11y/useValidLang](https://docs.rome.tools/lint/rules/useValidLang)
- [lint/complexity/noExtraSemicolon](https://docs.rome.tools/lint/rules/noExtraSemicolon)
- [lint/complexity/noUselessCatch](https://docs.rome.tools/lint/rules/noUselessCatch)
- [lint/complexity/noUselessConstructor](https://docs.rome.tools/lint/rules/noUselessConstructor)
- [lint/complexity/noUselessLabel](https://docs.rome.tools/lint/rules/noUselessLabel)
- [lint/complexity/noUselessRename](https://docs.rome.tools/lint/rules/noUselessRename)
- [lint/complexity/noUselessSwitchCase](https://docs.rome.tools/lint/rules/noUselessSwitchCase)
- [lint/complexity/noWith](https://docs.rome.tools/lint/rules/noWith)
- [lint/correctness/noGlobalObjectCalls](https://docs.rome.tools/lint/rules/noGlobalObjectCalls)
- [lint/correctness/noInnerDeclarations](https://docs.rome.tools/lint/rules/noInnerDeclarations)
- [lint/correctness/noInvalidConstructorSuper](https://docs.rome.tools/lint/rules/noInvalidConstructorSuper)
- [lint/correctness/noSwitchDeclarations](https://docs.rome.tools/lint/rules/noSwitchDeclarations)
- [lint/correctness/noUnreachableSuper](https://rome.tools/docs/lint/rules/noUnreachableSuper)
- [lint/correctness/noUnsafeOptionalChaining](https://docs.rome.tools/lint/rules/noUnsafeOptionalChaining)
- [lint/correctness/noUnusedLabels](https://docs.rome.tools/lint/rules/noUnusedLabels)
- [lint/correctness/useYield](https://docs.rome.tools/lint/rules/useYield)
- [lint/style/noCommaOperator](https://docs.rome.tools/lint/rules/noCommaOperator)
- [lint/style/noInferrableTypes](https://docs.rome.tools/lint/rules/noInferrableTypes)
- [lint/style/noNamespace](https://docs.rome.tools/lint/rules/noNamespace)
- [lint/style/noParameterAssign](https://docs.rome.tools/lint/rules/noParameterAssign)
- [lint/style/noParameterProperties](https://docs.rome.tools/lint/rules/noParameterProperties)
- [lint/style/noRestrictedGlobals](https://docs.rome.tools/lint/rules/noRestrictedGlobals)
- [lint/suspicious/noAssignInExpressions](https://docs.rome.tools/lint/rules/noAssignInExpressions)
- [lint/suspicious/noClassAssign](https://docs.rome.tools/lint/rules/noClassAssign)
- [lint/suspicious/noConfusingLabels](https://docs.rome.tools/lint/rules/noConfusingLabels)
- [lint/suspicious/noDuplicateCase](https://docs.rome.tools/lint/rules/noDuplicateCase)
- [lint/suspicious/noDuplicateClassMembers](https://docs.rome.tools/lint/rules/noDuplicateClassMembers)
- [lint/suspicious/noPrototypeBuiltins](https://docs.rome.tools/lint/rules/noPrototypeBuiltins)
- [lint/suspicious/noRedeclare](https://docs.rome.tools/lint/rules/noRedeclare)
- [lint/suspicious/noSelfCompare](https://docs.rome.tools/lint/rules/noSelfCompare)
- [lint/suspicious/useNamespaceKeyword](https://docs.rome.tools/lint/rules/useNamespaceKeyword)

Note that, `noExtraSemicolons` and `noExtraLabels` are renamed to [`noExtraSemicolon`](https://docs.rome.tools/lint/rules/noextrasemicolon/) and [`noUselessLabel`](https://docs.rome.tools/lint/rules/nouselesslabel/).

#### Other changes

- Code actions are formatted using Rome's formatter. If the formatter is disabled,
	the code action is not formatted.
- Fixed an issue that [`useShorthandArrayType`](https://docs.rome.tools/lint/rules/useShorthandArrayType) rule did not handle nested ReadonlyArray types correctly and erroneously reported TsObjectType [#4354](https://github.com/rome/tools/issues/4353).
- [`noUndeclaredVariables`](https://docs.rome.tools/lint/rules/noUndeclaredVariables) detects globals based on the file type.
- Fix an issue when [`noUndeclaredVariables`](https://docs.rome.tools/lint/rules/noundeclaredvariables/) incorrectly identifies `AggregateError` as an undeclared variable. [#4365](https://github.com/rome/tools/issues/4365)
- Fix an issue that `useLiteralKeys` rule doesn't ignore valid uses of square bracket notation. [#4370](https://github.com/rome/tools/issues/4370)
- Fix [#4348](https://github.com/rome/tools/issues/4348) that caused [`noNonNullAssertion`](https://docs.rome.tools/lint/rules/nononnullassertion/) to emit incorrect code action
- Fix [#4410](https://github.com/rome/tools/issues/4410) that caused [`useButtonType`](https://docs.rome.tools/lint/rules/usebuttontype/) to miss some cases
- Fix false positive diagnostics that [`useCamelCase`](https://docs.rome.tools/lint/rules/usecamelcase/) caused to default exported components
- Fix false positive diagnostics that [`useCamelCase`](https://docs.rome.tools/lint/rules/usecamelcase/) caused to private class members
- Fix false positive diagnostics that [`useHookAtTopLevel`](https://docs.rome.tools/lint/rules/usehookattoplevel/) caused to arrow functions, export default functions and function expressions.
- Fix false positive diagnostics that [`useHookAtTopLevel`](https://docs.rome.tools/lint/rules/usehookattoplevel/) caused to `as` or `satisfies` expression.
- Fix false positive diagnostics that [`noHeaderScope`](https://docs.rome.tools/lint/rules/noheaderscope/) caused to custom components
- Fix false negative diagnostics that [`noNoninteractiveElementToInteractiveRole`](https://docs.rome.tools/lint/rules/nononinteractiveelementtointeractiverole/) and [`noNoninteractiveTabindex`](https://docs.rome.tools/lint/rules/nononinteractivetabindex/) caused to non-interactive elements.


### Parser

#### Other changes

- Allow module syntax in `cts` files
- Changed import assertion grammar to the new import attribute assertion
```diff
- import "module" assert {}
+ import "module" with {}
```
- Allow decorators before `export` and `export default`. [#4252](https://github.com/rome/tools/issues/4252)
- Add support for Stage 3 decorators

### VSCode

- `requireConfiguration` is set to `true` by default

## 12.0.0

### CLI

##### Breaking changes

- Review how the traversal of the file system works. Now Rome won't navigate folders that are ignored.
	While this change is a bug fix, this could affect how the `ignore` entries are defined inside a project. We suggest to review them
	and make sure they still work.
- `--apply-suggested` is now called `--apply-unsafe`
- `rome check --apply` and `rome check --apply-unsafe` exits with non-zero code (error code)
if there are still diagnostics to be addressed.

##### Other changes

- `rome check` now checks import statements. This is an experimental feature that needs to be
	enabled via configuration. Import can be sorted using `rome check --apply-unsafe`
- Rome is able to auto discover the configuration file. If Rome doesn't fine a configuration in the
working directory, it will try to find one in the parent directories.
- Add a new global options called `--config-path`. It tells Rome to try and discover a `rome.json` file
in the given path.
	```shell
	rome format --config-path=../../other/path/
	rome check --config-path=../../other/path/
	```

### Configuration

#### Other changes

- Rome now uses the internal JSON parser to validate the configuration file. This means Rome won't
	exit anymore if there are issues with the `rome.json` file, instead it will apply its defaults
	to the sections that are incorrect.
- Add `javascript.organizeImports`. This is an experimental feature and users need to opt-in.

```json
{
  "organizeImports": {
    "enabled": true,
    "ignore": ["trickyFile.js"]
  }
}
```
- Add `linter.rules.all` and `linter.rules.[group].all`. These options allow to enable or disable **all**
rules, or all rules for a **given group**. `all` and `recommended` can't be both `true`.


```json
{
  "linter": {
    "rules": {
      "all": true,
      "style" : {
        "all": false
      }
    }
  }
}
```

The previous example will enable all rules and disable all rules that belong to the `style` group.

### Editors

##### Other changes

- Add support to display diagnostics for JSON files.
- Add support to format JSON files.
- Pull diagnostics when parsing a `rome.json` file.
- Imports sorting is not applied for files that are not supported or ignored.

### Formatter

- Add support for JSON files
- Add support for TypeScript 4.7
- Add support for TypeScript 5.0

### Linter

New rules are promoted, please check [#4239](https://github.com/rome/tools/pull/4239) for more
details.
- [lint/correctness/noUnsafeFinally](https://docs.rome.tools/lint/rules/noUnsafeFinally)
- [lint/correctness/noConstructorReturn](https://docs.rome.tools/lint/rules/noConstructorReturn)
- [lint/correctness/noPrecisionLoss](https://docs.rome.tools/lint/rules/noPrecisionLoss)
- [lint/correctness/noVoidTypeReturn](https://docs.rome.tools/lint/rules/noVoidTypeReturn)
- [lint/correctness/noStringCaseMismatch](https://docs.rome.tools/lint/rules/noStringCaseMismatch)
- [lint/correctness/noSetterReturn](https://docs.rome.tools/lint/rules/noSetterReturn)
- [lint/a11y/useHtmlLang](https://docs.rome.tools/lint/rules/useHtmlLang)
- [lint/a11y/noDistractingElements](https://docs.rome.tools/lint/rules/noDistractingElements)
- [lint/a11y/noHeaderScope](https://docs.rome.tools/lint/rules/noHeaderScope)
- [lint/a11y/noAccessKey](https://docs.rome.tools/lint/rules/noAccessKey)
- [lint/style/useExponentiationOperator](https://docs.rome.tools/lint/rules/useExponentiationOperator)
- [lint/style/useNumericLiterals](https://docs.rome.tools/lint/rules/useNumericLiterals)
- [lint/style/useDefaultParameterLast](https://docs.rome.tools/lint/rules/useDefaultParameterLast)
- [lint/style/useConst](https://docs.rome.tools/lint/rules/useConst)
- [lint/style/noVar](https://docs.rome.tools/lint/rules/noVar)
- [lint/style/noNonNullAssertion](https://docs.rome.tools/lint/rules/noNonNullAssertion)
- [lint/style/useEnumInitializers](https://docs.rome.tools/lint/rules/useEnumInitializers)
- [lint/suspicious/noEmptyInterface](https://docs.rome.tools/lint/rules/noEmptyInterface)
- [lint/suspicious/noExtraNonNullAssertion](https://docs.rome.tools/lint/rules/noExtraNonNullAssertion)
- [lint/suspicious/noRedundantUseStrict](https://docs.rome.tools/lint/rules/noRedundantUseStrict)
- [lint/suspicious/noConstEnum](https://docs.rome.tools/lint/rules/noConstEnum)
- [lint/suspicious/useDefaultSwitchClauseLast](https://docs.rome.tools/lint/rules/useDefaultSwitchClauseLast)
- [lint/suspicious/noDuplicateObjectKeys](https://docs.rome.tools/lint/rules/noDuplicateObjectKeys)


### Parser

- Support for TypeScript 4.7
- Support for TypeScript 5.0

### VSCode

##### Other changes
- Add a new option called `requireConfiguration`. Enabling this option will force Rome to require
a configuration file in your workspace/project. If Rome doesn't find a `rome.json` file, it won't
emit diagnostics.

## 11.0.0

### CLI

#### BREAKING CHANGES

- the argument `--no-colors` has been removed, in favor of `--color=off`

#### Other changes

- The `init` command now adds the `$schema` property to the generated `rome.json` file
  if `rome` is installed inside the `node_modules` folder. Follow [this guide](https://docs.rome.tools/configuration#schema) to add the `$schema` property
  manually in a project with an existing `rome.json` file.
- A new `--semicolons` option that configures if the formatter prints semicolons at the end of every statement (default) or at the beginning of statements when necessary to prevent ASI failures.
- Rome exits with an error code if it doesn't process any file.
- Fixed how the maximum number of diagnostics is calculated [#3869](https://github.com/rome/tools/pull/3869).
  Rome now prints the total number of errors caused in the files.
- Rome now traverses symbolic links and emits warnings if it detects loops, and continues processing the next file during the directory traversal.
- You can force color output using the new global `--colors` option with the value `force`. Forcing color output can be useful if you spawn Rome as a subprocess.
  Rome is spawned as a process;

### Configuration

- Added the JSON schema `$schema` property. The schema enables auto-completion by editors and...
  auto-completion and descriptions of all fields of the configuration file.
- Added a new `files.ignore` option where users can ignore files across tools.

### Editors

- We also publish Rome to [Open VSX](https://open-vsx.org/).
- The extension now resolves the Rome version installed in the `node_modules` folder.
- Fixed an issue where diagnostics were not updated after a change to the configuration file (#3724)[https://github.com/rome/tools/pull/3724]
- The LSP emits a new action where the user can suppress a rule.
- The extension now allows [sort imports](https://github.com/rome/tools/blob/main/editors/vscode/README.md#imports-sorting-experimental)

### Formatter

#### BREAKING CHANGES

- Fixed incompatibility issues with Prettier [#3531](https://github.com/rome/tools/issues/3531)
  - [#3686](https://github.com/rome/tools/pull/3686)
  - [#3732](https://github.com/rome/tools/pull/3732)
  - [#3842](https://github.com/rome/tools/pull/3842)
- Fixed an issue where infinite parentheses were wrongly inserted [#3735](https://github.com/rome/tools/issues/3735)
- Better formatting for `jestEach` templates

#### Other changes

- Added [support](https://docs.rome.tools/configuration/#javascriptformattersemicolon) for omitting semicolons.


### Linter

- Fixed false positives emitted by [`noUselessFragments`](https://docs.rome.tools/lint/rules/nouselessfragments/) [#3668](https://github.com/rome/tools/issues/3668)
- Fixed [`noArrayIndexKey`](https://docs.rome.tools/lint/rules/noarrayindexkey/) where some cases were not detected [#3670](https://github.com/rome/tools/issues/3670)
- Fixed false positives emitted by [`noConstAssign`](https://docs.rome.tools/lint/rules/noconstassign/) [#3728](https://github.com/rome/tools/issues/3728)
- Fixed false positives emitted by [`noShoutyConstants`](https://docs.rome.tools/lint/rules/noshoutyconstants/) [#3867](https://github.com/rome/tools/issues/3867)
- Fixed false positives emitted by [`noUnusedVariables`](https://docs.rome.tools/lint/rules/nounusedvariables/) [#3779](https://github.com/rome/tools/issues/3779)
- Fixed [`noUndeclaredVariables`](https://docs.rome.tools/lint/rules/noundeclaredvariables/) where some cases were not detected [#3798](https://github.com/rome/tools/issues/3798)
- Fixed [`noUndeclaredVariables`](https://docs.rome.tools/lint/rules/noundeclaredvariables/) where types were incorrectly detected [#3669](https://github.com/rome/tools/issues/3669)

#### Rules

The following rules have been stabilized:
- `nursery/useFlatMap` -> `complexity/useFlatMap`
- `nursery/useValidForDirection` -> `correctness/useValidForDirection`
- `nursery/noExplicitAny` -> `suspicious/noExplicitAny`
- `nursery/noConstAssign` -> `correctness/noConstAssign`

These rules are all recommended, so they will be enabled by default. You can simply remove those entries from your configuration file if you had enabled them manually from the `nursery` group.

The following rules have been renamed:
- `a11y/useBlankTarget` -> `a11y/noBlankTarget`
- `correctness/noMultipleSpacesInRegularExpressionLiterals` -> `complexity/noMultipleSpacesInRegularExpressionLiterals`
- `style/useOptionalChain` -> `complexity/useOptionalChain`
- `correctness/noUselessFragments` -> `complexity/noUselessFragments`
- `correctness/noDelete` -> `performance/noDelete`
- `correctness/useSingleCaseStatement` -> `style/useSingleCaseStatement`
- `correctness/useWhile` -> `style/useWhile`
- `correctness/noArguments` -> `style/noArguments`
- `correctness/noAsyncPromiseExecutor` -> `suspicious/noAsyncPromiseExecutor`
- `correctness/noCommentText` -> `suspicious/noCommentText`
- `correctness/noCompareNegZero` -> `suspicious/noCompareNegZero`
- `correctness/noDebugger` -> `suspicious/noDebugger`
- `correctness/noDoubleEquals` -> `suspicious/noDoubleEquals`
- `correctness/noShadowRestrictedNames` -> `suspicious/noShadowRestrictedNames`
- `correctness/noSparseArray` -> `suspicious/noSparseArray`
- `correctness/noUnsafeNegation` -> `suspicious/noUnsafeNegation`
- `correctness/useValidTypeof` -> `suspicious/useValidTypeof`
- `correctness/noArrayIndexKey` -> `suspicious/noArrayIndexKey`
- `correctness/noCatchAssign` -> `suspicious/noCatchAssign`
- `correctness/noDupeArgs` -> `suspicious/noDuplicateParameters`
- `correctness/noFunctionAssign` -> `suspicious/noFunctionAssign`
- `correctness/noImportAssign` -> `suspicious/noImportAssign`
- `correctness/noLabelVar` -> `suspicious/noLabelVar`
- `correctness/noRestrictedGlobals` -> `nursery/noRestrictedGlobals`
- `nursery/noDupeKeys` -> `nursery/noDuplicateObjectKeys`

If you were not changing the severity level of any of these rules in your configuration file, or suppressing a diagnostic emitted by those rules using suppression comments, you do not have to do anything. But if you did, Rome will now emit diagnostics for the parts of your configuration or suppression comments you need to update.

The following rules are no longer recommended:
- `style/noImplicitBoolean`
- `style/noNegationElse`
- `style/useBlockStatements`
- `style/useShorthandArrayType`
- `correctness/useSingleCaseStatement` / `style/useSingleCaseStatement`
- `style/noShoutyConstants`

The styling decisions imposed by these rules were not deemed to be idiomatic enough in the JavaScript ecosystem to be enabled by default. If you do want to enforce those rules in your project, you will have to enable them manually in you configuration file:

```json
{
  "linter": {
    "rules": {
        "style": {
            "useBlockStatements": "warn"
        }
    }
  }
}
```

Finally, the following new rules have been introduced to the nursery group in this release:
- [`nursery/noAccessKey`](https://docs.rome.tools/lint/rules/noAccessKey)
- [`nursery/noConditionalAssignment`](https://docs.rome.tools/lint/rules/noConditionalAssignment)
- [`nursery/noConstEnum`](https://docs.rome.tools/lint/rules/noConstEnum)
- [`nursery/noConstructorReturn`](https://docs.rome.tools/lint/rules/noConstructorReturn)
- [`nursery/noDistractingElements`](https://docs.rome.tools/lint/rules/noDistractingElements)
- [`nursery/noDuplicateObjectKeys`](https://docs.rome.tools/lint/rules/noDuplicateObjectKeys)
- [`nursery/noEmptyInterface`](https://docs.rome.tools/lint/rules/noEmptyInterface)
- [`nursery/noExtraNonNullAssertion`](https://docs.rome.tools/lint/rules/noExtraNonNullAssertion)
- [`nursery/noHeaderScope`](https://docs.rome.tools/lint/rules/noHeaderScope)
- [`nursery/noNonNullAssertion`](https://docs.rome.tools/lint/rules/noNonNullAssertion)
- [`nursery/noPrecisionLoss`](https://docs.rome.tools/lint/rules/noPrecisionLoss)
- [`nursery/noRedundantUseStrict`](https://docs.rome.tools/lint/rules/noRedundantUseStrict)
- [`nursery/noSetterReturn`](https://docs.rome.tools/lint/rules/noSetterReturn)
- [`nursery/noStringCaseMismatch`](https://docs.rome.tools/lint/rules/noStringCaseMismatch)
- [`nursery/noUnsafeFinally`](https://docs.rome.tools/lint/rules/noUnsafeFinally)
- [`nursery/noVoidTypeReturn`](https://docs.rome.tools/lint/rules/noVoidTypeReturn)
- [`nursery/useDefaultSwitchClauseLast`](https://docs.rome.tools/lint/rules/useDefaultSwitchClauseLast)
- [`nursery/useNumericLiterals`](https://docs.rome.tools/lint/rules/useNumericLiterals)
- [`nursery/useAriaPropTypes`](https://docs.rome.tools/lint/rules/useAriaPropTypes)
- [`nursery/useAriaPropsForRole`](https://docs.rome.tools/lint/rules/useAriaPropsForRole)
- [`nursery/noVar`](https://docs.rome.tools/lint/rules/noVar)
- [`nursery/useConst`](https://docs.rome.tools/lint/rules/useConst)

Please give them a try by manually enabling them in your configuration and please share your feedback on the rule, diagnostics, and code fixes.

### Parser

- Added support for `JSON`;
- Added support `satisfies` keyword;
- Fixed parse for `async` used as label [#3612](https://github.com/rome/tools/issues/3612)
- Fixed parse of `export default function` in `d.ts` files [#3485](https://github.com/rome/tools/issues/3485)
- Improved the parsing of `await` in non-async contexts [#2479](https://github.com/rome/tools/issues/2479)

### VSCode

- Removed the "preview" label from the extension.
- Improved logging when the extension can't connect to the server. [#3920](https://github.com/rome/tools/issues/3920)

### JavaScript APIs

#### Breaking change

- The concept of `backend` has been removed, in favor of the concept of `distribution`.
- Removed the possibility to connect to the daemon, for the time being.
- The APIs are asynchronous anymore.

#### Other changes

- The package has been marked as unstable and in alpha state.

## 10.0.1

### CLI

- Respect the formatter / linter `enabled` flag from configuration ([#3591](https://github.com/rome/tools/issues/3591))
- Correctly account for diff diagnostics in the printed diagnostics count ([#3595](https://github.com/rome/tools/issues/3595))

### Formatter

- Do not insert a trailing comma in import expressions ([#3600](https://github.com/rome/tools/issues/3600))

### Linter

- Fixed false positives in [`noUselessFragments`](https://docs.rome.tools/lint/rules/nouselessfragments/), [`noArrayIndexKey`](https://docs.rome.tools/lint/rules/noarrayindexkey/), [`noChildrenProp`](https://docs.rome.tools/lint/rules/nochildrenprop/), [`noUselessFragments`](https://docs.rome.tools/lint/rules/nouselessfragments/), [`noVoidElementsWithChildren`](https://docs.rome.tools/lint/rules/novoidelementswithchildren/), [`noDangerouslySetInnerHtml`](https://docs.rome.tools/lint/rules/nodangerouslysetinnerhtml/), [`noDangerouslySetInnerHtmlWithChildren`](https://docs.rome.tools/lint/rules/nodangerouslysetinnerhtmlwithchildren/), `useValidAnchor`, [`noRenderReturnValue`](https://docs.rome.tools/lint/rules/norenderreturnvalue/), [`noUnusedVariables`](https://docs.rome.tools/lint/rules/nounusedvariables/) and [`useKeyWithClickEvents`](https://docs.rome.tools/lint/rules/usekeywithclickevents/)
([#3592](https://github.com/rome/tools/pull/3592), [#3619](https://github.com/rome/tools/pull/3619), [#3599](https://github.com/rome/tools/pull/3599), [#3626](https://github.com/rome/tools/pull/3626), [#3620](https://github.com/rome/tools/pull/3620) & [#3644](https://github.com/rome/tools/pull/3644))

### Editors

- Display the version of the language server in the status bar ([#3616](https://github.com/rome/tools/issues/3616))

## 10.0.0

### CLI

- Added the new command `rome version`.
- Added the new command `rome rage`.
- Added the new command `rome lsp-proxy`.
- Added the new option`--version` as an alias for `rome version`
- Added a new argument `--files-max-size` to change the allowed size of files, in bytes.
- Added a new argument `--formatter-enabled` to the command `rome ci`.
- Added a new argument `--linter-enabled` to the command `rome ci`.
- Added the new `format` option `--trailing-comma` to configure where to add trailing commas.
- Correctly show the supported options for `rome ci`, closes [#3456](https://github.com/rome/tools/issues/3456).
- Fixed the command `rome ci` command to run the linter even if the formatter is disabled, closes [#3495](https://github.com/rome/tools/issues/3495).
- Fixed the messaging of some diagnostics, [#3460](https://github.com/rome/tools/pull/3460).

### Configuration

- Added `files.maxSize`, to change the allowed size of files, in bytes.

### Diagnostics

- Fix false positive for unknown lint rule in suppression comments during formatting [#3406](https://github.com/rome/tools/issues/3406).
- Correctly handle empty lines when printing code diffs [#3375](https://github.com/rome/tools/issues/3375).


### Formatter

- Added the new trailing comma option that configures where to add trailing commas. Supports the values: `all`, `es5` and `none`; refer to the [documentation](https://rome.tools/docs/#javascriptformattertrailingcomma) to learn more.
- Improved JSX formatting [#3499](https://github.com/rome/tools/issues/3499), [#3211](https://github.com/rome/tools/issues/3211), [#3377](https://github.com/rome/tools/issues/3377)
- Better formatting of object destructing
- Improved formatting of test calls
- Fixed formatting of trailing comments in arrow functions

### Linter

- **BREAKING CHANGE**: some rules have been moved to new groups to better reflect their purpose. This may result in Rome failing to load your configuration or suppression comments that now refer to unknown rules. Please check out [#3471](https://github.com/rome/tools/pull/3471) to learn more about the affected rules.
- Fixed issues in the [`noUnreachable`](https://docs.rome.tools/lint/rules/nounreachable/) rule
- Fixed false positive cases for [`noNegationElse`](https://docs.rome.tools/lint/rules/nonegationelse/) [#3141](https://github.com/rome/tools/issues/3141)
- Fixed false positive cases for [`noUnusedVariables`](https://docs.rome.tools/lint/rules/nounusedvariables/) [#3169](https://github.com/rome/tools/issues/3169)
- Fixed an issue in our CFG [#3390](https://github.com/rome/tools/issues/3390)

#### New rules

- [`noAutoFocus`](https://rome.tools/docs/lint/rules/noAutoFocus/)
- [`useAltText`](https://rome.tools/docs/lint/rules/useAltText/)
- [`noBlankTarget`](https://rome.tools/docs/lint/rules/noBlankTarget/)
- [`useAnchorContent`](https://rome.tools/docs/lint/rules/useAnchorContent/)
- [`useKeyWithClickEvents`](https://rome.tools/docs/lint/rules/useKeyWithClickEvents/)
- [`useKeyWithMouseEvents`](https://rome.tools/docs/lint/rules/useKeyWithMouseEvents/)
- [`noPositiveTabIndex`](https://rome.tools/docs/lint/rules/noPositiveTabIndex/)
- [`useValidAnchor`](https://rome.tools/docs/lint/rules/useValidAnchor/)
- [`noRestrictedGlobals`](https://rome.tools/docs/lint/rules/noRestrictedGlobals/)
- [`useSimplifiedBooleanExpression`](https://rome.tools/docs/lint/rules/useSimplifiedBooleanExpression/)
- [`noInvalidConstructorSuper`](https://rome.tools/docs/lint/rules/noInvalidConstructorSuper/)
- [`useValidForDirection`](https://rome.tools/docs/lint/rules/useValidForDirection/)
- [`noConstAssign`](https://rome.tools/docs/lint/rules/noConstAssign/)
- [`noExplicitAny`](https://rome.tools/docs/lint/rules/noExplicitAny/)
- [`noBannedTypes`](https://rome.tools/docs/lint/rules/noBannedTypes/)
- [`useMapFlat`](https://rome.tools/docs/lint/rules/useMapFlat/)
- [`useExhaustiveDependencies`](https://rome.tools/docs/lint/rules/useExhaustiveDependencies/)

### Parser

- Improved messaging of diagnostics, using our new infrastructure
- Fixed an issue where diagnostics couldn't be printed in WASM [#3349](https://github.com/rome/tools/pull/3349)
- Allow arguments in d.ts files [#3388](https://github.com/rome/tools/issues/3388)
- Fix parsing of less than in optional call chains [#3486](https://github.com/rome/tools/issues/3486)
- Fixed a case where `export {"a"} from "b";` wasn't correctly parsed

### VSCode

- Make the "rename" command opt-in and use the VS Code provided "rename" feature that offers whole project renaming instead.
- Added the new command `Restart LSP Server`
- The LSP server is now able to listen to changes of `rome.json` and apply the new configuration



## 0.10.1

### CLI
- Fixed a poor diagnostic that was emitted when navigating a symbolic symbol [#3329](https://github.com/rome/tools/issues/3329)
- Added a size limit when inspecting files [#3330](https://github.com/rome/tools/issues/3330)

### Diagnostics
- Do not print tabs and spaces for unchanged lines [#3327](https://github.com/rome/tools/issues/3327)

### VSCode
- Fixed the calculation of text diffs inside the LSP [#3350](https://github.com/rome/tools/pull/3350)

## 0.10.0

### Core

- Rome is now faster and uses less memory on macOS and Linux systems! [#3237](https://github.com/rome/tools/pull/3237)
- We completely revamped our diagnostics! The new diagnostics allow us to give better information about the errors generated by Rome.
- Greatly increased the performance of Rome's daemon, up to 300%! [#3151](https://github.com/rome/tools/pull/3151)

### Configuration

You can now ignore folders and files using the Unix shell style patterns:

```json
{
  "formatter": {
    "ignore": ["scripts/*.js"]
  },
  "linter": {
    "ignore": ["src/**.test.{ts,js}"]
  }
}
```

### Formatter

- Completely revamped how the formatter handles comments and their placement inside the code [#3277](https://github.com/rome/tools/pull/3227)
- Improved formatting of intersection and unions types [#3162](https://github.com/rome/tools/issues/3162)
- Improved formatting of member chains [#3283](https://github.com/rome/tools/pull/3283)
- Improved formatting of call arguments [#3290](https://github.com/rome/tools/pull/3290)

### Linter

- **BREAKING CHANGE**: This release changes the naming of the lint rule groups with the goal to make them language agnostic and avoid confusion among users and contributors.
were named after a language, and this caused confusion among users and contributors. Please
check our [website](https://rome.tools/docs/lint/rules/) to know better about the new groups.
The new groups are heavily inspired from [`clippy`](https://github.com/rust-lang/rust-clippy#clippy)
- Added a new group called `nursery`, this group incubates new rules that are being developed.
- Added a new group called `style`, this group incubates rules that orbits around styling.
- Added a new group called `correctness`, this group incubates rules that orbits catching possible bugs.
- Fixed a code action for `useBlockStatements` [#3199](https://github.com/rome/tools/issues/3199)
- Improved the rule `useCamelCase` [#3190](https://github.com/rome/tools/pull/3190) [#3210](https://github.com/rome/tools/pull/3210)
- Fixed invalid code action for `useOptionalChain` [#3257](https://github.com/rome/tools/issues/3257)
- Fixed bugs in [`noUnusedVariables`](https://docs.rome.tools/lint/rules/nounusedvariables/) [#3170](https://github.com/rome/tools/issues/3170), [#3316](https://github.com/rome/tools/pull/3316)

#### New rules

- [`useButtonType`](https://rome.tools/docs/lint/rules/useButtonType/)
- [`noRenderReturnValue`](https://rome.tools/docs/lint/rules/noRenderReturnValue/)
- [`noDangerouslySetInnerHtml`](https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml/)
- [`useOptionalChain`](https://rome.tools/docs/lint/rules/useOptionalChain/)
- [`useFragmentSyntax`](https://rome.tools/docs/lint/rules/useFragmentSyntax/)
- [`noUselessFragments`](https://rome.tools/docs/lint/rules/noUselessFragments/)
- [`noChildrenProp`](https://rome.tools/docs/lint/rules/noChildrenProp/)
- [`noArrayIndexKey`](https://rome.tools/docs/lint/rules/noArrayIndexKey/)
- [`noVoidElementsWithChildren`](https://rome.tools/docs/lint/rules/noVoidElementsWithChildren/)
- [`noUndeclaredVariables`](https://rome.tools/docs/lint/rules/noUndeclaredVariables/)
- [`noDangerouslySetInnerHtmlWithChildren`](https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtmlWithChildren/)


### Parser

- Fixed an issue where the parser was _not_ emitting a diagnostic on a certain TypeScript syntax [#3115](https://github.com/rome/tools/issues/3115)

### VSCode

- The setting `lspBin` can be also expressed as **relative path**
- The rules have been added to the configuration schema, allowing users to receive autocomplete
when editing the `rome.json` for the [`rules`](https://rome.tools/#linterrulescorrectness) section


## 0.9.2

### CLI

- Fixes an issue where arguments were not correctly picked up and applied to the formatter [#3175](https://github.com/rome/tools/issues/3175)

## 0.9.1

### CLI

- Fixes a regression where the arguments passed via CLI were ignored [#3175](https://github.com/rome/tools/issues/3175)
- Fixes a regression where the command `rome ci` was not correctly reading the configuration [#3167](https://github.com/rome/tools/issues/3167)

### VSCode

- Windows: fixes an issue where the extension could not load the configuration file [#3182](https://github.com/rome/tools/issues/3182)

## 0.9.0

### CLI

- You can now format content from standard input when using the command `rome format`:
```shell
echo "function f() { return {} }" | rome format --stdin-file-path example.js
```
the argument  `--stdin-file-path` is mandatory when formatting from standard in. The path should represent a
file name with its extension.
- Added `--apply-suggested` argument to the `rome check` command, to apply suggested and safe fixes.
Suggested fixes should be considered **unstable** and applied with care.
- Added the `rome start` and `rome stop` commands to control the Rome daemon server process.
- Added the `--use-server` global flag to the command line to make the CLI connect to a running instance of the
Rome daemon server.

### Configuration

- **BREAKING CHANGE**: removed the second `"rules"` field from a field group.
```diff
{
  "linter": {
    "enabled": true,
    "rules": {
      "js": {
+        "noDebugger": "off"
-        "rules": {
-          "noDebugger": "off"
-        },
      }
    }
  }
}
```
- fixed a problem that was incorrectly turning off rules in certain circumstances

### Formatter

Significantly improved formatting and prettier compatibility of:

* JSX [#3144](https://github.com/rome/tools/pull/3144)
* Conditional expression and conditional types [#2427](https://github.com/rome/tools/issues/2427)
* Function signatures [#2993](https://github.com/rome/tools/pull/2993), [#2990](https://github.com/rome/tools/pull/2990)
* Return and throw statements [#2986](https://github.com/rome/tools/pull/2986)
* Logical and binary expressions [#3079](https://github.com/rome/tools/pull/3079)
* Templates [#3063](https://github.com/rome/tools/pull/3063)
* Arrow expression chains [#3122](https://github.com/rome/tools/pull/3122)
* Member expression assignments [#3061](https://github.com/rome/tools/pull/3061)
* Array expressions [#3126](https://github.com/rome/tools/pull/3126)
* Parenthesized expressions and types, including inserting parentheses to improve readability [#3057](https://github.com/rome/tools/pull/3057), [#3083](https://github.com/rome/tools/pull/3083), [#3108](https://github.com/rome/tools/pull/3108)
* Doc comments [#3129](https://github.com/rome/tools/pull/3129)

### Linter

- Changed the default severity for recommended rules to "error". You can [change the severity in the rome.json](https://rome.tools/#configure-a-rule).
- Added [`js/noExtraBooleanCast`](https://rome.tools/docs/lint/rules/noExtraBooleanCast/) lint rule.
- Added [`js/noDupeArgs`](https://rome.tools/docs/lint/rules/noDupeArgs/) lint rule.
- Added [`js/noShadowRestrictedNames`](https://rome.tools/docs/lint/rules/noShadowRestrictedNames/) lint rule.
- Added `js/inlineVariable` code action.
- Applied various stability fixes to the rule [`js/noUnusedVariables`](https://rome.tools/docs/lint/rules/noUnusedVariables/). [#3124](https://github.com/rome/tools/pull/3124) [#3060](https://github.com/rome/tools/pull/3060) [#3004](https://github.com/rome/tools/pull/3004)
- Fixed how the suggestion is applied [`js/noNegationElse`](https://rome.tools/docs/lint/rules/noNegationElse/). [#2999](https://github.com/rome/tools/issues/2999)
- Fixed a false positive in the rule [`js/noShoutyConstants`](https://rome.tools/docs/lint/rules/noShoutyConstants/). [#3077](https://github.com/rome/tools/issues/3077)
- Fixed a false positive in the rule [`ts/useShorthandArrayType`](https://rome.tools/docs/lint/rules/useShorthandArrayType/). [#3111](https://github.com/rome/tools/issues/3111)

### VSCode

- fixed an issue where it wasn't possible to format newly created files [3006](https://github.com/rome/tools/issues/3006)
- added a status bar [3139](https://github.com/rome/tools/pull/3139)

## 0.8.0

### CLI

- Added `--max-diagnostics` argument to the command `rome check`.
- The maximum number of diagnostics printed is now 20, use `--max-diagnostics` to change the default.
- Added a new command `rome init`.

### Configuration

- You can create a configuration file called `rome.json` to customize Rome's default options.
This will work from both CLI and LSP.

### Formatter

- You can now use the configuration file `rome.json` to change Rome's defaults:

  Example:
  ```json
  {
    "root": true,
    "formatter": {
      "indentStyle": "space"
    }
  }
  ```
- Fixed some edge cases where the comment suppressions were not working as expected.

### Linter

The linter is now marked as "alpha" and it can be used to lint code from the CLI and
from the LSP.


### VSCode

- **BREAKING CHANGE**: Removed the majority of settings that were available in the extension, use the
configuration file `rome.json` to change the Rome's defaults.
- The extension now allows to rename variables;

## 0.7.0

### CLI

- Added `--no-colors` argument.

### Formatter

- JSX and TSX are now formatted by default! Make sure to enable Rome as default formatter in the VSCode extension.
- Improved the consistency of formatting of various statements:
  - call arguments;
  - object property members;
  - variable declarations;
  - object patterns;
  - class property members;
- Fixed a bunch of issues in the TypeScript formatting.

### Linter

- Added the new `--apply` argument to the `rome check` command;
- New rules added to the linter, check the [website](https://rome.tools/docs/lint/rules/);

## 0.6.1

Fixes a regression introduced in the `rome format` command ([#2670](https://github.com/rome/tools/issues/2670))

## 0.6.0

### Formatter

- BREAKING CHANGES: the command `rome format --ci` has been removed, use `rome ci` instead.

#### Improved the compatibility with Prettier (check [#2403](https://github.com/rome/tools/issues/2403) for more details)

- TypeScript's formatting is better in line with what Prettier does.
- Better formatting of string literals.
Removing unnecessary quotes in string literals and quotes from member names.
Correctly choose the correct quote based on quantity of quotes inside a literal:
  ```js
  // original code
  let a = {
    "something": 3
  }
  let b = "cool isn\'t it";
  let c = "\"content\" ' ";

  // formatted code
  let a = {
    something: 3
  }
  let b = "cool isn't it";
  let c = '"content" \' ';
  ```
- Better formatting of various statements
- Improved the performance of the formatter an average of 20%-30%! Check the relevant
PRs [1](https://github.com/rome/tools/pull/2456), [2](https://github.com/rome/tools/pull/2638), [3](https://github.com/rome/tools/pull/2612), [4](https://github.com/rome/tools/pull/2462), [5](https://github.com/rome/tools/pull/2634) if you're interested in what the team did.

To reach better compatibility with Prettier, the team had to revise the foundation of our printer,
which caused some regressions around how comments are printed. These are known issues that we
plan to close by next release.

### Linter

We've built the foundation of our linter. At the moment is only opt-in, and it contains
only a bunch of rules. **Safe fixes are not enabled yet via CLI**.

Refer to the [website](https://rome.tools/#linter) to learn how to start using it.

## 0.5.0

- BREAKING CHANGES: the `format` command doesn't write on disk by default. Now the command prints on terminal.

    **Migration**: add the `--write` argument when calling `rome format`

    ```shell
    rome format --write
    ```

- Added a new option called `--quote-style` to the formatter. This option is also available on VSCode.

## 0.4.0

Rome has been [rewritten in Rust](https://rome.tools/blog/2021/09/21/rome-will-be-rewritten-in-rust)!

The great majority of the previous functionality won't work anymore, as we rewrote the whole software
from scratch.

Rome, for now, exposes a new formatter that has been revisited and, is way faster compared to its former version!

To install it, use the `next` tag on `npm`:

```shell
npm i rome@next
```

Or follow our [getting started](https://rome.tools/#getting-started) section for more details.
