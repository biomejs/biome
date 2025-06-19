# @biomejs/biome

## 2.0.1

### Patch Changes

- [#6362](https://github.com/biomejs/biome/pull/6362) [`a56b37e`](https://github.com/biomejs/biome/commit/a56b37ea5f547f77640e75991c25b564785594e3) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6360](https://github.com/biomejs/biome/issues/6360): The following pseudo classes and elements are no longer reported by `noUnknownPseudoClass` or `noUnknownPseudoElement` rules.

  - `:open`
  - `::details-content`
  - `::prefix`
  - `::search-text`
  - `::suffix`

- [#6378](https://github.com/biomejs/biome/pull/6378) [`64d3d1a`](https://github.com/biomejs/biome/commit/64d3d1a81675d3b333fd1165db9e0844b822df02) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6357](https://github.com/biomejs/biome/issues/6357), where the boolean values weren't correctly merged when using the `extends` functionality. Now Biome correctly merges the values.

- [#6342](https://github.com/biomejs/biome/pull/6342) [`46428bb`](https://github.com/biomejs/biome/commit/46428bb7f6df2215ad1f07d3f0776a9aef1258a6) Thanks [@Jentob](https://github.com/Jentob)! - Fixed [#6341](https://github.com/biomejs/biome/issues/6341): Fixed an issue where Biome would throw an error for the language tags `nb` and `nn`.

- [#6323](https://github.com/biomejs/biome/pull/6323) [`04bd1a7`](https://github.com/biomejs/biome/commit/04bd1a7d68f6612ceefd93b225af91b77b79df67) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6278](github.com/biomejs/biome/issues/6278): `useExhaustiveDependencies` no longer adds duplicated dependencies into the list.

- [#6397](https://github.com/biomejs/biome/pull/6397) [`50b7fe6`](https://github.com/biomejs/biome/commit/50b7fe676f5aa4735e0868a8e9f40b57c6a5b3da) Thanks [@wojtekmaj](https://github.com/wojtekmaj)! - Fix #6396, where `vi.useFakeTimers()` and `vi.useRealTimers()` incorrectly triggered React Hooks-related rules

- [#6378](https://github.com/biomejs/biome/pull/6378) [`64d3d1a`](https://github.com/biomejs/biome/commit/64d3d1a81675d3b333fd1165db9e0844b822df02) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where Biome didn't correctly discover nested configuration files when using the `lint` command and the linter is disabled in the root configuration.

## 2.0.0

### Major Changes

- Biome now resolves globs and paths from the configuration. Before, paths and globs were resolved from the working directory.

- Biome now raises a **warning** diagnostic for suppression comments that have `<explanation>` as reason.

  `<explanation>` is provided as a placeholder when applying the suppression code fix from LSP editors.

- Removed the `--config-path` argument from the `biome lsp-proxy` and `biome start` commands.

  The option was overriding the configuration path for all workspaces opened in the Biome daemon, which led to a configuration mismatch problem when multiple projects are opened in some editors or IDEs.

  If you are using one of our official plugins for IDEs or editors, it is recommended to update it to the latest version of the plugin, or you will get unexpected behavior.

  If you are a developer of a plugin, please update your plugin to use the `workspace/configuration` response instead of using the `--config-path` argument. Biome's LSP will resolve a configuration in the workspace automatically, so it is recommended to keep it empty unless you are using a custom configuration path.

- Downgraded some code fixes to unsafe which were previously safe.

  The following rules have now a unsafe fix:

  - [`noFlatMapIdentity`](https://biomejs.dev/linter/rules/no-flat-map-identity)
  - [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports)

  If you want to keep applying these fixes automatically, [configure the rule fix](https://next.biomejs.dev/linter/#configure-the-code-fix) as safe:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noFlatMapIdentity": {
            "level": "error",
            "fix": "safe"
          },
          "noUnusedImports": {
            "level": "error",
            "fix": "safe"
          }
        }
      }
    }
  }
  ```

- Previously the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` checked both regular expression literals like `/regex/` and dynamically built regular expressions like `new RegExp("regex")`.

  Checking dynamically built regular expressions has many limitations, edge cases, and complexities.
  In addition, other rules that lint regular expressions don't check dynamically built regular expressions.

  Rather than add support for other rules and have half-baked checking, we decided to remove support for dynamically built regular expressions.

  Now the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` only check literals of regular expressions.

- The lint rule [`noRestrictedGlobals`](https://biomejs.dev/linter/rules/no-restricted-globals/) now supports customizing message for each global name.

  For example, the following configuration:

  ```json
  {
    "options": {
      "deniedGlobals": {
        "$": "jQuery is not allowed. Use native DOM manipulation instead."
      }
    }
  }
  ```

  emits a diagnostic:

  ```
  index.js:1:13 lint/style/noRestrictedGlobals ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ⚠ Do not use the global variable $.

    > 1 │ console.log($)
        │             ^
      2 │

    ℹ jQuery is not allowed. Use native DOM manipulation instead.
  ```

  Breaking Change: The option `deniedGlobals` is now a record instead of an array. Run `biome migrate` to migrate the configuration automatically.

- Removed `include` and `ignore` fields in favor of the new field `includes`.

  The Biome configuration file allows users to specify which files should be processed using [glob patterns](<https://en.wikipedia.org/wiki/Glob_(programming)>).
  Prior to Biome 2.0, this was done using the `include` and `ignore` fields.
  In Biome 2.0, `include` and `ignore` are removed and replaced by `includes`.
  You can run `biome migrate` to convert `include` and `ignore` into `includes` automatically.

  `includes` uses a different glob pattern format that fixes [many](https://github.com/biomejs/biome/issues/2421) [issues](https://github.com/biomejs/biome/issues/3345) and many other limitations that Biome users reported.

  `includes` accepts an array of glob patterns.
  A glob pattern starting with a `!` is a negated pattern also called exception.
  This replaces `ignore` patterns and allows users to create chains of include and ignore patterns.
  Thus, it is now possible to include again a file previously ignored.
  This was not possible with `include` and `ignore`, because `ignore` has priority over `include`.

  The semantics of `*` and `**/*` have changed too.
  Before, with `include` and `ignore`, the glob `*` was interpreted as `**/*`.
  Now, with `includes`, the globs `*` and `**/*` are interpreted differently.
  The first pattern matches all files that are inside a folder.
  The second pattern recursively matches all files **and sub-folders** inside a folder.

  Let's take an example.
  Given the following file hierarchy of a project...

  ```
  ├── biome.json
  ├── src
  │   ├── file.js
  │   ├── file.ts
  │   ├── out.gen.js
  │   ├── file.test.js
  │   └── test
  │       └── special.test.js
  └── test ...
  ```

  ...we want:

  1. Ignore all files ending with `.test.js`, except `special.test.ts`.
  2. Ignore all files of the `test` directory.
     The `test` directory is located at the root of the project.
  3. Execute the linter on files in the `src` directory, that don't end with `.gen.js`.
     The `src` directory is located at the root of the project.
  4. Enable the `noDefaultExport` lint rule on files ending with `.ts`.

  Prior to Biome 2.0, the configuration might look like:

  ```json
  {
    "files": {
      "ignore": ["*.test.js", "test"]
    },
    "linter": {
      "include": ["src/**"],
      "ignore": ["*.gen.js"],
      "enabled": true
    },
    "overrides": [
      {
        "include": ["*.ts"],
        "linter": { "rules": { "style": { "noDefaultExport": "on" } } }
      }
    ]
  }
  ```

  Unfortunately, the configuration doesn't quite fit what we want:

  1. There is no way to ignore files and unignore one of them.
     Thus, we ignore all files ending with `.test.js`, including `special.test.ts`.
  2. The configuration ignores all directories named `test`, including `src/test`.
  3. The linter is executed on all files of all directories named `src`

  All these issues and limitations are fixed with `includes`.
  Here the migrated configuration:

  ```json
  {
    "files": {
      "includes": ["**", "!**/*.test.js", "**/special.test.ts", "!test"]
    },
    "linter": {
      "includes": ["src/**", "!**/*.gen.js"],
      "enabled": true
    },
    "overrides": [
      {
        "includes": ["**/*.ts"],
        "linter": { "rules": { "style": { "noDefaultExport": "on" } } }
      }
    ]
  }
  ```

  1. All files named `special.test.ts` are unignored because the pattern appear after the pattern that ignore files ending with `.test.js`.
  2. Only the `test` directory at the project's root is ignored because the pattern doesn't start with `**/`.
  3. The linter is executed on the `src` directory at the project's root only.

  Because `includes` pattern have a different pattern format than `include` and `ignore` we made some adjustments:

  - We added the pattern `**` in `files.includes` to ensure that all files are included before ignoring some of them.
  - We added the prefix `**/` for patterns that must match at any level of the file hierarchy.

- `noUndeclaredVariables` no longer reports TypeScript types.

  In TypeScript projects, developers often use global declaration files to declare global types.
  Biome is currently unable to detect these global types.
  This creates many false positives for `noUndeclaredVariables`.

  TypeScript is better suited to perform this kind of check.
  As proof of this, TypeScript ESLint doesn't provide any rule that extends the `no-undef` ESLint rule.

  This is why Biome 1.9 introduced a new option `checkTypes` which, when it is set to false, ignores undeclared type references.
  The option was set to `true` by default.

  This option is now set to `false` by default.
  To get the previous behavior, you have to set `checkTypes` to `true`:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noUndeclaredVariables": {
            "level": "on",
            "options": { "checkTypes": true }
          }
        }
      }
    }
  }
  ```

- The rule `noUnusedVariables` no longer reports unused function parameters. Use [`noUnusedFunctionParameters`](https://biomejs.dev/linter/rules/no-unused-function-parameters/).

- Fixed [#5564](https://github.com/biomejs/biome/issues/5564). `noTypeOnlyImportAttributes` now ignores files ending with the extension `.ts` when the type field of `package.json` is set to `commonjs`.

- The Biome formatter no longer adds a trailing comma in `.json` files, even when `json.formatter.trailingCommas` is set to `true`.

- [Prettier 3.4](https://prettier.io/blog/2024/11/26/3.4.0.html) introduced a change in their normalization process of string literals: it no longer unescapes useless escape sequences.
  Biome now matches the new behavior of Prettier when formatting code.
  This affects the JSON and JavaScript formatters.

- Reduced accepted values for formatter options:

  - The option `--quote-style` doesn't accept `Single` and `Double` anymore.
  - The option `--quote-properties` doesn't accept `AsNeeded` and `Preserve` anymore.
  - The option `--semicolons` doesn't accept `AsNeeded` and `Always` anymore.
  - The option `--arrow-parenthesis` doesn't accept `AsNeeded` and `Always` anymore.
  - The option `--trailing-commas` doesn't accept `ES5`, `All` and `None` anymore.
  - The option `--attribute-position` doesn't accept `Single` and `Multiline` anymore.

- Removed the option `enumMemberCase` from the lint rule `useNamingConvention`.

  `enumMemberCase` is an option that allows to customize the enforced case for TypeScript's enum members.
  The option was introduced prior to the `conventions` option that allows to do the same thing.

  The following configuration...

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "useNamingConvention": {
            "level": "on",
            "options": {
              "enumMemberCase": "PascalCase"
            }
          }
        }
      }
    }
  }
  ```

  ...must be rewritten as:

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "useNamingConvention": {
            "level": "on",
            "options": {
              "conventions": [
                {
                  "selector": { "kind": "enumMember" },
                  "formats": ["PascalCase"]
                }
              ]
            }
          }
        }
      }
    }
  }
  ```

  Run `biome migrate --write` to turn `enumMemberCase` into `conventions` automatically.

- Removed support for `BIOME_LOG_DIR`.

  The environment variable `BIOME_LOG_DIR` isn't supported anymore.

  Use `BIOME_LOG_PATH` instead.

- Remove deprecated rules.

  The following _deprecated_ rules have been deleted:

  - `noInvalidNewBuiltin`
  - `noNewSymbol`
  - `useShorthandArrayType`
  - `useSingleCaseStatement`
  - `noConsoleLog`

  Run the command `biome migrate --write` to update the configuration.

- Removed the deprecated `indentSize` option.

  The deprecated option `indentSize`, and its relative CLI options, has been removed:

  - Configuration file: `formatter.indentSize`
  - Configuration file: `javascript.formatter.indentSize`
  - Configuration file: `json.formatter.indentSize`
  - CLI option `--indent-size`
  - CLI option `--javascript-formatter-indent-size`
  - CLI option `--json-formatter-indent-size`

  Use `indentWidth` and its relative CLI options instead.

- Removed `ROME_BINARY`. Use `BIOME_BINARY` instead.

- Removed support for legacy suppressions.

  Biome used to support "legacy suppressions" that looked like this:

  ```js
  // biome-ignore lint(complexity/useWhile): reason
  ```

  This format is no longer supported.

- Removed support for `max_line_length` from `.editorconfig`, as it isn't part of the official spec anymore.

- Removed support for `rome-ignore` suppression comments.

  Use `biome-ignore` suppression comments instead.

- Removed support for `rome.json`.

  Use `biome.json` or `biome.jsonc` instead.

- Removed the option `all` from the linter.

  The options `linter.rules.all` and `linter.rules.<group>.all` has been removed.

  The number of rules in Biome have increased in scope and use cases, and sometimes some of them can conflict with each other.

  The option was useful at the beginning, but now it's deemed harmful, because it can unexpected behaviours in users projects.

  To automatically remove it, run the following command:

  ```shell
  biome migrate --write
  ```

- Removed the option `trailingComma` from the configuration and the CLI. Use the option `trailingCommas` instead:

  ```diff
  {
    "javascript": {
      "formatter": {
  -      "trailingComma": "es5"
  +      "trailingCommas": "es5"
      }
    }
  }
  ```

  ```diff
  -biome format --trailing-comma=es5
  +biome format --trailing-commas=es5
  ```

- Removed `--apply` and `--apply-unsafe`.

  The CLI options `--apply` and `--apply-unasfe` aren't accepted anymore. Use `--write` and `--write --unafe` instead:

  ```diff
  -biome check --apply-unsafe
  +biome check --write --unsafe
  ```

  ```diff
  -biome check --apply
  +biome check --write
  ```

- Removed support for `assert` syntax.

  Biome now longer supports the `assert` syntax, use the new `with` syntax instead

  ```diff
  -import {test} from "foo.json" assert { for: "for" }
  -export * from "mod" assert { type: "json" }
  +import {test} from "foo.json" with { for: "for" }
  +export * from "mod" with { type: "json" }
  ```

- Fixed [#5495](https://github.com/biomejs/biome/issues/5495): The rule
  [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) has been
  updated to accept the `rel="noopener"` in addition to `rel="noreferrer"`.
  In addition, an option has been added that allows `rel="noreferrer"` to be
  disabled.

  The rule has been moved from the `a11y` group to the `security` group.

- The rule `useImportRestrictions` has been renamed to [`noPrivateImports`](https://biomejs.dev/linter/rules/no-private-imports), and its
  functionality has been significantly upgraded.

  Previously, the rule would assume that any direct imports from modules inside
  other directories should be forbidden due to their _package private_ visibility.

  The updated rule allows configuring the default visibility of exports, and
  recognises JSDoc comments to override this visibility. The default visibility
  is now `**public**`, but can be set to `**package**`, or even `**private**`.
  Refer to the [documentation of the rule](https://biomejs.dev/linter/rules/no-private-imports) to understand how to leverage the JSDoc comments.

  `noPrivateImports` is now recommended by default.

- The Biome daemon now reuses its workspace across connections. This allows multiple clients to
  reuse the same documents and other cached data that we extract from them.

  This primarily affects our IDE extensions: If you open multiple IDEs/windows for the same project,
  they'll connect to the same daemon and reuse each other's workspace.

  The Biome CLI is unaffected unless you opt in with the `--use-server` argument.

- Biome no longer treats too large files as errors.

  Previously, files that exceed the configured size limit would throw an error, and the CLI would exit with an error code.

  Now, the CLI ignores the file, emits an _information_ diagnostic and doesn't exit with an error code.

- Change the group of some rules, promote nursery rules and update the recommended rule set.

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
  Use the `biome migrate` command to automatically migrate nursery rules that have been promoted.

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

- Update the default severity level of lint rules.

  Every diagnostic emitted by Biome has a severity level set to `error`, `warn`, or `info`.
  Previously, all recommended lint rules had a default severity level set to `error`.
  All other lint rules had a default severity level set to `warn`.

  We have adjusted the default severity level of every rule, whether recommended or not, to better communicate the _severity_ that a diagnostic highlights.

  - Rules that report hard errors, likely erroneous code, dangerous code, or accessibility issues now have a default severity level of `error`.
  - Rules that report possibly erroneous codes, or code that could be cleaner if rewritten in another way now have a default severity level of `warn`.
  - Rules that reports stylistic suggestions now have a default severity level of `info`.

  You can use the CLI option `--diagnostic-level=error` to display only errors, or `--diagnostic-level=warning` to display both errors and warnings.
  By default, all diagnostics are shown.
  You can also use the CLI option `--error-on-warnings` to make the command fail when warnings are emitted.

- Reworked some recommended rules recommended to be less pedantic and blocking. This is a **breaking change** if your project relied on those rules to block the CI in case of violations; if that's the case, you should raise their severity level to **error**.

  Some rules aren't recommended anymore, and some others return a different severity.

  The following rules return a **warning** diagnostic:

  - `noDelete`
  - `noForEach`
  - `noSuspiciousSemicolonInJsx`
  - `noThisInStatic`
  - `noUnusedLabels`

  The following rules return an **information** diagnostic:

  - `noUselessCatch`
  - `noUselessConstructor`
  - `noUselessEmptyExport`
  - `noUselessFragments`
  - `noUselessLabel`
  - `noUselessLoneBlockStatements`
  - `noUselessSwitchCase`
  - `noUselessTernary`
  - `noUselessThisAlias`
  - `noUselessTypeConstraint`
  - `noFlatMapIdentity`

  The following rules aren't recommended anymore:

  - `noDelete`
  - `noForEach`

  The rule `noRenderReturnValue` and `useExhaustiveDependencies` are only recommended when the `react` domain is enabled.

- Renamed the global option `--skip-errors` to `--skip-parse-errors`.

- Remove the code action `quickfix.suppressRule`.

  The code action `quickfix.suppressRule` was removed in favour of two new code actions:

  - `quickfix.suppressRule.inline.biome`: a code action that adds a suppression comment for each violation.
  - `quickfix.suppressRule.topLevel.biome`: a code action that adds a suppression comment at the top of the file which suppresses a rule for the whole file.

  Given the following code

  ```js
  let foo = "one";
  debugger;
  ```

  The code action `quickfix.suppressRule.inline.biome` will result in the following code:

  ```js
  // biome-ignore lint/style/useConst: <explanation>
  let foo = "one";
  // biome-ignore lint/suspicious/noDebugger: <explanation>
  debugger;
  ```

  The code action `quickfix.suppressRule.topLevel.biome`, instead, will result in the following code:

  ```js
  /** biome-ignore lint/suspicious/noDebugger: <explanation> */
  /** biome-ignore lint/style/useConst: <explanation> */

  let foo = "one";
  debugger;
  ```

- Changed default formatting of `package.json`.

  When Biome encounters a file called `package.json`, by default it will format the file with all objects and arrays expanded.

  ```diff
  - { "name": "project", "dependencies": { "foo": "latest" } }
  + {
  +  "projectName": "project",
  +  "dependencies": {
  +    "foo": "^1.0.0"
  +  }
  + }
  ```

- The `organizeImports` is now part of Biome Assist.

- The rule [`noVar`](https://biomejs.dev/linter/rules/no-var/) now belongs to the `suspicious` group

- The rule [`useWhile`](https://biomejs.dev/linter/rules/use-while/) now belongs to the `complexity` group.

- The rule [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) has been updated to suggest actual file extensions instead of guesses based on hueristics.

  As part of this, the `suggestedExtensions` option has been removed. A simpler,
  new option called `forceJsExtensions` has been introduced for those who use
  `tsc`'s `"module": "node16"` setting.

  The rule also no longer reports diagnostics to add an extension when the path
  doesn't exist at all, with or without extension.

- Fixed [#4545](https://github.com/biomejs/biome/issues/4545): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now correctly ignores declarations inside TypeScript's external modules.

  The following interface name is no longer reported by the rule:

  ```ts
  declare module "myExternalModule" {
    export interface my_INTERFACE {}
  }
  ```

- The rule [`useAltText`](https://biomejs.dev/linter/rules/use-alt-text/) no longer checks the element's attributes containing object spread.

  The following code doesn't trigger the rule anymore:

  ```jsx
  <img src="test.png" alt={alt} {...restProps}></img>
  ```

- The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer accepts non-ASCII characters by default.

  Prior to Biome 2.0, non-ASCII names were accepted by default. They are now rejected.

  For example, the following code is now reported as invalid by the `useNamingConvention` rule.

  ```js
  let johnCafé;
  ```

  If you want to allow non ASCII filenames and non-ASCII identifiers, you need to set the `requireAscii` options in your Biome configuration file to `false`:

  ```json
  {
      "linter": {
          "rules": {
              "style": {
                  "useFilenamingConvention": {
                      "level": "on",
                      "options": {
                          "requireAscii": false
                      }
                  }
                  "useFilenamingConvention": {
                      "level": "on",
                      "options": {
                          "requireAscii": false
                      }
                  }
              }
          }
      }
  }
  ```

- Renamed the rule `noUnnecessaryContinue` to `noUselessContinue`. Run the command `biome migrate` to update your configuration.

- Renamed the rule `noMultipleSpacesInRegularExpressionLiterals` to `noAdjacentSpacesInRegex`. Run the command `biome migrate` to update your configuration.

### Minor Changes

- An option called `allowNoReferrer` has been added to the
  [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) rule.

  By default, `noBlankTarget` accepts both `rel="noopener"` and `rel="noreferrer"`
  with links that have `target="_blank"`. This is because the latter _implies_ the
  former, so either one is sufficient to mitigate the security risk.

  However, allowing `rel="noreferrer"` may still be undesirable, because it can
  break tracking, which may be an undesirable side-effect. As such, you can set
  `allowNoReferrer: false` to _only_ accept `rel="noopener"`.

- Added new option `javascript.parser.jsxEverywhere`. This new option allows to control whether Biome should expect JSX syntax in `.js`/`.mjs`/`.cjs` files.

  When `jsxEverywhere` is set to `false`, having JSX syntax like `<div></div>` inside `.js`/`.mjs`/`.cjs` files will result in a **parsing error**.

  Despite the name of the option, JSX is never supported inside `.ts` files. This is because TypeScript generics syntax may conflict with JSX in such files.

  This option defaults to `true`.

- Add a new JS assist rule - `useSortedKeys` which enforces ordering of a JS object properties.
  This rule will consider spread/calculated keys e.g `[k]: 1` as non-sortable.
  Instead, whenever it encounters a non-sortable key, it will sort all the
  previous sortable keys up until the nearest non-sortable key, if one exist.
  This prevents breaking the override of certain keys using spread keys.

  Source: https://perfectionist.dev/rules/sort-objects

  ```js
  // Base
  // from
  const obj = {
    b: 1,
    a: 1,
    ...g,
    ba: 2,
    ab: 1,
    set aab(v) {
      this._aab = v;
    },
    [getProp()]: 2,
    aba: 2,
    abc: 3,
    abb: 3,
    get aaa() {
      return "";
    },
  };
  // to
  const obj = {
    a: 1,
    b: 1,
    ...g,
    set aab(v) {
      this._aab = v;
    },
    ab: 1,
    ba: 2,
    [getProp()]: 2,
    get aaa() {
      return "";
    },
    aba: 2,
    abb: 3,
    abc: 3,
  };
  ```

- Added the new rule [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises).

- Added the new rule [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles).

- Added the new rule [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore).

- Added the new rule [`noUnwantedPolyfillio`](https://biomejs.dev/linter/rules/no-unwanted-polyfillio).

- Added a format option `expand` for Javascript and JSON formatters.
  The option allows to enforce the formatting of arrays and objects on multiple lines, regardless of their length.
  It has three options:

  When set to `auto` (default), objects are expanded if the first property has a leading newline.
  Arrays are collapsed when they fit to a single line.
  For example, both styles below are considered as already formatted:

  ```js
  const obj = {
    foo: "bar",
  };
  ```

  ```js
  const obj = { foo: "bar" };
  ```

  When set to `always`, objects and arrays are always expanded.

  When set to `never`, objects and arrays are never expanded when they fit in a single line.
  It is equivalent to Prettier's [Object Wrap](https://prettier.io/docs/options#object-wrap) option with `collapse`.

- The nursery rule [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) has been added.

  Importing a non-existing export is an error at runtime or build time. With this
  rule, Biome can detect such incorrect imports and report errors for them.

  Note that if you use TypeScript, you probably don't want to use this rule, since
  TypeScript already performs such checks for you.

- The rule [`noFocusedTests`](https://biomejs.dev/linter/rules/no-focused-tests/) can now detect the usage of focused tests inside loops.

  ```js
  // invalid
  describe.only.each([["a"], ["b"]])("%s", (a) => {});
  it.only.each([["a"], ["b"]])("%s", (a) => {});
  test.only.each([["a"], ["b"]])("%s", (a) => {});

  // valid
  describe.each([["a"], ["b"]])("%s", (a) => {});
  it.each([["a"], ["b"]])("%s", (a) => {});
  test.each([["a"], ["b"]])("%s", (a) => {});
  ```

- Linter groups now accept new options to enable/disable all rules that belong to a group, and control the severity
  of the rules that belong to those groups.

  For example, you can downgrade the severity of rules that belong to `"style"` to emit `"info"` diagnostics:

  ```json
  {
    "linter": {
      "rules": {
        "style": "info"
      }
    }
  }
  ```

  You can also enable all rules that belong to a group using the default severity of the rule using the `"on"` option:

  ```json
  {
    "linter": {
      "rules": {
        "complexity": "on"
      }
    }
  }
  ```

- Biome assist is a new feature of the Biome analyzer. The assist is meant to provide **actions**. Actions differ from linter rules in that they aren't meant to signal errors.

  The assist will provide code actions that users can opt into via configuration or via IDEs/editors, using the Language Server Protocol.

  The assist **is enabled by default**. However, you can turn if off via configuration:

  ```json
  {
    "assist": {
      "enabled": false
    }
  }
  ```

  You can turn on the actions that you want to use in your configuration. For example, you can enable the `useSortedKeys` action like this:

  ```json
  {
    "assist": {
      "actions": {
        "source": {
          "useSortedKeys": "on"
        }
      }
    }
  }
  ```

  Alternatively, IDE/editor users can decide which action to apply on save _directly from the editor settings_, as long as the assist is enabled.

  For example, in VS Code you can apply the `useSortedKeys` action when saving a file by adding the following snippet in `settings.json`:

  ```json
  {
    "editor.codeActionsOnSave": {
      "source.biome.useSortedKeys": "explicit"
    }
  }
  ```

  In Zed, you can achieve the same by adding the following snippet in `~/.config/zed/settings.json`:

  ```json
  {
    "code_actions_on_format": {
      "source.biome.useSortedKeys": true
    }
  }
  ```

- Biome migrate eslint outputs a better overriding behavior.

  A Biome rule can have multiple ESLint equivalent rules.
  For example, [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) has two ESLint equivalent rules: [dot-notation](https://eslint.org/docs/latest/rules/dot-notation) and [@typescript-eslint/dot-notation](https://typescript-eslint.io/rules/dot-notation/).

  Previously, Biome wouldn't always enable a Biome rule even if one of its equivalent rules was enabled.
  Now Biome uses the higher severity level of all the equivalent ESLint rules to set the severity level of the Biome rule.

  The following ESLint configuration...

  ```json
  {
    "rules": {
      "@typescript-eslint/dot-notation": "error",
      "dot-notation": "off"
    }
  }
  ```

  ...is now migrated to...

  ```json
  {
    "linter": {
      "rules": {
        "complexity": {
          "useLiteralKeys": "error"
        }
      }
    }
  }
  ```

  ...because `error` is higher than `off`.

- Add [useSymbolDescription](https://biomejs.dev/linter/rules/use-symbol-description/).

- Enhanced the command `migrate eslint`. Now the command shows which ESLint rules were migrated,
  and which rules aren't supported yet.

  ```
  ./eslint.config.js migrate ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ℹ 42% (3/7) of the rules have been migrated.

    ℹ Migrated rules:

    - getter-return
    - prefer-const
    - @typescript-eslint/require-await

    ℹ Rules that can be migrated to an inspired rule using --include-inspired:

    - @typescript-eslint/parameter-properties

    ℹ Rules that can be migrated to a nursery rule using --include-nursery:

    - @typescript-eslint/switch-exhaustiveness-check

    ℹ Stylistic rules that the formatter may support (manual migration required):

    - semi

    ℹ Unsupported rules:

    - block-scoped-var

  configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ℹ Migration results:

    - ./biome.json: configuration successfully migrated.
  ```

- Suppression of syntax rules

  Added support for suppressing syntax rules. Syntax rules are particular rules meant **to complement the parser**, hence they can't be configured.

  Biome now allows to suppress those rules. This can, for example, be useful in case the rule is affected by a bug. However, this is more an escape hatch, so if a syntax rule requires a suppression, please file an issue.

  Example:

  ```typescript
  // biome-ignore syntax/correctness/noTypeOnlyImportAttributes: bug
  import type { MyType } from "my-esm-pkg" with { "resolution-mode": "import" };
  ```

  Biome now requires all `biome-ignore-start` suppressions to have an equivalent `biome-ignore-end` comment.

- Add a new lint rule `noConstantBinaryExpression`.
  This rule is inspired from ESLint's [no-constant-binary-expression](https://eslint.org/docs/latest/rules/no-constant-binary-expression) rule.

- The CLI options `--only` and `--skip` now accept rule and action names without prefixing the group name.

  Previously `--only=noDebugger` was rejected.
  You had to add the group name: `--only=suspicious/noDebugger`.

- Fixed [#3574](https://github.com/biomejs/biome/issues/3574): `noUnusedImports` now reports empty named imports and suggests their removal.

  The rule now suggests the removal of empty named imports such as:

  ```diff
  - import {} from "mod";
  ```

- Added the new rule [`useAdjacentGetterSetter`](https://biomejs.dev/linter/rules/use-adjacent-getter-setter), which enforces getters and setters for the same property
  to be adjacent in class and object definitions.

  **Example (Invalid): Name getter and setter are not adjacent:**

  ```js
  class User {
    get name() {
      return this._name;
    }
    constructor() {}
    set name(value) {
      this._name = value;
    }
  }
  ```

  \*\*Example (Invalid): Getter should go before the setter.

  ```js
  const user = {
    set name(value) {
      this._name = value;
    },
    get name() {
      return this._name;
    },
  };
  ```

  **Example (Valid): Name getter and setter are adjacent:**

  ```js
  class User {
    get name() {
      return this._name;
    }
    set name(value) {
      this._name = value;
    }
    get age() {
      return this._age;
    }
    set age(age) {
      this._age = age;
    }
  }
  ```

- Added new rule [useConsistentResponse](https://biomejs.dev/linter/rules/use-consistent-response) which suggests to use static [Response.json()](https://developer.mozilla.org/en-US/docs/Web/API/Response/json) and [Response.redirect()](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static) methods instead of `new Response` when possible.

  Example:

  ```js
  new Response(JSON.stringify({ value: 1 }));
  Response.json({ value: 1 });
  ```

- Biome users can now configure code actions from linter rules as well as assist actions directly in the settings of their IDE/editor.

  For example, let's consider the lint rule [`noSwitchDeclarations`](https://biomejs.dev/linter/rules/no-switch-declarations/), which has an unsafe fix.
  Previously, if you wanted to use this rule, you were "forced" to enable it via configuration, and if you wanted to apply its fix when you saved a file, you were forced to mark the fix as safe:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noSwitchDeclarations": {
            "level": "error",
            "fix": "safe"
          }
        }
      }
    }
  }
  ```

  Now, you can benefit from the code action without making the fix safe for the entire project. IDEs and editors that are LSP compatible allow to list a series of "filters" or code actions that can be applied on save. In the case of VS Code, you will need to add the following snippet in the `settings.json`:

  ```json
  {
    "editor.codeActionsOnSave": {
      "quickfix.biome.correctness.noSwitchDeclarations": "explicit"
    }
  }
  ```

  Upon save, Biome will inform the editor the apply the code action of the rule `noSwitchDeclarations`.

- Fixed [#3401](https://github.com/biomejs/biome/issues/3401): `noUnusedImports` now keeps comments separated from the import with a blank line.

  For example:

  ```diff
    // Orphan comment

  - // Header comment
  - import {} from "mod";
  ```

- Added a new `propertyAssignment` option to the `noParameterAssign` rule.
  This option allows to configure whether property assignments on function parameters are permitted.
  By default, `propertyAssignment` is set to `allow`.
  Setting it to `deny` enforces stricter immutability by disallowing property mutations on function parameters.

- The formatter option `bracketSpacing` is now also supported in JSON files.

- `useValidTypeof` now accepts comparisons with variables.

  Previously, the rule required to compare a `typeof` expression against another `typeof` expression or a valid string literal. We now accept more cases, notably comparison against a variable:

  ```js
  if (typeof foo === bar) {
    // ...
  }
  ```

- Added the new rule [`noNestedComponentDefinitions`](https://biomejs.dev/linter/rules/no-nested-component-definitions),
  which disallows nested component definitions in React components.

  This rule is useful for preventing potential performance issues and improving code readability by ensuring that components are defined at the top level.

  **Example (Invalid):**

  ```jsx
  function ParentComponent() {
    function ChildComponent() {
      return <div>Hello</div>;
    }
    return <ChildComponent />;
  }
  ```

  **Example (Valid):**

  ```jsx
  function ChildComponent() {
    return <div>Hello</div>;
  }
  function ParentComponent() {
    return <ChildComponent />;
  }
  ```

- Added the new rule [`noDestructuredProps`](https://biomejs.dev/linter/rules/no-destructured-props/), which disallow the use of destructured props in Solid projects.

- You can now enable lint rules using the default severity suggested by Biome using the new variant `"on"`, when enabling a rule.

  For example, the default severity of the rule `style.noVar` is `error`, so you would use `"on"`, and then linting a code that uses `var`, will result in an error:

  ```json
  {
    "linter": {
      "recommended": false,
      "rules": {
        "style": {
          "noVar": "on"
        }
      }
    }
  }
  ```

  ```js
  // main.js
  var name = "tobias";
  ```

  The command `biome lint main.js` will result in an error due to the default severity assigned to `noVar`.

  Refer to the documentation page of each rule to know their suggested diagnostic severity, or use the command `biome explain <RULE_NAME>`:

  ```shell
  biome explain noVar
  ```

- Biome VCS integration now supports nested ignore files.

  For `git`, if a `.gitignore` is found in a nested folder `root/packages/foo/`, and it contains the pattern `dist/`, only files and directories inside `root/packages/foo/dist` are matched.

- Added the rule [useUniqueElementIds](https://biomejs.dev/linter/rules/use-unique-element-ids/).
  This rule disallows the use of static IDs in React components. It encourages to generate unique IDs for accessibility purposes using [`useId`](https://react.dev/reference/react/useId).

  The following code is now reported as invalid:

  ```jsx
  function App() {
    return <div id="static-id" />;
  }
  ```

  The following code is now reported as valid:

  ```jsx
  import { useId } from "react";
  function App() {
    const id = useId();
    return <div id={id} />;
  }
  ```

- Added the new JavaScript rule [`useConsistentObjectDefinition`](https://biomejs.dev/linter/rules/use-consistent-object-definition/) rule. The rule enforces a consistent style for the definition of objects:

  By default, the rule enforces a shorthand style:

  ```js
  const validShorthand = {
    // Property shorthand
    foo,

    // Method shorthand
    method() {
      return "method";
    },
  };
  ```

  Alternatively, the rule can be configured to enforce an explicit style:

  ```js
  const invalidExplicit = {
    // Basic property shorthand violations
    foo: foo,

    // Method shorthand violations
    method: function () {
      return "method";
    },
  };
  ```

- Introduced more advanced logging capabilities:

  Every Biome CLI command can now be passed a `--log-file=<path>` argument, which
  will write all log messages for that invocation to the given path instead of
  `stdout`.

  In addition, the `--log-level` parameter now also accepts a `tracing` value.
  When `--log-level=tracing` is used, Biome also prints timing information from
  tracing spans to the log.

  Combined with Biome's ability to print logs in JSON format, and the `jq` command
  line utility, this allows you to perform advanced analysis on Biome's internal
  performance.

  For example, if you want to figure out which paths take the longest when
  building the module graph, you can use the following commands:

  ```sh
  biome lint --log-level=tracing --log-kind=json --log-file=tracing.json
  cat tracing.json | jq '. | select(.span.name == "update_module_graph") | { path: .span.path, time_busy: .["time.busy"], time_idle: .["time.idle"] }' > filtered.json
  ```

  Now you will have a file called `filtered.json` with all the relevant timings,
  together with the paths used during the invocations.

- Added options to `suspicious/noConfusingLabels` to allow specific labels.

- Fixed [#4549](https://github.com/biomejs/biome/issues/4549): [noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property/) now accepts more known CSS properties.

  ```diff
  - ['anchor-default', 'anchor-scroll', 'inset-area', 'position-animation', 'position-fallback', 'position-fallback-bounds', 'position-try-options']
  + ['anchor-scope', 'interpolate-size', 'line-fit-edge', 'masonry', 'masonry-auto-tracks', 'masonry-direction', 'masonry-fill', 'masonry-flow', 'masonry-slack', 'masonry-template-areas', 'masonry-template-tracks', 'position-anchor', 'position-area', 'position-try-fallbacks', 'position-visibility', 'scroll-start-target', 'text-box', 'view-transition-class', 'view-transition-group']
  ```

  This change replaces deprecated properties, improving CSS validation.

- LSP clients can now override the configuration path for each workspace, by responding to
  `workspace/configuration` requests.

- Added the new CSS rule [`noImportantStyles`](https://biomejs.dev/linter/rules/no-important-styles), which prevents the use of `!important` inside CSS declarations.

- Biome now emits a warning diagnostic if the configuration contains an out-of-sync schema URL.

- Introduced a new configuration setting `files.experimentalScannerIgnores`.

  This setting may be used to configure a set of file and folder names that should
  be unconditionally ignored by Biome's scanner.

  Biome maintains an internal list of default ignore entries, which is based on
  user feedback and which may change in any release. This setting allows
  overriding this internal list completely.

  This is considered an advanced feature that users _should_ not need to tweak
  themselves, but they can as a last resort. This setting can only be configured
  in root configurations, and is ignored in nested configs.

  Entries must be file or folder _names_. Specific paths and globs are not
  supported.

  Examples where this may be useful:

  ```jsonc
  {
    "files": {
      "experimentalScannerIgnores": [
        // You almost certainly don't want to scan your `.git` folder, which
        // is why it's already ignored by default:
        ".git",

        // But the scanner does scan `node_modules` by default. If you
        // *really* don't want this, you can ignore it like this:
        "node_modules",

        // But it's probably better to ignore a specific dependency.
        // For instance, one that happens to be particularly slow to scan:
        "RedisCommander.d.ts",
      ],
    },
  }
  ```

  Please be aware that rules relying on the module graph or type inference
  information may be negatively affected if dependencies of your project aren't
  (fully) scanned.

- Added the new rule [useSingleJsDocAsterisk](https://biomejs.dev/linter/rules/use-single-js-doc-asterisk/) which enforces JSDoc comment lines to start with a single asterisk.

  ```js
  // Invalid
  /**
   ** Description
   */

  // Valid
  /**
   * Description
   */
  ```

- The CLI flag `--javascript-attribute-position` was renamed to `--javascript-formatter-attribute-position` for consistency.

- Introduced the `domains` linter feature. The Biome linter now has a new way to opt-in rules, with a concept called `domains`.

  Domains can be seen as concepts shared by different rules.

  You can enable and disable multiple rules that belong to a domain. When you assign `"all"`, Biome will enable all the rules, when you assign `"none"`, Biome will disable the rules, when you assign "recommended", Biome will enable all rules of the domain that are recommended.

  ```json5
  // biome.jsonc
  {
    linter: {
      domains: {
        test: "all", // all rules that belong to this domain are enabled
        react: "recommended", // only the recommended rules from this domain are enabled
        solid: "none", // rules related to Solid are disabled
      },
    },
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

- Biome now prints diagnostics sorted by their severity. The order is the following:

  1. information
  2. warning
  3. error

  This means that _error_ diagnostics are printed **last**, so users can see them first.

- Added the new rule [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return), which enforces consistent return values in iterable callbacks.

  The following methods require a return value in their callback:

  - `every`
  - `filter`
  - `find`
  - `findIndex`
  - `findLast`
  - `findLastIndex`
  - `flatMap`
  - `map`
  - `reduce`
  - `reduceRight`
  - `some`
  - `sort`
  - `toSorted`
    — `from` (when called on `Array`)

  The rule disallows a return value inside the callback of the method `forEach`.

  Examples:

  ```js
  [].map(() => {
    // Missing return value
  });
  ```

  ```js
  [].forEach(() => {
    return 1; // Disallowed
  });
  ```

- Added the new rule [`noReactPropAssign`](https://biomejs.dev/linter/rules/no-react-prop-assign), based on the react-hooks rule [react-hooks/react-compiler](https://www.npmjs.com/package/eslint-plugin-react-hooks)

  The following code is now reported as invalid:

  ```jsx
  function Foo(props) {
    props.bar = `Hello ${props.bar}`;
    return <div>{props.bar}</div>;
  }
  ```

  The following code is now reported as valid:

  ```jsx
  function Foo({ bar }) {
    bar = `Hello ${bar}`;
    return <div>{bar}</div>;
  }
  ```

- Added new rule [`noBitwiseOperators`](https://biomejs.dev/linter/rules/no-bitwise-operators/), which disallows bitwise operators.

- The Biome analyzer now supports a new top-level suppression. These suppression have to be placed at the top of the file, and they must be followed by two newlines (`\n\n\`).

  The analyzer rules specified inside the block comment will be suppressed for the whole file.

  In the example, we suppress the rules `lint/style/useConst` and `lint/suspicious/noDebugger` for the whole file:

  ```js
  // main.js
  /**
   * biome-ignore-all lint/style/useConst: i like let
   * biome-ignore-all lint/suspicious/noDebugger: needed now
   */

  let path = "/path";
  let _tmp = undefined;
  debugger;
  ```

  In this other example, we suppress `lint/suspicious/noEmptyBlock` for a whole CSS file:

  ```css
  /**
  /* biome-ignore-all lint/suspicious/noEmptyBlock: it's fine to have empty blocks
  */

  a {
  }
  span {
  }
  ```

  A new diagnostic is emitted if `biome-ignore-all` suppression isn't placed at the top of the file:

  ```block
  file.js:3:1 suppressions/incorrect ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ! Top level suppressions can only be used at the beginning of the file.

      2 │ let foo = 2;
    > 3 │ /**
        │ ^^^
    > 4 │ * biome-ignore-all lint/style/useConst: reason
    > 5 │ */
        │ ^^
      6 │ let bar = 33;

    i Rename this to biome-ignore

      2 │ let foo = 2;
      3 │ /**
    > 4 │ * biome-ignore-all lint/style/useConst: reason
        │   ^^^^^^^^^^^^^^^^
      5 │ */
      6 │ let bar = 33;


  ```

- Added the new rule [`useNumericSeparators`](https://biomejs.dev/linter/rules/use-numeric-separators), which encourages the use of numeric separators to improve readability.

- [useImportExtensions](https://biomejs.dev/linter/rules/use-import-extensions/) now checks imports with sub extensions.

  ```js
  - import 'styles.css'
  + import 'styles.css.ts'
  ```

- It's possible to override the option `files.maxSize`. This option is helpful if you need to process specific files that exceed the default `maxSize`:

  ```json
  {
  	"overrides": [{
  		"includes": ["dist/**"]
  		"files": {
  			"maxSize": 20000
  		}
  	}]
  }
  ```

- Added the new CLI option called `--threads` to the `ci` command. It allows to control the numbers of threads that can be used when using the Biome CLI.

  It's possible to use the environment variable `BIOME_THREADS` as an alternatives.

  This feature is useful when running the CLI in environments that have limited resources, for example CI/CD.

  ```shell
  biome ci --threads=1
  BIOME_THREADS=1 biome ci
  ```

- Added the new rule [`useObjectSpread`](https://biomejs.dev/linter/rules/use-object-spread), which prefers object spread syntax over `Object.assign()` when constructing new objects.

  **Example (Invalid): Using Object.assign with an empty object:**

  ```js
  Object.assign({}, foo);
  Object.assign({}, { foo: "bar" });
  ```

  **Example (Invalid): Using Object.assign with object literal as first argument:**

  ```js
  Object.assign({ foo: "bar" }, baz);
  Object.assign({}, baz, { foo: "bar" });
  ```

  **Example (Valid): Using object spread syntax:**

  ```js
  ({ ...foo });
  ({ ...baz, foo: "bar" });
  ```

  **Example (Valid): Modifying existing objects is allowed:**

  ```js
  Object.assign(foo, { bar: baz });
  Object.assign(foo, bar, baz);
  ```

- Added an option to the `lint` command called `--suppress`. The new option suppresses a violation instead of applying a rule fix. The option accepts a string that is used as _reason_ of the suppression comment.

  When running the following command, it will add the suppression comment:

  ```shell
  biome lint --write --suppress="Migration to Biome"
  ```

  ```js
  debugger;
  foo == bar;
  ```

  ```diff
  + // biome-ignore lint/suspicious/noDebugger: Migration to Biome
  debugger;
  + // biome-ignore lint/suspicious/noDoubleEquals: Migration to Biome
  foo == bar;
  ```

- Add an `ignoreRestSiblings` option into [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables).

  When this option is set to `false`, the rule will **not** ignore variables that created using the rest pattern:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noUnusedVariables": {
            "level": "error",
            "options": {
              "ignoreRestSiblings": false
            }
          }
        }
      }
    }
  }
  ```

  ```js
  const { lorem, ...test } = bar; // the variable "test" will trigger the rule
  console.log(lorem);
  ```

- Upgraded some unsafe fixes to safe fixes.

  The following rules have now a safe fix:

  - [noExtraBooleanCast](https://biomejs.dev/linter/rules/no-extra-boolean-cast)
  - [noNonoctalDecimalEscape](https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape)
  - [noSwitchDeclarations](https://biomejs.dev/linter/rules/no-switch-declarations)
  - [noThisInStatic](https://biomejs.dev/linter/rules/no-this-in-static)
  - [noUnusedTemplateLiteral](https://biomejs.dev/linter/rules/no-unused-template-literal)
  - [noUselessContinue](https://biomejs.dev/linter/rules/no-useless-continue)
  - [noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
  - [noUselessStringConcat](https://biomejs.dev/linter/rules/no-useless-string-concat)
  - [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals)
  - [useExponentiationOperator](https://biomejs.dev/linter/rules/use-exponentiation-operator)
  - [useNumberToFixedDigitsArgument](https://biomejs.dev/linter/rules/use-number-to-fixed-digits-argument)
  - [useNumericLiterals](https://biomejs.dev/linter/rules/use-numeric-literals)
  - [useSimplifiedLogicExpression](https://biomejs.dev/linter/rules/use-simplified-logic-expression)

- Added support for monorepos. The feature will work _out of the box_ for the majority of the users. If your project
  has **nested configuration** files, use the command `biome migrate` from the _root of the project_.

  Monorepo support in Biome is done in a single way. Create a `biome.json` at the root of the project. This configuration
  file is now called the root configuration. Then, each nested configuration file must specify the new field `"root": false`.

  We also introduced a new microsyntax for _extending a nested configuration from the root configuration_, which is `"extends": "//"`. This new syntax means “this config _extends_ from the root config”. When using this microsyntax, you **may omit** the `"root": false` field as it is implied.

  Note that nested configs are not required to extend from the root config, and you can still have independent nested configs, as well as nested configs that extend from other files. In those cases, `"root": false` must be specified explicitly.

- Added support for formatting `.html` files. The formatting is considered **experimental,** and it's only opt-in via configuration:

  ```json
  {
    "html": {
      "formatter": {
        "enabled": true
      }
    }
  }
  ```

  Biome formatter attempts to format as Prettier, however some default options might differ.

  An option `html.formatter.selfCloseVoidElements` allows to control whether the trailing `/` of [void elements](https://html.spec.whatwg.org/#void-elements) should be printed.

  **By default**, Biome formatter will _remove_ the `/`:

  ```diff
  - <input />
  + <input>
  ```

  If you come from Prettier and you want to keep the same formatting behaviour, you should set the option to `"always"`:

  ```json
  {
    "html": {
      "formatter": {
        "selfCloseVoidElements": "always"
      }
    }
  }
  ```

  ```diff
  - <input>
  + <input />
  ```

  Use to the command `biome migrate prettier` to apply this change automatically.

- Added an **unsafe** fix to the rule [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies).

  For example, this violation will provide the following code fix:

  ```js
  import { useEffect } from "react";

  function MyComponent() {
    let a = 1;
    useEffect(() => {}, [a]);
  }
  ```

  ```
    × This hook specifies more dependencies than necessary: a

      3 │ function MyComponent() {
      4 │   let a = 1;
    > 5 │   useEffect(() => {}, [a]);
        │   ^^^^^^^^^
      6 │ }
      7 │

    i This dependency can be removed from the list.

      3 │ function MyComponent() {
      4 │   let a = 1;
    > 5 │   useEffect(() => {}, [a]);
        │                        ^
      6 │ }
      7 │

    i Unsafe fix: Remove the extra dependencies from the list.

      5 │ ··useEffect(()·=>·{},·[a]);
        │                        -
  ```

- The rule `useExhaustiveDependencies` now reports a diagnostic when the dependency list is not an array literal.

- Added the new rule [`useIndexOf`](https://biomejs.dev/linter/rules/use-index-of), based on the unicorn rule [prefer-array-index-of](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/prefer-array-index-of.md)

- Added a new rule [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/), which detects any missing cases for switch statements.
  Currently, it supports only literal union types.

  For example:

  ```ts
  type Day =
    | "Monday"
    | "Tuesday"
    | "Wednesday"
    | "Thursday"
    | "Friday"
    | "Saturday"
    | "Sunday";

  const day: Day = "Monday";
  let result = 0;

  switch (day) {
    case "Monday": {
      result = 1;
      break;
    }
  }
  ```

  The switch statement is missing other cases than `'Monday'`, which will cause a runtime error.
  To fix this issue, add missing cases or a default case to the statement.

- Fixed [#4416](https://github.com/biomejs/biome/pull/4416): The rules [`useExportType`](https://biomejs.dev/linter/rules/use-export-type/) and [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now ignore TypeScript declaration files.

- Added the new rule [`useForComponent`](https://biomejs.dev/linter/rules/use-for-component/).

  This rule enforces usage of Solid's `<For />` component for mapping an array to JSX elements.

- Added new lint rule [`noShadow`](http://biomejs.dev/linter/rules/no-shadow), a port of eslint's `no-shadow`.

  This rule disallows variable declarations from shadowing variables declared in an outer scope. For example:

  ```js
  const foo = 1;

  function bar() {
    const foo = 2; // This variable shadows the outer foo
  }
  ```

- Add `style` option for the [useImportType](https://biomejs.dev/linter/rules/use-import-type/) rule.

  The rule now allows enforcing an import style for importing types.
  See the rule documentation for more details.

- Added the new rule [`useJsonImportAttribute`](https://biomejs.dev/linter/rules/use-json-import-attribute) to enforce the use of import attributes for JSON modules.

  This rule ensures that all imports of `.json` files include the `with { type: "json" }` assertion, which is required to inform the JavaScript runtime that the imported file should be parsed as JSON.

  ```diff
  - import jsonData from './data.json';
  + import jsonData from './data.json' with { type: "json" };
  ```

  ```diff
  - import jsonData from './data.json' with { someOtherAttribute: "value" };
  + import jsonData from './data.json' with { type: "json", someOtherAttribute: "value" };
  ```

  This rule is based on the proposal in issue [#6043](https://github.com/biomejs/biome/issues/6043).

- [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) now handles numeric keys and is declared as being the same rule as the ESLint [no-useless-computed-key](https://eslint.org/docs/latest/rules/no-useless-computed-key) rule.

- `useNamingConmvention` now ignores unused variables prefixed with an underscore `_`.

  This avoids conflicts with the unsafe fix of `noUnusedVariables`.
  The following code is now accepted because the variable is unused and prefixed with an underscore.

  ```js
  const _Unknown_Style = 0;
  ```

- The package now requires `v2` of the WebAssembly packages. The internal APIs of Workspace are now `camelCase`.

- The rule [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now provides a code fix.

  ```diff
  - const xs = new Array();
  + const xs = [];
  ```

  The code fix is currently marked as unsafe.
  We plan to make it safe in a future release of Biome.

- The command `migrate` is now able to migrate nested configuration files.

- Added the new rule [`noRestrictedElements`](https://biomejs.dev/linter/rules/no-restricted-elements), which prevents use of the specified HTML elements and components.

- Added the new lint rule [`noAwaitInLoop`](https://biomejs.dev/linter/rules/no-await-in-loop).

### Patch Changes

- Fix [#5001](https://github.com/biomejs/biome/issues/5001), where the CSS formatter removes whitespace from selector preceded by a comment

- Fixed [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) rule to suggest unsafe fix for unused function declarations.

- Fixed a false positive of `noUselessEscapeInRegex` where `\k` was reported as useless in non-Unicode regular expressions.

- Fixed an issue where the ordering of the diagnostics wasn't predictable.

- Fixed a bug where the environment variable `BIOME_CONFIG_PATH` wasn't correctly picked up.

- Biome logs a warning in case a folder contains `biome.json` and `biome.jsonc`, and it will use `biome.json` by default.

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) is now able to bind read of value to a type-only import in ambient contexts ([#4526](https://github.com/biomejs/biome/issues/4526)).

  In the following code, `A` is now correctly bound to the type-only import.
  Previously, `A` was reported as an undeclared variable.

  ```ts
  import type { A } from "mod";

  declare class B extends A {}
  ```

- Fix [#4317](https://github.com/biomejs/biome/issues/4317), setter parameter can contain a trailing comma, the following example will now parsed correctly:

  ```ts
  export class DummyClass {
    set input(value: string) {}
  }
  ```

- Fix [#4575](https://github.com/biomejs/biome/issues/4575), don't wrap selector indentation after css comments.

- Fix [#4258](https://github.com/biomejs/biome/issues/4258), where fixed css parse error with @-moz-document url-prefix().

- Fixed [#4391](https://github.com/biomejs/biome/issues/4391): Some files from the `.vscode` directory are no longer incorrectly parsed as JSON.

- The `biome format` command now correctly handles the `--skip-errors` option, allowing it to skip files with syntax errors and continue formatting the remaining valid files.
  When this option is used, skipped syntax errors are reported as information, since the user is already aware of them.

- `biome migrate eslint` now correctly resolves the scoped package named `eslint-config`.

- Fixed [#3836](https://github.com/biomejs/biome/issues/3836): The CSS parser will now correctly parse the following:

  ```css
  .foo {
    color: red;
  }
  ```

- Fixed a bug where the related diagnostics attached to the main diagnostics didn't have a correct message.

- Fixed `noAccumulatingSpread` not reporting calls to `Object.assign`. The following code will now be reported:

  ```js
  let a = [{ a: 1 }, { b: 2 }];
  a.reduce((acc, val) => Object.assign(acc, val), []);
  ```

- The `summary` reporter doesn't take `--max-diagnostics` into account anymore.

- Fixed [#4553](https://github.com/biomejs/biome/issues/4553): `noUselessFragments` will now correctly fix JSX attributes:

  ```jsx
  <Suspense
    fallback={
      <>
        <span>Loading...</span>
      </>
    }
  >
    {children}
  </Suspense>
  ```

  becomes:

  ```jsx
  <Suspense fallback={<span>Loading...</span>}>{children}</Suspense>
  ```

- Fixed [#4528](https://github.com/biomejs/biome/issues/4528): `biome migrate eslint` now correctly handles shared ESLint configuration that don't follow the ESLint naming convention.

  ESLint recommends that a package that exports a shared configuration be prefixed with `eslint-config-` or simply named `eslint-config`.
  This is only a recommendation.
  Packages that export shared configurations can have arbitrary names.
  Biome is now able to load any package.

- Fixed [#4993](https://github.com/biomejs/biome/issues/4993): [`useAwait`](https://biomejs.dev/linter/rules/use-await/) now correctly warn on functions with decorator with callback argument.

- Fixed [#4756](https://github.com/biomejs/biome/issues/4756): `noDuplicateProperties` now throws lint errors properly when we use `@supports`.

- Fixed [#5981](https://github.com/biomejs/biome/issues/5981), where `noUnknownPseudoClass` didn't take `:global` into consideration when `cssModules` is enabled.

- Fixed [#2406](https://github.com/biomejs/biome/issues/2406): Biome longer expands properties of object type annotations in the only function parameter to align with Prettier.

- Fixed [#4740](https://github.com/biomejs/biome/issues/4740): `biome migrate eslint` now correctly handles ESLint configuration with `null` values in file lists.

- Fixed [#4202](https://github.com/biomejs/biome/issues/4202): Align with Prettier in formatting test functions.

- Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now properly handles unterminated string literals, such as:

  ```jsx
  function Comp() {
    return (
        <a rel="
  ```

- Fixed a bug where syntax rules didn't provide an automatic way to suppress the rule. Now the LSP will show supression actions if a syntax rule is violated.

- Fixed a CSS parser error: `@-moz-document url-prefix(https://example.com)` and `@-moz-document domain(example.com)` are now valid.

- Fixed [#4967](https://github.com/biomejs/biome/issues/4967): The fix for `useArrowFunction` no longer breaks function bodies starting with `{`.

- Fixed [#5998](https://github.com/biomejs/biome/issues/5998). The rule `noUnknownPseudoElement` now correctly cheks names
  of pseudo-element functions.

- Fixed [#5024](https://github.com/biomejs/biome/issues/5024): Added `useJsxKeyInIterable` rule to React domain.

- Fixed [#5410](https://github.com/biomejs/biome/issues/5410). Biome now correctly parse an `.editorconfig` that includes character classes in glob patterns.

- Fixed [#2260](https://github.com/biomejs/biome/2260): The LSP server now returns correct text edits for the specified range in `textDocument/rangeFormatting` and `textDocument/onTypeFormatting` requests.

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports top-level variables in a global declaration file as unused.

- Type exports now support renaming types to `default`.

  The following code is now parsed successfully:

  ```ts
  export { type A as default } from "./b.ts";
  ```

- Added proper support for arrow functions in the lint rule https://biomejs.dev/linter/rules/use-explicit-type/

- The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer suggests renaming top-level variables in a global declaration file.

- Improved context in error messages when migrating Prettier configurations

- Allowed single spaces in `useConsistentCurlyBraces` rule.

- Fixed [#4413](https://github.com/biomejs/biome/issues/4413): The GraphQL formatter no longer adds a new line at the start of block comments on Windows.

- Fixed [#5407](https://github.com/biomejs/biome/issues/5407). Now the `noUnusedImports` code fix correctly keeps top-level comments that were attached to lone imports.

- Fixed [#3859](https://github.com/biomejs/biome/issues/3859): the `--skip-parse-errors` option is now applied to commands: `lint`, `check`, and `ci`.

- The `rage` command now prints the configuration path relative to the working directory, if applicable.

- Fixed [#5606](https://github.com/biomejs/biome/issues/5606): We now correctly
  handle `.mjs` extensions in Node.js packages with `"type": "commonjs"`.

- Fixed [#1597](https://github.com/biomejs/biome/issues/1597): `useExhaustiveDependencies` no longer gets confused about the stability of dependencies by parentheses or type assertions.

- Fixed [#4751](https://github.com/biomejs/biome/issues/4751) by checking fragments inside `JSXElement` and conditional expressions.

  For example, the following two cases will now be reported:

  ```jsx
  <section>
    <>
      <div />
      <div />
    </>
  </section>
  ```

  ```jsx
  showFullName ? <>{fullName}</> : <>{firstName}</>;
  ```

- The rule `noFallthroughSwitchCase` no longer panics on some incomplete code snippets.

- Fixed [#5007](https://github.com/biomejs/biome/issues/5007): Resolved false positives in `noMissingVarFunction` for `container-name`.

- Fixed [#4841](https://github.com/biomejs/biome/issues/4841): Shebang and top leading comments in `.cjs` files are now handled correctly

  **Example: shebang only (keep it as is)**

  ```
  #!/usr/bin/env node
  ```

  **Example: comments only (keep it as is)**

  ```
  // comment
  ```

  **Example: with shebang**

  ```diff
  - #!/usr/bin/env node"use strict";
  + #!/usr/bin/env node
  + "use strict";
  let some_variable = "some value";
  ```

  **Example: with comment**

  ```diff
  - // comment
  - "use strict"; // comment
  + "use strict";
  + // comment
  let some_variable = "some value";
  ```

  **Example: with shebang and comment**

  ```diff
  - #!/usr/bin/env node"use strict";
  - // comment
  + #!/usr/bin/env node
  + "use strict";
  + // comment
  let some_variable = "some value";
  ```

- Fixes [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) missing dependencies being defined after the hook itself failure.

  Example:

  ```jsx
  import { useState, useEffect } from "react";

  function MyComponent() {
    useEffect(() => {
      console.log(a);
    }, []);

    let a = 1;
  }
  ```

- Fixed [#4714](https://github.com/biomejs/biome/pull/4714): Suppression comments no longer fail on functions that themselves contain suppression comments.

  This now works correctly:

  ```ts
  // biome-ignore lint/complexity/useArrowFunction: this suppression now works
  const foo0 = function (bar: string) {
    // biome-ignore lint/style/noParameterAssign: even if there are other suppressions inside
    bar = "baz";
  };
  ```

- Add @vitest/eslint-plugin to list of Biome rule sources

- Fixed `useHookAtTopLevel` rule to properly detect React components wrapped in `memo` and `forwardRef`, and correctly handle property accessors in control flow analysis.

  The rule now correctly identifies hooks in components like:

  ```js
  const TestMemo = memo(
    forwardRef((props, ref) => {
      useEffect(() => {
        const [test, setTest] = useState(1); // now properly flagged
      }, []);
      return <div ref={ref}>test</div>;
    }),
  );
  ```

  And properly handles property accessors:

  ```js
  function ReactComponent() {
    const testObj = {
      get print() {
        return "hello"; // no longer considered component return
      },
    };
    const callback = useCallback(() => {}, []);
    return <></>;
  }
  ```

- [noMisleadingCharacterClass](https://biomejs.dev/linter/rules/no-misleading-character-class/) no longer panics on malformed escape sequences that end with a multi-byte character ([#4587](https://github.com/biomejs/biome/issues/4587)).

- Fixed the flag `--bracket-spacing` that was duplicated between the global configuration and the language-specific override for JavaScript.

- Fixed [#4715](https://github.com/biomejs/biome/issues/4715): The `useJsxKeyInIterable` rule now reports missing keys inside `switch` and `if` statements.

  ```jsx
  const data = [
    { value: "a", type: "string" },
    { value: 9, type: "number" },
    { value: "c", type: "string" },
  ];

  const MyComponent = () => {
    return (
      <>
        {/* if statements */}
        {data.map((x) => {
          if (x.type === "string") {
            return <div>{x.value}</div>; // no key, emits diagnostic
          } else {
            return <div>{x.value}</div>; // no key, emits diagnostic
          }
        })}

        {/* switch statements */}
        {data.map((x) => {
          switch (x.type) {
            case "string":
              return <div>{x.value}</div>; // no key, emits diagnostic
            case "number":
              return <div>{x.value}</div>; // no key, emits diagnostic
            default:
              return <div key={x.value}>{x.value}</div>;
          }
        })}
      </>
    );
  };
  ```

- Fixed [#4121](https://github.com/biomejs/biome/issues/4326): The CSS formatter no longer indents a selector when it has leading comments.

- Fixed an issue where react lint rules could panic Biome when some incorrect code was analyzed.

- Fixed [#4982](https://github.com/biomejs/biome/issues/4982): the JavaScript parser now throws a syntax error for the following code:

  ```ts
  type T = import;
  type U = typeof import;
  ```

- Fixed a bug with the `--verbose` CLI flag. Now the printed paths are **relative** to the working directory.

- Fixed [`noNoninteractiveElementToInteractiveRole`](https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role/) mistakenly flagging `<li role="treeitem">`,

- Fixed [#4622](https://github.com/biomejs/biome/issues/4622): Our JavaScript parser can now gracefully handle situations where we detect the parser to have stalled.

  This means we don't fail with an assertion anymore, but invalid code can trigger a regular diagnostic in such cases.

- Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now correctly handles invalid object member names, such as:

  ```js
  ({
    params: { [paramName: string]: number } = {}
  })
  ```

- Fixed [#6211](https://github.com/biomejs/biome/issues/6211): previously the
  import organizer emitted broken code when it merged an import at the start of
  the file with another import and placed the merged result after a third import.

  The following code is now correctly organized:

  ```diff
  - import { B } from "bc";
  - import { C } from "bc";
    import { A } from "a";
  + import { B, C } from "bc";
  ```

- Fixed [#4334](https://github.com/biomejs/biome/issues/4334): The formatter no longer inserts trailing a comma inside dynamic `import` expressions.

- Fixed [#5629](https://github.com/biomejs/biome/issues/5629): useHookAtTopLevel no longer report false-positives where the hook is at the top-level in a class method.

- Fixed [#5900](https://github.com/biomejs/biome/issues/5900): `biome migrate eslint` now support a nested `files` property in ESLint flat configs.

- Fixed [#3895](https://github.com/biomejs/biome/issues/3895): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) no longer reports used values imported as types in an external module.

- Fixed a case where the code fix for `noUselessFragments` would remove more than just the fragment.

- Fixed [#5919](https://github.com/biomejs/biome/issues/5919). Now Biome correctly loads the configuration passed via `--config-path` when its path starts with `./` e.g. `--confi-path=./project/biome.json`

- Fixed [#5031](https://github.com/biomejs/biome/issues/5031): CSS formatting has been improved for numbers:

  ```diff
  .class {
  -	padding: .5em;
  -	marding: 1.0;
  +	padding: 0.5em;
  +	marding: 1;
  }
  ```

- Fixed [#5989](https://github.com/biomejs/biome/issues/5989) where large octal escape sequences led to an overflow.

- Implement improved error handling for the supports at rule

- Fix [#5053](https://github.com/biomejs/biome/issues/5053), now the rule correctly handles `console.log` inside arrow function expressions.

- Fix [#6105](https://github.com/biomejs/biome/issues/6105): css lint rules `useSortedProperties` should skip unknown properties.

- Fixed [#3229](https://github.com/biomejs/biome/issues/3229): Made formatting of compound selectors more consistent.

- Fixed a bug where passing `--max-diagnostics=0` would return a zero code even when errors were emitted.

- Fixed a bug where Biome didn't report any error when `--stdin-file-path` didn't have any extension.
  Now Biome returns an error if `--stdin-file-path` doesn't have an extension.

- Fixed [#5601](https://github.com/biomejs/biome/issues/5601): The [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) rule now properly preserves the original JSX quote style when sorting utility classes, preventing syntax errors.

- The fix for `useSelfClosingElements` was marked as safe and the error message was improved.

- Fixed overrides that include language-specific settings from having an effect for some languages

- Fixed [#6144](https://github.com/biomejs/biome/issues/6144): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) reported incorrectly imports that were used as the type of parameters with the same name.
  In the following code, the import `name` was reported as unused.

  ```ts
  import name from "mod";
  function f(name: name.Readable): void {}
  ```

- The lint rules [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) and [`useFilenamingConvention`](https://biomejs.dev/linter/rules/use-filenaming-convention/) now accept character escapes at the start of a regex group.

  Both these rules provide options that allow matching names against a regular expression.
  Previously, an escaped character at the start of a regex group reported an error. They are now accepted.

  For example, the following configuration is now valid doesn't emit an error anymore.

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "useNamingConvention": {
            "level": "on",
            "options": {
              "conventions": [
                {
                  "selector": {
                    "kind": "let"
                  },
                  "match": "(\\n.*)"
                }
              ]
            }
          }
        }
      }
    }
  }
  ```

- Fixed [#5617](https://github.com/biomejs/biome/issues/5617): [noDuplicateObjectKeys](https://biomejs.dev/linter/rules/no-duplicate-object-keys/) now transfers the leading comments of the removed member.

- Fixed [#5409](https://github.com/biomejs/biome/issues/5409): [noParameterAssign](https://biomejs.dev/linter/rules/no-parameter-assign) now reports reassigned parameter of unparenthesized arrow functions.

  The following code is now reported as invalid.

  ```js
  const f = (param) => {
    param = {}; // Reassigning a function parameter is confusing.
  };
  ```

- Fixed [#4875](https://github.com/biomejs/biome/issues/4875): Relative file paths are now clickable in the Jetbrains IDE terminal.

- Fixed [#4719](https://github.com/biomejs/biome/issues/4719): `bracketSameLine` now performs as expected when a comment is placed before the last JSX attribute.

- Fixed [#4564](https://github.com/biomejs/biome/issues/4564): Biome no longer panics when a multi-byte character is found in a unicode escape sequence.

- Fixed [#4950](https://github.com/biomejs/biome/issues/4950): Resolved a false positive of character class range operators in regular expressions.

- Fixed handling of top-level variables by `useExplicitType` rule ([#5932](https://github.com/biomejs/biome/issues/5932)). Biome now allows all variables with explicit annotations, as well as variables with trivial RHS. Biome no longer emits duplicated errors when an untyped function is assigned to an untyped variable.

- Fixed [#4947](https://github.com/biomejs/biome/issues/4947): The `useTemplate` lint rule now ignores concatenated literals folded to multiple lines.

- Fixed [#4568](https://github.com/biomejs/biome/issues/4568): Broken import statements no longer can cause a panic in `useExhaustiveDependencies`.

- Fixed [#6042](https://github.com/biomejs/biome/pull/6042): [`noUselessEscapeInString`](https://next.biomejs.dev/linter/rules/no-useless-escape-in-string/) now reports useless escapes after skipping \${ in template literals.

- Fixed [#6229](https://github.com/biomejs/biome/issues/6229) where the fix of `noUnusedImports` emitted an invalid syntax. Now the following case emits a code fix that is syntactically correct:

  ```js
  import Used, { NotUsed } from "foo";

  Used();
  ```

- Fix [#5682](https://github.com/biomejs/biome/issues/5682): Object patterns with a nested assignment pattern no longer break properties.

  For example, the following code:

  ```js
  const { foo: { bar } = { bar: false } } = props;
  ```

  is used to be formatted into:

  ```js
  const { foo: { bar } = { bar: false } } = props;
  ```

  , while Prettier does not expand properties in this case.

- Fixed #5620, [noConsole](https://biomejs.dev/linter/rules/no-console/) rule now correctly handles indirect `console.log` calls and references.

- When pulling code actions from the LSP, now the first choice suggested by the client will be the safe fix.

- Fixed [#6022](https://github.com/biomejs/biome/issues/6022), now the rule `noDuplicateProperties` does't trigger properties defined inside the `@keyframes` at rule

- Enhanced the error message of the diagnostics emitted when Biome can't parse a suppression comment.

- Fixed link to the docs inside CLI markup

- Fixed a bug where a suppression comment with an empty explanation was valid.

  Now a suppression comment `// biome-ignore lint:` will raise a **warning** diagnostic.

- Fixed [#4026](https://github.com/biomejs/biome/issues/4026): Comments in `grid-template` are no longer moved by the formatter.

- Fixed [#3394](https://github.com/biomejs/biome/issues/3394): Resolved a false positive in `useSortedClasses`.

- Fixed [#342](https://github.com/biomejs/biome/issues/342) and [#4562](https://github.com/biomejs/biome/issues/4562): Biome no longer crashes when a `declare` statement is followed by an unexpected token.

- Fixed false positive in the rule [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function) where the [`tech`](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face/src#tech) function was incorrectly flagged as an unknown function.

- Fixed [#4511](https://github.com/biomejs/biome/issues/4511): [noLabelWithoutControl](https://biomejs.dev/linter/rules/no-label-without-control/) now detects `<button>` tags as input.

- Fixed [#6039](https://github.com/biomejs/biome/issues/6039): [`noUselessEscapeInString`](https://next.biomejs.dev/linter/rules/no-useless-escape-in-string/) no longer reports `\${` escape in template literals.

- Fixed [#5985](https://github.com/biomejs/biome/issues/5985), which caused the import organizer to fail the merging of a default import with a named import.
  The following code is now correctly organized:

  ```diff
  - import moment from 'moment';
  - import { Moment } from 'moment';
  + import moment, { Moment } from 'moment';
  ```

- Fixed an issue where the `explain` command didn't the diagnostic category when a rule was explained.

- Improved the diagnostic of the rule `noUnusedVariables`. The rule message now provides the name of the unused binding.

- Added `RegExpStringIterator` to the analyzer globals.

- Fixed [#4208](https://github.com/biomejs/biome/issues/4208): [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) now handles `JsxAttributeInitializerClause`, ensuring that fragments inside expressions like `<A b=<></> />` are preserved.

- Fixed [#4533](https://github.com/biomejs/biome/issues/4533): `noUnknownPseudoClass` no longer reports pseudo classes after a webkit scrollbar pseudo element.

  The following code will no longer report a diagnostic:

  ```css
  ::-webkit-scrollbar-thumb:hover {
  }
  ```

- Updates the [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/) rule to more closely match the behavior of the ESLint plugin (e.g. mark the whole fragment as incorrect when no key is present). This also adds the option to check shorthand fragments (`<></>`)

- Renamed the rule `noDuplicatedFields` to `noDuplicateFields`. Run the command `biome migrate` to update your configuration.

- Fixed an issue where ignored files were incorrectly tracked by the Daemon.

- Fixed [#5116](https://github.com/biomejs/biome/issues/5116): [noUnknownPseudoElement](https://biomejs.dev/linter/rules/no-unknown-pseudo-element/) now supports `::slotted`.

- Fixed [#5979](https://github.com/biomejs/biome/issues/5979): `biome search` now correctly skips files that don't match the pattern's target language.

- Fixed [#4323](https://github.com/biomejs/biome/issues/4258): Fixed the case where `useSemanticElement` accidentally showed recommendations for `role="searchbox"` instead of `role="search"`.

- Support setting `indent_size` to `tab` in `.editorconfig`, the following config will not cause error:

  ```editorconfig
  root = true
  [*]
  indent_size = tab
  ```

- Fixed [#4565](https://github.com/biomejs/biome/issues/4565): [noControlCharactersInRegex](https://biomejs.dev/linter/rules/no-control-characters-in-regex) no longer panics when it encounters an unterminated unicode escape sequence.

- Fixed [#5770](https://github.com/biomejs/biome/issues/5770), Biome's configuration file is now respected by the `migrate` command during migration

- Fixed an issue where the lexer didn't report errors for unterminated regex or string literals, such as the following cases:

  ```js
  "string
  'str
  /\\217483
  ```

- The [`useKeyWithClickEvents`](https://biomejs.dev/linter/rules/use-key-with-click-events/) rule has been improved with better support for ARIA roles.

  Key improvements:

  1. **Accessibility checks**:

  Now the rule correctly handles the following cases:

  - If an element is hidden from screen readers
  - If an element has the presentation role
  - If an element is interactive

  ```jsx
  // No errors
  <div aria-hidden="true" onClick={() => {}} /> // hidden from screen reader
  <div role="presentation" onClick={() => {}} /> // presentation role
  <button onClick={() => {}} /> // interactive role
  ```

  This change ensures the rule is more accurate and helpful.

  2. **Checks spread syntax**:

  Spread syntax used to be ignored, but has been changed to be pointed out for more stringent checking.

  ```jsx
  // Errors
  <div {...props} onClick={() => {}} />
  // No errors
  <div {...props} onClick={() => {}} onKeyDown={foo} />;
  ```

  3. **Refactor**:

  Now the rule uses the aria roles to determine if an element is interactive.

  The changes shown here are meant to be closer to the original [jsx-eslint's `click-events-have-key-events` rule](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/click-events-have-key-events.md).

- Fixed [#6029](https://github.com/biomejs/biome/issues/6029): A new line before the semicolon in the previous statement is now kept after formatting.

  For example, the following code:

  ```js
  const foo = 3;

  [1, 2, 3].map((x) => x * 2);
  ```

  when `javascript.formatter.semicolons` is `always`, it becomes:

  ```js
  const foo = 3;

  [1, 2, 3].map((x) => x * 2);
  ```

  when `javascript.formatter.semicolons` is `asNeeded`, the original code is considered as already formatted.

- [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now reports all expressions using the `Array` constructors.

  Previously, the rule reported only use of the `Array` constructor in expressions statements.

  ```js
  // This was reported
  new Array();
  // This was not reported
  const xs = new Array();
  ```

- Improved error handling for the container at-rule.

- Fixed [#4665](https://github.com/biomejs/biome/issues/4665): the LSP previously
  identified `.cjs` files as ESM files, making rules like `noRedundantUseStrict`
  reports incorrectly valid `"use strict"` directives.

- Fixed [#5382](https://github.com/biomejs/biome/issues/5382): `useExportType` no longer reports an identifier that bound by both a variable and a type.

- Fixed [#5826](https://github.com/biomejs/biome/issues/5826): [`useNumericSeparators`](https://next.biomejs.dev/linter/rules/use-numeric-separators/) no longer reports single-digit `0`.

- Fixed [#5307](https://github.com/biomejs/biome/issues/5307), where CSS value lists were wrapped in a way that did not preserve semantic structure.

  Biome now ensures that CSS value lists follow a more readable format, aligning with Prettier's behavior.

  Before:

  ```css
  * {
    box-shadow:
      0 0 0 1px #fff,
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px red,
      0 0 0 3.2px rgba(89, 89, 235, 0.25);
  }
  ```

  After:

  ```css
  * {
    box-shadow:
      0 0 0 1px #fff,
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px rgba(89, 89, 235, 0.25),
      0 0 0 3.2px red,
      0 0 0 3.2px rgba(89, 89, 235, 0.25);
  }
  ```

- `tsconfig.*.json` files will now be treated the same as `tsconfig.json` files.

- The `summary` reporter now prints the files processed and the files fixed when passing the `--verbose` flag.

- Fixed [#5693](https://github.com/biomejs/biome/issues/5693): [`useRegexLiterals`](https://biomejs.dev/linter/rules/use-regex-literals/) now correctly handle useless escaped character in string literals.

- [useRegexLiterals](https://biomejs.dev/linter/rules/use-regex-literals) now suggests a correct fix when the pattern contains an escaped anti-slash `\/`.

  Previously the rule suggested the following fix that led to a syntax error:

  ```diff
  - new RegExp("\/");
  + /\\//
  ```

  The rule now suggests a correct fix:

  ```diff
  - new RegExp("\/");
  + /\//
  ```

  Fixed [#5487](https://github.com/biomejs/biome/issues/5487).

- Fixed [`useConsistentCurlyBraces breaks react/no-unescaped-entities rule`](https://github.com/biomejs/biome/issues/5391)

  Added a check for forbidden characters: `>`, `"`, `'` and `}`.
  If any of these characters are detected, curly braces will be preserved.

  Example:

  ```jsx
  function MyComponent() {
    return <Foo>Jupiter {">"} Venus</Foo>;
  }
  ```

- The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now suggests a rename that preserves uppercase if possible.

  For instance, Biome suggested renaming `HTMLWrapper` as `htmlWrapper`:

  ```diff
  - import HTMLWrapper from "HTMLWrapper.tsx";
  + import htmlWrapper from "HTMLWrapper.tsx";

    function component() {
  -   return <HTMLWrapper> </HTMLWrapper>;
  +   return <htmlWrapper> </HTMLWrapper>;
    }
  ```

  Since both `PascalCase` and `CamelCase` are accepted, Biome now suggests renaming `HTMLWrapper` as `HtmlWrapper`:

  ```diff
  - import HTMLWrapper from "HTMLWrapper.tsx";
  + import HtmlWrapper from "HTMLWrapper.tsx";

    function component() {
  -   return <HTMLWrapper> </HTMLWrapper>;
  +   return <HtmlWrapper> </HTMLWrapper>;
    }
  ```

- Fix a parsing error when a `JsxElementName` is `JsxMemberExpression`, and a `JsLogicalExpreesion` before it without a semicolon.

  The following case will now not throw error:

  ```jsx
  import React from "react";

  let b = 0;

  function A() {
    const a = b > 0 && b < 1;

    return <React.Fragment>{a}</React.Fragment>;
  }
  ```

- Fixed Biome being unable to parse `insert_final_newline = unset` in EditorConfig files.

- Fixed [#4530](https://github.com/biomejs/biome/issues/4530): [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/) now preserves directives.

  Previously the rule removed the directives when a function expression was turned into an arrow function.
  The rule now correctly keeps the directives.

  ```diff
  - const withDirective = function () {
  + const withDirective = () => {
      "use server";
      return 0;
    }
  ```

- Fixed [#4855](https://github.com/biomejs/biome/issues/4855): [useSortedClasses](https://biomejs.dev/linter/rules/use-sorted-classes/) now suggests code fixes that match the JSX quote style of the formatter.
