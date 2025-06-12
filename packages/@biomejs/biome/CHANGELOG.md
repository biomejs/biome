# @biomejs/biome

## 2.0.0

### Major Changes

- [#4823](https://github.com/biomejs/biome/pull/4823) [`d3b7b2d`](https://github.com/biomejs/biome/commit/d3b7b2d734a3cb40b7e75c65d9e04b1a7f30f2fb) Thanks [@ematipico](https://github.com/ematipico)! - Biome now resolves globs and paths from the configuration. Before, paths and globs were resolved from the working directory.

- [#5776](https://github.com/biomejs/biome/pull/5776) [`4874007`](https://github.com/biomejs/biome/commit/4874007c15e88a1a9c0dd298db5a8f8a8b568d34) Thanks [@ematipico](https://github.com/ematipico)! - Biome now raises a **warning** diagnostic for suppression comments that have `<explanation>` as reason.

  `<explanation>` is provided as a placeholder when applying the suppression code fix from LSP editors.

- [#5235](https://github.com/biomejs/biome/pull/5235) [`7037c0f`](https://github.com/biomejs/biome/commit/7037c0f56491676709face0d13b791fba2f818ec) Thanks [@siketyan](https://github.com/siketyan)! - Removed the `--config-path` argument from the `biome lsp-proxy` and `biome start` commands.

  The option was overriding the configuration path for all workspaces opened in the Biome daemon, which led to a configuration mismatch problem when multiple projects are opened in some editors or IDEs.

  If you are using one of our official plugins for IDEs or editors, it is recommended to update it to the latest version of the plugin, or you will get unexpected behavior.

  If you are a developer of a plugin, please update your plugin to use the `workspace/configuration` response instead of using the `--config-path` argument. Biome's LSP will resolve a configuration in the workspace automatically, so it is recommended to keep it empty unless you are using a custom configuration path.

- [#6146](https://github.com/biomejs/biome/pull/6146) [`9d37b53`](https://github.com/biomejs/biome/commit/9d37b5321679496294b1779a0f7b19f05c5b5dfe) Thanks [@ethanniser](https://github.com/ethanniser)! - Downgraded some code fixes to unsafe which were previously safe.

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

- [#5226](https://github.com/biomejs/biome/pull/5226) [`983ab6f`](https://github.com/biomejs/biome/commit/983ab6f0792af23a7d6abe68b262ea1423759635) Thanks [@Conaclos](https://github.com/Conaclos)! - Previously the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` checked both regular expression literals like `/regex/` and dynamically built regular expressions like `new RegExp("regex")`.

  Checking dynamically built regular expressions has many limitations, edge cases, and complexities.
  In addition, other rules that lint regular expressions don't check dynamically built regular expressions.

  Rather than add support for other rules and have half-baked checking, we decided to remove support for dynamically built regular expressions.

  Now the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` only check literals of regular expressions.

- [#5714](https://github.com/biomejs/biome/pull/5714) [`84380cb`](https://github.com/biomejs/biome/commit/84380cba8b68126fffc1dfe0889333a0c0c94a7f) Thanks [@siketyan](https://github.com/siketyan)! - The lint rule [`noRestrictedGlobals`](https://biomejs.dev/linter/rules/no-restricted-globals/) now supports customizing message for each global name.

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

- [#4899](https://github.com/biomejs/biome/pull/4899) [`c047886`](https://github.com/biomejs/biome/commit/c04788665ee45cd4416f4f8ee88fe865ba3b1791) Thanks [@Conaclos](https://github.com/Conaclos)! - Removed `include` and `ignore` fields in favor of the new field `includes`.

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

- [#5414](https://github.com/biomejs/biome/pull/5414) [`e7b712e`](https://github.com/biomejs/biome/commit/e7b712eae87e62f28e296b5170395dc5ce4ec909) Thanks [@Conaclos](https://github.com/Conaclos)! - `noUndeclaredVariables` no longer reports TypeScript types.

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

- [#5406](https://github.com/biomejs/biome/pull/5406) [`c8a863a`](https://github.com/biomejs/biome/commit/c8a863ad800ab5a704f1b0b3b59326127be71036) Thanks [@ematipico](https://github.com/ematipico)! - The rule `noUnusedVariables` no longer reports unused function parameters. Use [`noUnusedFunctionParameters`](https://biomejs.dev/linter/rules/no-unused-function-parameters/).

- [#5566](https://github.com/biomejs/biome/pull/5566) [`c3b5f87`](https://github.com/biomejs/biome/commit/c3b5f873ceb164210c28fdac128b94dab6018364) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#5564](https://github.com/biomejs/biome/issues/5564). `noTypeOnlyImportAttributes` now ignores files ending with the extension `.ts` when the type field of `package.json` is set to `commonjs`.

- [#4803](https://github.com/biomejs/biome/pull/4803) [`f86999d`](https://github.com/biomejs/biome/commit/f86999d6a0a52c78e6f09a19bf353253bc1bbe72) Thanks [@ematipico](https://github.com/ematipico)! - The Biome formatter no longer adds a trailing comma in `.json` files, even when `json.formatter.trailingCommas` is set to `true`.

- [#5228](https://github.com/biomejs/biome/pull/5228) [`344a131`](https://github.com/biomejs/biome/commit/344a1316069eac6361bb6bd1715f33575c387e32) Thanks [@Conaclos](https://github.com/Conaclos)! - [Prettier 3.4](https://prettier.io/blog/2024/11/26/3.4.0.html) introduced a change in their normalization process of string literals: it no longer unescapes useless escape sequences.
  Biome now matches the new behavior of Prettier when formatting code.
  This affects the JSON and JavaScript formatters.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`72ef826`](https://github.com/biomejs/biome/commit/72ef8260135e9d15fe34429846a1b81e940c7efe) Thanks [@ematipico](https://github.com/ematipico)! - Reduced accepted values for formatter options:

  - The option `--quote-style` doesn't accept `Single` and `Double` anymore.
  - The option `--quote-properties` doesn't accept `AsNeeded` and `Preserve` anymore.
  - The option `--semicolons` doesn't accept `AsNeeded` and `Always` anymore.
  - The option `--arrow-parenthesis` doesn't accept `AsNeeded` and `Always` anymore.
  - The option `--trailing-commas` doesn't accept `ES5`, `All` and `None` anymore.
  - The option `--attribute-position` doesn't accept `Single` and `Multiline` anymore.

- [#5178](https://github.com/biomejs/biome/pull/5178) [`882aca8`](https://github.com/biomejs/biome/commit/882aca8f5e6ed923c180e99fa04ae4384d88402b) Thanks [@Conaclos](https://github.com/Conaclos)! - Removed the option `enumMemberCase` from the lint rule `useNamingConvention`.

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

- [#4760](https://github.com/biomejs/biome/pull/4760) [`17ff3f6`](https://github.com/biomejs/biome/commit/17ff3f63e74e5916a102efd1c709a68e98383487) Thanks [@ematipico](https://github.com/ematipico)! - Removed support for `BIOME_LOG_DIR`.

  The environment variable `BIOME_LOG_DIR` isn't supported anymore.

  Use `BIOME_LOG_PATH` instead.

- [#4766](https://github.com/biomejs/biome/pull/4766) [`1907096`](https://github.com/biomejs/biome/commit/190709672495d3adda58473ad73e411b3273c02a) Thanks [@ematipico](https://github.com/ematipico)! - Remove deprecated rules.

  The following _deprecated_ rules have been deleted:

  - `noInvalidNewBuiltin`
  - `noNewSymbol`
  - `useShorthandArrayType`
  - `useSingleCaseStatement`
  - `noConsoleLog`

  Run the command `biome migrate --write` to update the configuration.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`1be9494`](https://github.com/biomejs/biome/commit/1be949465b1127712ec1dc89771c265bfb3b4631) Thanks [@ematipico](https://github.com/ematipico)! - Removed the deprecated `indentSize` option.

  The deprecated option `indentSize`, and its relative CLI options, has been removed:

  - Configuration file: `formatter.indentSize`
  - Configuration file: `javascript.formatter.indentSize`
  - Configuration file: `json.formatter.indentSize`
  - CLI option `--indent-size`
  - CLI option `--javascript-formatter-indent-size`
  - CLI option `--json-formatter-indent-size`

  Use `indentWidth` and its relative CLI options instead.

- [#4853](https://github.com/biomejs/biome/pull/4853) [`dac3882`](https://github.com/biomejs/biome/commit/dac388281e06e7fc33134abc088bd1ac968e8e2e) Thanks [@SuperchupuDev](https://github.com/SuperchupuDev)! - Removed `ROME_BINARY`. Use `BIOME_BINARY` instead.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`0680ba5`](https://github.com/biomejs/biome/commit/0680ba51765fbb3d6334008471485f9ed54791d3) Thanks [@ematipico](https://github.com/ematipico)! - Removed support for legacy suppressions.

  Biome used to support "legacy suppressions" that looked like this:

  ```js
  // biome-ignore lint(complexity/useWhile): reason
  ```

  This format is no longer supported.

- [#4894](https://github.com/biomejs/biome/pull/4894) [`d43aa7e`](https://github.com/biomejs/biome/commit/d43aa7efa7ef2d91c06e103b93beac54dcedd0e4) Thanks [@ematipico](https://github.com/ematipico)! - Removed support for `max_line_length` from `.editorconfig`, as it isn't part of the official spec anymore.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`36b4b1c`](https://github.com/biomejs/biome/commit/36b4b1cfbe483ecef0c32d06c87aeb75c0cae0f4) Thanks [@ematipico](https://github.com/ematipico)! - Removed support for `rome-ignore` suppression comments.

  Use `biome-ignore` suppression comments instead.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`36b4b1c`](https://github.com/biomejs/biome/commit/36b4b1cfbe483ecef0c32d06c87aeb75c0cae0f4) Thanks [@ematipico](https://github.com/ematipico)! - Removed support for `rome.json`.

  Use `biome.json` or `biome.jsonc` instead.

- [#4664](https://github.com/biomejs/biome/pull/4664) [`55acffe`](https://github.com/biomejs/biome/commit/55acffeb18256763f5bf6fbb20f2bafd23e4765f) Thanks [@ematipico](https://github.com/ematipico)! - Removed the option `all` from the linter.

  The options `linter.rules.all` and `linter.rules.<group>.all` has been removed.

  The number of rules in Biome have increased in scope and use cases, and sometimes some of them can conflict with each other.

  The option was useful at the beginning, but now it's deemed harmful, because it can unexpected behaviours in users projects.

  To automatically remove it, run the following command:

  ```shell
  biome migrate --write
  ```

- [#4760](https://github.com/biomejs/biome/pull/4760) [`1accca5`](https://github.com/biomejs/biome/commit/1accca58a651398df0ca1719d32c4d537459c0eb) Thanks [@ematipico](https://github.com/ematipico)! - Removed the option `trailingComma` from the configuration and the CLI. Use the option `trailingCommas` instead:

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

- [#4760](https://github.com/biomejs/biome/pull/4760) [`0425b90`](https://github.com/biomejs/biome/commit/0425b903077d97bbc5aff497f64a6fa3931caef3) Thanks [@ematipico](https://github.com/ematipico)! - Removed `--apply` and `--apply-unsafe`.

  The CLI options `--apply` and `--apply-unasfe` aren't accepted anymore. Use `--write` and `--write --unafe` instead:

  ```diff
  -biome check --apply-unsafe
  +biome check --write --unsafe
  ```

  ```diff
  -biome check --apply
  +biome check --write
  ```

- [#4760](https://github.com/biomejs/biome/pull/4760) [`6a8ad85`](https://github.com/biomejs/biome/commit/6a8ad85aa520dc3d59dd085255df79b6bc3c848c) Thanks [@ematipico](https://github.com/ematipico)! - Removed support for `assert` syntax.

  Biome now longer supports the `assert` syntax, use the new `with` syntax instead

  ```diff
  -import {test} from "foo.json" assert { for: "for" }
  -export * from "mod" assert { type: "json" }
  +import {test} from "foo.json" with { for: "for" }
  +export * from "mod" with { type: "json" }
  ```

- [#5527](https://github.com/biomejs/biome/pull/5527) [`5ca1af2`](https://github.com/biomejs/biome/commit/5ca1af22e42007d7db8e3bb8f90f6497e8dad1ea) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#5495](https://github.com/biomejs/biome/issues/5495): The rule
  [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) has been
  updated to accept the `rel="noopener"` in addition to `rel="noreferrer"`.
  In addition, an option has been added that allows `rel="noreferrer"` to be
  disabled.

  The rule has been moved from the `a11y` group to the `security` group.

- [#5388](https://github.com/biomejs/biome/pull/5388) [`2e835e1`](https://github.com/biomejs/biome/commit/2e835e1c148b4b38fabc4682bd1d7c098d533710) Thanks [@arendjr](https://github.com/arendjr)! - The rule `useImportRestrictions` has been renamed to [`noPrivateImports`](https://biomejs.dev/linter/rules/no-private-imports), and its
  functionality has been significantly upgraded.

  Previously, the rule would assume that any direct imports from modules inside
  other directories should be forbidden due to their _package private_ visibility.

  The updated rule allows configuring the default visibility of exports, and
  recognises JSDoc comments to override this visibility. The default visibility
  is now `**public**`, but can be set to `**package**`, or even `**private**`.
  Refer to the [documentation of the rule](https://biomejs.dev/linter/rules/no-private-imports) to understand how to leverage the JSDoc comments.

  `noPrivateImports` is now recommended by default.

- [#5193](https://github.com/biomejs/biome/pull/5193) [`14ad3f5`](https://github.com/biomejs/biome/commit/14ad3f503832bf6b43d55bac1b4bbbb93f7e01f1) Thanks [@arendjr](https://github.com/arendjr)! - The Biome daemon now reuses its workspace across connections. This allows multiple clients to
  reuse the same documents and other cached data that we extract from them.

  This primarily affects our IDE extensions: If you open multiple IDEs/windows for the same project,
  they'll connect to the same daemon and reuse each other's workspace.

  The Biome CLI is unaffected unless you opt in with the `--use-server` argument.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`3936022`](https://github.com/biomejs/biome/commit/39360226f674cecd0fbdbe63f23fab9ad98193e4) Thanks [@ematipico](https://github.com/ematipico)! - Biome no longer treats too large files as errors.

  Previously, files that exceed the configured size limit would throw an error, and the CLI would exit with an error code.

  Now, the CLI ignores the file, emits an _information_ diagnostic and doesn't exit with an error code.

- [#5899](https://github.com/biomejs/biome/pull/5899) [`07803ad`](https://github.com/biomejs/biome/commit/07803ad3a6c276867a16efec774c87a128bf4651) Thanks [@Conaclos](https://github.com/Conaclos)! - Change the group of some rules, promote nursery rules and update the recommended rule set.

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

- [#5899](https://github.com/biomejs/biome/pull/5899) [`07803ad`](https://github.com/biomejs/biome/commit/07803ad3a6c276867a16efec774c87a128bf4651) Thanks [@Conaclos](https://github.com/Conaclos)! - Update the default severity level of lint rules.

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

- [#5133](https://github.com/biomejs/biome/pull/5133) [`84e0407`](https://github.com/biomejs/biome/commit/84e0407a278e3228c3ddfebfefebaf7542f5df4d) Thanks [@ematipico](https://github.com/ematipico)! - Reworked some recommended rules recommended to be less pedantic and blocking. This is a **breaking change** if your project relied on those rules to block the CI in case of violations; if that's the case, you should raise their severity level to **error**.

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

- [#5805](https://github.com/biomejs/biome/pull/5805) [`901058a`](https://github.com/biomejs/biome/commit/901058ab6ae4fc90ad2df462d051d2eead00239e) Thanks [@unvalley](https://github.com/unvalley)! - Renamed the global option `--skip-errors` to `--skip-parse-errors`.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`0680ba5`](https://github.com/biomejs/biome/commit/0680ba51765fbb3d6334008471485f9ed54791d3) Thanks [@ematipico](https://github.com/ematipico)! - Remove the code action `quickfix.suppressRule`.

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

- [#4819](https://github.com/biomejs/biome/pull/4819) [`78c8910`](https://github.com/biomejs/biome/commit/78c8910ff0eb63288618e744fd64529682b87d46) Thanks [@ematipico](https://github.com/ematipico)! - Changed default formatting of `package.json`.

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

- [#4788](https://github.com/biomejs/biome/pull/4788) [`93d1e23`](https://github.com/biomejs/biome/commit/93d1e232ec38e30f71929e61c8fa6c0bcb9e6d03) Thanks [@ematipico](https://github.com/ematipico)! - The `organizeImports` is now part of Biome Assist.

- [#4759](https://github.com/biomejs/biome/pull/4759) [`9568041`](https://github.com/biomejs/biome/commit/95680416c9c7aec57635e2e5762db3163d693414) Thanks [@ematipico](https://github.com/ematipico)! - The rule [`noVar`](https://biomejs.dev/linter/rules/no-var/) now belongs to the `suspicious` group

- [#4777](https://github.com/biomejs/biome/pull/4777) [`4f4cafb`](https://github.com/biomejs/biome/commit/4f4cafb16d8c05a545190feda2acb44a217eac88) Thanks [@ematipico](https://github.com/ematipico)! - The rule [`useWhile`](https://biomejs.dev/linter/rules/use-while/) now belongs to the `complexity` group.

- [#5332](https://github.com/biomejs/biome/pull/5332) [`08de81d`](https://github.com/biomejs/biome/commit/08de81dfeb1ef7c23e7b74f4a6aae7b34a4ff5ed) Thanks [@arendjr](https://github.com/arendjr)! - The rule [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) has been updated to suggest actual file extensions instead of guesses based on hueristics.

  As part of this, the `suggestedExtensions` option has been removed. A simpler,
  new option called `forceJsExtensions` has been introduced for those who use
  `tsc`'s `"module": "node16"` setting.

  The rule also no longer reports diagnostics to add an extension when the path
  doesn't exist at all, with or without extension.

- [#5741](https://github.com/biomejs/biome/pull/5741) [`814de9f`](https://github.com/biomejs/biome/commit/814de9fb69dc5387bafc9cc5000f41f7e170fd2a) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#4545](https://github.com/biomejs/biome/issues/4545): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now correctly ignores declarations inside TypeScript's external modules.

  The following interface name is no longer reported by the rule:

  ```ts
  declare module "myExternalModule" {
    export interface my_INTERFACE {}
  }
  ```

- [#4935](https://github.com/biomejs/biome/pull/4935) [`112d43e`](https://github.com/biomejs/biome/commit/112d43ecac2120d70576bd9879b6005e28badae4) Thanks [@fireairforce](https://github.com/fireairforce)! - The rule [`useAltText`](https://biomejs.dev/linter/rules/use-alt-text/) no longer checks the element's attributes containing object spread.

  The following code doesn't trigger the rule anymore:

  ```jsx
  <img src="test.png" alt={alt} {...restProps}></img>
  ```

- [#4945](https://github.com/biomejs/biome/pull/4945) [`069cbb4`](https://github.com/biomejs/biome/commit/069cbb4f6118e5daf6113bbe54ec3c4c3e4b7d0f) Thanks [@Conaclos](https://github.com/Conaclos)! - The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer accepts non-ASCII characters by default.

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

- [#5403](https://github.com/biomejs/biome/pull/5403) [`adaa65c`](https://github.com/biomejs/biome/commit/adaa65ce06b7882accd60a97207ad4403bde23cf) Thanks [@ematipico](https://github.com/ematipico)! - Renamed the rule `noUnnecessaryContinue` to `noUselessContinue`. Run the command `biome migrate` to update your configuration.

- [#5351](https://github.com/biomejs/biome/pull/5351) [`07775c7`](https://github.com/biomejs/biome/commit/07775c7a2c4aa12b15d8fd958e0d195f84724aac) Thanks [@ematipico](https://github.com/ematipico)! - Renamed the rule `noMultipleSpacesInRegularExpressionLiterals` to `noAdjacentSpacesInRegex`. Run the command `biome migrate` to update your configuration.

### Minor Changes

- [#5527](https://github.com/biomejs/biome/pull/5527) [`5ca1af2`](https://github.com/biomejs/biome/commit/5ca1af22e42007d7db8e3bb8f90f6497e8dad1ea) Thanks [@arendjr](https://github.com/arendjr)! - An option called `allowNoReferrer` has been added to the
  [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) rule.

  By default, `noBlankTarget` accepts both `rel="noopener"` and `rel="noreferrer"`
  with links that have `target="_blank"`. This is because the latter _implies_ the
  former, so either one is sufficient to mitigate the security risk.

  However, allowing `rel="noreferrer"` may still be undesirable, because it can
  break tracking, which may be an undesirable side-effect. As such, you can set
  `allowNoReferrer: false` to _only_ accept `rel="noopener"`.

- [#4718](https://github.com/biomejs/biome/pull/4718) [`21ef4aa`](https://github.com/biomejs/biome/commit/21ef4aa7a1a6bd4701a8d2eae38c14c456c7c5bc) Thanks [@ematipico](https://github.com/ematipico)! - Added new option `javascript.parser.jsxEverywhere`. This new option allows to control whether Biome should expect JSX syntax in `.js`/`.mjs`/`.cjs` files.

  When `jsxEverywhere` is set to `false`, having JSX syntax like `<div></div>` inside `.js`/`.mjs`/`.cjs` files will result in a **parsing error**.

  Despite the name of the option, JSX is never supported inside `.ts` files. This is because TypeScript generics syntax may conflict with JSX in such files.

  This option defaults to `true`.

- [#5079](https://github.com/biomejs/biome/pull/5079) [`0cfcaec`](https://github.com/biomejs/biome/commit/0cfcaec193717fd29a0a3e14835bbee2c9157630) Thanks [@r1tsuu](https://github.com/r1tsuu)! - Add a new JS assist rule - `useSortedKeys` which enforces ordering of a JS object properties.
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

- [#4911](https://github.com/biomejs/biome/pull/4911) [`d400d69`](https://github.com/biomejs/biome/commit/d400d69bbf91339dc3931bad6f0648e75da19019) Thanks [@kaykdm](https://github.com/kaykdm)! - Added the new rule [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises).

- [#4948](https://github.com/biomejs/biome/pull/4948) [`b8c57d2`](https://github.com/biomejs/biome/commit/b8c57d2724bda65a27575a741a70df5600f5c6a4) Thanks [@arendjr](https://github.com/arendjr)! - Added the new rule [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles).

- [#4650](https://github.com/biomejs/biome/pull/4650) [`c1b2e7b`](https://github.com/biomejs/biome/commit/c1b2e7b74023c822e625c6e5a55c5f40c33aa2ca) Thanks [@ematipico](https://github.com/ematipico)! - Added the new rule [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore).

- [#4731](https://github.com/biomejs/biome/pull/4731) [`5c3e3e1`](https://github.com/biomejs/biome/commit/5c3e3e1516055f3577c51e5b0e503710b2684de2) Thanks [@unvalley](https://github.com/unvalley)! - Added the new rule [`noUnwantedPolyfillio`](https://biomejs.dev/linter/rules/no-unwanted-polyfillio).

- [#4819](https://github.com/biomejs/biome/pull/4819) [`78c8910`](https://github.com/biomejs/biome/commit/78c8910ff0eb63288618e744fd64529682b87d46) Thanks [@ematipico](https://github.com/ematipico)! - Added a format option `expand` for Javascript and JSON formatters.
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

- [#5530](https://github.com/biomejs/biome/pull/5530) [`ae1a8e9`](https://github.com/biomejs/biome/commit/ae1a8e970eb4c9e0ad8ec3b30862e1a1d1a164bf) Thanks [@arendjr](https://github.com/arendjr)! - The nursery rule [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) has been added.

  Importing a non-existing export is an error at runtime or build time. With this
  rule, Biome can detect such incorrect imports and report errors for them.

  Note that if you use TypeScript, you probably don't want to use this rule, since
  TypeScript already performs such checks for you.

- [#5642](https://github.com/biomejs/biome/pull/5642) [`2d86f22`](https://github.com/biomejs/biome/commit/2d86f2243bb4312bf0f5ce52b6cd55558a4ab456) Thanks [@amasotti](https://github.com/amasotti)! - The rule [`noFocusedTests`](https://biomejs.dev/linter/rules/no-focused-tests/) can now detect the usage of focused tests inside loops.

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

- [#4867](https://github.com/biomejs/biome/pull/4867) [`94bf15e`](https://github.com/biomejs/biome/commit/94bf15e3d24ddae0631aa2e9966a7ade771356d1) Thanks [@ematipico](https://github.com/ematipico)! - Linter groups now accept new options to enable/disable all rules that belong to a group, and control the severity
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

- [#4760](https://github.com/biomejs/biome/pull/4760) [`f281e8a`](https://github.com/biomejs/biome/commit/f281e8aadcdf9eb25e45def923c3c153c9ff299c) Thanks [@ematipico](https://github.com/ematipico)! - Biome assist is a new feature of the Biome analyzer. The assist is meant to provide **actions**. Actions differ from linter rules in that they aren't meant to signal errors.

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

- [#4760](https://github.com/biomejs/biome/pull/4760) [`59f7e10`](https://github.com/biomejs/biome/commit/59f7e100aca357d322d331713668bf6c38c5603d) Thanks [@ematipico](https://github.com/ematipico)! - Biome migrate eslint outputs a better overriding behavior.

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

- [#5232](https://github.com/biomejs/biome/pull/5232) [`da7b99e`](https://github.com/biomejs/biome/commit/da7b99e155da8b27b7e56951f0d223cbfbea8072) Thanks [@minht11](https://github.com/minht11)! - Add [useSymbolDescription](https://biomejs.dev/linter/rules/use-symbol-description/).

- [#5619](https://github.com/biomejs/biome/pull/5619) [`2333c7a`](https://github.com/biomejs/biome/commit/2333c7ad5ab1cd626dcc5dfe6049633340cd7252) Thanks [@hanseltime](https://github.com/hanseltime)! - Suppression of syntax rules

  Added support for suppressing syntax rules. Syntax rules are particular rules meant **to complement the parser**, hence they can't be configured.

  Biome now allows to suppress those rules. This can, for example, be useful in case the rule is affected by a bug. However, this is more an escape hatch, so if a syntax rule requires a suppression, please file an issue.

  Example:

  ```typescript
  // biome-ignore syntax/correctness/noTypeOnlyImportAttributes: bug
  import type { MyType } from "my-esm-pkg" with { "resolution-mode": "import" };
  ```

  Biome now requires all `biome-ignore-start` suppressions to have an equivalent `biome-ignore-end` comment.

- [#5076](https://github.com/biomejs/biome/pull/5076) [`279311a`](https://github.com/biomejs/biome/commit/279311a213e71f1b5f7dca5c04459076065ef47e) Thanks [@siketyan](https://github.com/siketyan)! - Add a new lint rule `noConstantBinaryExpression`.
  This rule is inspired from ESLint's [no-constant-binary-expression](https://eslint.org/docs/latest/rules/no-constant-binary-expression) rule.

- [#6004](https://github.com/biomejs/biome/pull/6004) [`507f71e`](https://github.com/biomejs/biome/commit/507f71eb1269a1f67351fb8de9434d836045deb3) Thanks [@Conaclos](https://github.com/Conaclos)! - The CLI options `--only` and `--skip` now accept rule and action names without prefixing the group name.

  Previously `--only=noDebugger` was rejected.
  You had to add the group name: `--only=suspicious/noDebugger`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3574](https://github.com/biomejs/biome/issues/3574): `noUnusedImports` now reports empty named imports and suggests their removal.

  The rule now suggests the removal of empty named imports such as:

  ```diff
  - import {} from "mod";
  ```

- [#5964](https://github.com/biomejs/biome/pull/5964) [`3e8fbee`](https://github.com/biomejs/biome/commit/3e8fbeed0413a7cba05cb2802ca9f968cb66dd35) Thanks [@mdevils](https://github.com/mdevils)! - Added the new rule [`useAdjacentGetterSetter`](https://biomejs.dev/linter/rules/use-adjacent-getter-setter), which enforces getters and setters for the same property
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

- [#5960](https://github.com/biomejs/biome/pull/5960) [`b5084d6`](https://github.com/biomejs/biome/commit/b5084d6a5d88231ec9cea0c13e7b7e98fad65e20) Thanks [@minht11](https://github.com/minht11)! - Added new rule [useConsistentResponse](https://biomejs.dev/linter/rules/use-consistent-response) which suggests to use static [Response.json()](https://developer.mozilla.org/en-US/docs/Web/API/Response/json) and [Response.redirect()](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static) methods instead of `new Response` when possible.

  Example:

  ```js
  new Response(JSON.stringify({ value: 1 }));
  Response.json({ value: 1 });
  ```

- [#4760](https://github.com/biomejs/biome/pull/4760) [`f281e8a`](https://github.com/biomejs/biome/commit/f281e8aadcdf9eb25e45def923c3c153c9ff299c) Thanks [@ematipico](https://github.com/ematipico)! - Biome users can now configure code actions from linter rules as well as assist actions directly in the settings of their IDE/editor.

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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3401](https://github.com/biomejs/biome/issues/3401): `noUnusedImports` now keeps comments separated from the import with a blank line.

  For example:

  ```diff
    // Orphan comment

  - // Header comment
  - import {} from "mod";
  ```

- [#6212](https://github.com/biomejs/biome/pull/6212) [`35304ff`](https://github.com/biomejs/biome/commit/35304ff64863850cabbf2e184a6a9231e16ef3a0) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Added a new `propertyAssignment` option to the `noParameterAssign` rule.
  This option allows to configure whether property assignments on function parameters are permitted.
  By default, `propertyAssignment` is set to `allow`.
  Setting it to `deny` enforces stricter immutability by disallowing property mutations on function parameters.

- [#5083](https://github.com/biomejs/biome/pull/5083) [`7aa79e7`](https://github.com/biomejs/biome/commit/7aa79e79268fd5450dc791838e91812c59220ca2) Thanks [@siketyan](https://github.com/siketyan)! - The formatter option `bracketSpacing` is now also supported in JSON files.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - `useValidTypeof` now accepts comparisons with variables.

  Previously, the rule required to compare a `typeof` expression against another `typeof` expression or a valid string literal. We now accept more cases, notably comparison against a variable:

  ```js
  if (typeof foo === bar) {
    // ...
  }
  ```

- [#6053](https://github.com/biomejs/biome/pull/6053) [`64ca243`](https://github.com/biomejs/biome/commit/64ca2438db697e0dd2ee64a92872495c93a7b234) Thanks [@mdevils](https://github.com/mdevils)! - Added the new rule [`noNestedComponentDefinitions`](https://biomejs.dev/linter/rules/no-nested-component-definitions),
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

- [#4983](https://github.com/biomejs/biome/pull/4983) [`7e938f3`](https://github.com/biomejs/biome/commit/7e938f35d9a4890e0d723d1549702bb3e7c0cb2f) Thanks [@ematipico](https://github.com/ematipico)! - Added the new rule [`noDestructuredProps`](https://biomejs.dev/linter/rules/no-destructured-props/), which disallow the use of destructured props in Solid projects.

- [#4730](https://github.com/biomejs/biome/pull/4730) [`a478377`](https://github.com/biomejs/biome/commit/a478377129491ee20acd5e122228994a062b8b00) Thanks [@ematipico](https://github.com/ematipico)! - You can now enable lint rules using the default severity suggested by Biome using the new variant `"on"`, when enabling a rule.

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

- [#5195](https://github.com/biomejs/biome/pull/5195) [`d69a664`](https://github.com/biomejs/biome/commit/d69a664f0a42b0e7fb05f6aaf2ca17bfc612f58d) Thanks [@ematipico](https://github.com/ematipico)! - Biome VCS integration now supports nested ignore files.

  For `git`, if a `.gitignore` is found in a nested folder `root/packages/foo/`, and it contains the pattern `dist/`, only files and directories inside `root/packages/foo/dist` are matched.

- [#6082](https://github.com/biomejs/biome/pull/6082) [`d4f58b5`](https://github.com/biomejs/biome/commit/d4f58b5034934fac4b4cdc9f0030eac3d63fada1) Thanks [@mehm8128](https://github.com/mehm8128)! - Added the rule [useUniqueElementIds](https://biomejs.dev/linter/rules/use-unique-element-ids/).
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

- [#5042](https://github.com/biomejs/biome/pull/5042) [`d640aaf`](https://github.com/biomejs/biome/commit/d640aafebd6545f6e12360531efb9317a3c9b927) Thanks [@dy0gu](https://github.com/dy0gu)! - Added the new JavaScript rule [`useConsistentObjectDefinition`](https://biomejs.dev/linter/rules/use-consistent-object-definition/) rule. The rule enforces a consistent style for the definition of objects:

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

- [#6177](https://github.com/biomejs/biome/pull/6177) [`8cfbbd3`](https://github.com/biomejs/biome/commit/8cfbbd39b2b55f2526e93b3ad219fd67faba48b9) Thanks [@arendjr](https://github.com/arendjr)! - Introduced more advanced logging capabilities:

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

- [#5230](https://github.com/biomejs/biome/pull/5230) [`7c1e505`](https://github.com/biomejs/biome/commit/7c1e505aea0f7abb7ff4f7eb73357633daf09a1d) Thanks [@tim-we](https://github.com/tim-we)! - Added options to `suspicious/noConfusingLabels` to allow specific labels.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4549](https://github.com/biomejs/biome/issues/4549): [noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property/) now accepts more known CSS properties.

  ```diff
  - ['anchor-default', 'anchor-scroll', 'inset-area', 'position-animation', 'position-fallback', 'position-fallback-bounds', 'position-try-options']
  + ['anchor-scope', 'interpolate-size', 'line-fit-edge', 'masonry', 'masonry-auto-tracks', 'masonry-direction', 'masonry-fill', 'masonry-flow', 'masonry-slack', 'masonry-template-areas', 'masonry-template-tracks', 'position-anchor', 'position-area', 'position-try-fallbacks', 'position-visibility', 'scroll-start-target', 'text-box', 'view-transition-class', 'view-transition-group']
  ```

  This change replaces deprecated properties, improving CSS validation.

- [#5093](https://github.com/biomejs/biome/pull/5093) [`766492f`](https://github.com/biomejs/biome/commit/766492f3535aef3a0a9bef47d607d07f8b70c749) Thanks [@siketyan](https://github.com/siketyan)! - LSP clients can now override the configuration path for each workspace, by responding to
  `workspace/configuration` requests.

- [#5555](https://github.com/biomejs/biome/pull/5555) [`894e181`](https://github.com/biomejs/biome/commit/894e181f2fb7cde84cee3d345fdb4e5f6ca38747) Thanks [@ematipico](https://github.com/ematipico)! - Added the new CSS rule [`noImportantStyles`](https://biomejs.dev/linter/rules/no-important-styles), which prevents the use of `!important` inside CSS declarations.

- [#4796](https://github.com/biomejs/biome/pull/4796) [`e7dd706`](https://github.com/biomejs/biome/commit/e7dd706b93b44c5febe6710ba0cfa3b6365fccaf) Thanks [@MaxtuneLee](https://github.com/MaxtuneLee)! - Biome now emits a warning diagnostic if the configuration contains an out-of-sync schema URL.

- [#6177](https://github.com/biomejs/biome/pull/6177) [`8cfbbd3`](https://github.com/biomejs/biome/commit/8cfbbd39b2b55f2526e93b3ad219fd67faba48b9) Thanks [@arendjr](https://github.com/arendjr)! - Introduced a new configuration setting `files.experimentalScannerIgnores`.

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

- [#5844](https://github.com/biomejs/biome/pull/5844) [`3e49b1e`](https://github.com/biomejs/biome/commit/3e49b1e36bdcb3ab46b522ade8c1d5edb410b495) Thanks [@minht11](https://github.com/minht11)! - Added the new rule [useSingleJsDocAsterisk](https://biomejs.dev/linter/rules/use-single-js-doc-asterisk/) which enforces JSDoc comment lines to start with a single asterisk.

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

- [#5132](https://github.com/biomejs/biome/pull/5132) [`3065eb4`](https://github.com/biomejs/biome/commit/3065eb4554b2a44a7c9962dce4870c17f4df6f3c) Thanks [@siketyan](https://github.com/siketyan)! - The CLI flag `--javascript-attribute-position` was renamed to `--javascript-formatter-attribute-position` for consistency.

- [#4713](https://github.com/biomejs/biome/pull/4713) [`0a9d85a`](https://github.com/biomejs/biome/commit/0a9d85af5f9b35e6db78ae49236dac04e03faeb0) Thanks [@ematipico](https://github.com/ematipico)! - Introduced the `domains` linter feature. The Biome linter now has a new way to opt-in rules, with a concept called `domains`.

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

- [#5794](https://github.com/biomejs/biome/pull/5794) [`107d632`](https://github.com/biomejs/biome/commit/107d6327bf59fdefe22f12d7808f9f165f0cc61a) Thanks [@ematipico](https://github.com/ematipico)! - Biome now prints diagnostics sorted by their severity. The order is the following:

  1. information
  2. warning
  3. error

  This means that _error_ diagnostics are printed **last**, so users can see them first.

- [#5754](https://github.com/biomejs/biome/pull/5754) [`b675840`](https://github.com/biomejs/biome/commit/b675840b32e3c8c5514542c6f67c5ca4890c16f2) Thanks [@mdevils](https://github.com/mdevils)! - Added the new rule [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return), which enforces consistent return values in iterable callbacks.

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

- [#6157](https://github.com/biomejs/biome/pull/6157) [`a76c4d4`](https://github.com/biomejs/biome/commit/a76c4d41f6bc2739a9f7b465102c16b0bd97d859) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Added the new rule [`noReactPropAssign`](https://biomejs.dev/linter/rules/no_react_prop_assign), based on the react-hooks rule [react-hooks/react-compiler](https://www.npmjs.com/package/eslint-plugin-react-hooks)

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

- [#5274](https://github.com/biomejs/biome/pull/5274) [`77f2382`](https://github.com/biomejs/biome/commit/77f2382d9e55d919c0a93fc499ed08e167c40155) Thanks [@huangtiandi1999](https://github.com/huangtiandi1999)! - Added new rule [`noBitwiseOperators`](https://biomejs.dev/linter/rules/no-bitwise-operators/), which disallows bitwise operators.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`0680ba5`](https://github.com/biomejs/biome/commit/0680ba51765fbb3d6334008471485f9ed54791d3) Thanks [@ematipico](https://github.com/ematipico)! - The Biome analyzer now supports a new top-level suppression. These suppression have to be placed at the top of the file, and they must be followed by two newlines (`\n\n\`).

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

- [#5635](https://github.com/biomejs/biome/pull/5635) [`4301260`](https://github.com/biomejs/biome/commit/43012600fea906bcdb646b7a5151501c3484cd18) Thanks [@uncenter](https://github.com/uncenter)! - Added the new rule [`useNumericSeparators`](https://biomejs.dev/linter/rules/use-numeric-separators), which encourages the use of numeric separators to improve readability.

- [#5532](https://github.com/biomejs/biome/pull/5532) [`68798c3`](https://github.com/biomejs/biome/commit/68798c3c7e4819205f67d697f00c1056327ea6d8) Thanks [@minht11](https://github.com/minht11)! - [useImportExtensions](https://biomejs.dev/linter/rules/use-import-extensions/) now checks imports with sub extensions.

  ```js
  - import 'styles.css'
  + import 'styles.css.ts'
  ```

- [#6010](https://github.com/biomejs/biome/pull/6010) [`9272d5b`](https://github.com/biomejs/biome/commit/9272d5ba4711281455e2e4c7973b5897a058ca11) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - It's possible to override the option `files.maxSize`. This option is helpful if you need to process specific files that exceed the default `maxSize`:

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

- [#5337](https://github.com/biomejs/biome/pull/5337) [`bab955b`](https://github.com/biomejs/biome/commit/bab955bc52feb61259e1dd0d696843aa3aa30fc6) Thanks [@ematipico](https://github.com/ematipico)! - Added the new CLI option called `--threads` to the `ci` command. It allows to control the numbers of threads that can be used when using the Biome CLI.

  It's possible to use the environment variable `BIOME_THREADS` as an alternatives.

  This feature is useful when running the CLI in environments that have limited resources, for example CI/CD.

  ```shell
  biome ci --threads=1
  BIOME_THREADS=1 biome ci
  ```

- [#6129](https://github.com/biomejs/biome/pull/6129) [`67dfa88`](https://github.com/biomejs/biome/commit/67dfa888ce7a0815e06d90eea13d467bbb850d0d) Thanks [@mdevils](https://github.com/mdevils)! - Added the new rule [`useObjectSpread`](https://biomejs.dev/linter/rules/use-object-spread), which prefers object spread syntax over `Object.assign()` when constructing new objects.

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

- [#5121](https://github.com/biomejs/biome/pull/5121) [`98b43e6`](https://github.com/biomejs/biome/commit/98b43e60a80c4482a8b39557a8e7a7768765f473) Thanks [@anthonyshew](https://github.com/anthonyshew)! - Added an option to the `lint` command called `--suppress`. The new option suppresses a violation instead of applying a rule fix. The option accepts a string that is used as _reason_ of the suppression comment.

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

- [#5157](https://github.com/biomejs/biome/pull/5157) [`31b6870`](https://github.com/biomejs/biome/commit/31b68701464b6325247cf853ddb31ea36df9afc6) Thanks [@iamakulov](https://github.com/iamakulov)! - Add an `ignoreRestSiblings` option into [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables).

  When this option is set to `true`, the rule will ignore variables that created using the rest pattern:

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noUnusedVariables": {
            "level": "error",
            "options": {
              "ignoreRestSiblings": true
            }
          }
        }
      }
    }
  }
  ```

  ```js
  const { lorem, ...test } = bar; // the variable "test" won't trigger the rule
  console.log(lorem);
  ```

- [#6063](https://github.com/biomejs/biome/pull/6063) [`e491126`](https://github.com/biomejs/biome/commit/e4911261c528f9fc10e2c57ad7ab059befcbc2ef) Thanks [@Conaclos](https://github.com/Conaclos)! - Upgraded some unsafe fixes to safe fixes.

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

- [#6164](https://github.com/biomejs/biome/pull/6164) [`23a701c`](https://github.com/biomejs/biome/commit/23a701cec0b1728a90a81a519c4d40df631cb17d) Thanks [@ematipico](https://github.com/ematipico)! - Added support for monorepos. The feature will work _out of the box_ for the majority of the users. If your project
  has **nested configuration** files, use the command `biome migrate` from the _root of the project_.

  Monorepo support in Biome is done in a single way. Create a `biome.json` at the root of the project. This configuration
  file is now called the root configuration. Then, each nested configuration file must specify the new field `"root": false`.

  We also introduced a new microsyntax for _extending a nested configuration from the root configuration_, which is `"extends": "//"`. This new syntax means “this config _extends_ from the root config”. When using this microsyntax, you **may omit** the `"root": false` field as it is implied.

  Note that nested configs are not required to extend from the root config, and you can still have independent nested configs, as well as nested configs that extend from other files. In those cases, `"root": false` must be specified explicitly.

- [#5600](https://github.com/biomejs/biome/pull/5600) [`38ee189`](https://github.com/biomejs/biome/commit/38ee1898b9a17dc4afd13a68b6c2992c9dd1fc73) Thanks [@ematipico](https://github.com/ematipico)! - Added support for formatting `.html` files. The formatting is considered **experimental,** and it's only opt-in via configuration:

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

- [#5326](https://github.com/biomejs/biome/pull/5326) [`45d67f3`](https://github.com/biomejs/biome/commit/45d67f30f123d0e9352cd3159437466dd85a278e) Thanks [@siketyan](https://github.com/siketyan)! - Added an **unsafe** fix to the rule [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies).

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

- [#5469](https://github.com/biomejs/biome/pull/5469) [`15ddac7`](https://github.com/biomejs/biome/commit/15ddac78286dbcdfd88c483c15e28862a4314ace) Thanks [@siketyan](https://github.com/siketyan)! - The rule `useExhaustiveDependencies` now reports a diagnostic when the dependency list is not an array literal.

- [#6121](https://github.com/biomejs/biome/pull/6121) [`c113eb6`](https://github.com/biomejs/biome/commit/c113eb6880e21b1216e9c5a399f3e958c88f5baa) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Added the new rule [`useIndexOf`](https://biomejs.dev/linter/rules/use-index-of), based on the unicorn rule [prefer-array-index-of](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/prefer-array-index-of.md)

- [#5762](https://github.com/biomejs/biome/pull/5762) [`6a59e88`](https://github.com/biomejs/biome/commit/6a59e88c345586ac7298ab2ea445f07f48555bd8) Thanks [@siketyan](https://github.com/siketyan)! - Added a new rule [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/), which detects any missing cases for switch statements.
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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4416](https://github.com/biomejs/biome/pull/4416): The rules [`useExportType`](https://biomejs.dev/linter/rules/use-export-type/) and [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now ignore TypeScript declaration files.

- [#5203](https://github.com/biomejs/biome/pull/5203) [`d95df40`](https://github.com/biomejs/biome/commit/d95df40a86c8debb369fdc9070c91642325bfe1f) Thanks [@fireairforce](https://github.com/fireairforce)! - Added the new rule [`useForComponent`](https://biomejs.dev/linter/rules/use-for-component/).

  This rule enforces usage of Solid's `<For />` component for mapping an array to JSX elements.

- [#5761](https://github.com/biomejs/biome/pull/5761) [`74e5eb6`](https://github.com/biomejs/biome/commit/74e5eb61ee10f8775974ed93ec6b7e6494b5c82d) Thanks [@dyc3](https://github.com/dyc3)! - Added new lint rule [`noShadow`](http://biome.dev/linter/rules/no-shadow), a port of eslint's `no-shadow`.

  This rule disallows variable declarations from shadowing variables declared in an outer scope. For example:

  ```js
  const foo = 1;

  function bar() {
    const foo = 2; // This variable shadows the outer foo
  }
  ```

- [#5717](https://github.com/biomejs/biome/pull/5717) [`fadee6b`](https://github.com/biomejs/biome/commit/fadee6b94a487acfe0f7824c1877e62e1239170e) Thanks [@Conaclos](https://github.com/Conaclos)! - Add `style` option for the [useImportType](https://biomejs.dev/linter/rules/use-import-type/) rule.

  The rule now allows enforcing an import style for importing types.
  See the rule documentation for more details.

- [#6184](https://github.com/biomejs/biome/pull/6184) [`c1dc0ac`](https://github.com/biomejs/biome/commit/c1dc0acd0df0b6c4d502d30ee1f201ca58278d1e) Thanks [@Jordanh1996](https://github.com/Jordanh1996)! - Added the new rule [`useJsonImportAttribute`](https://biomejs.dev/linter/rules/use-json-import-attribute) to enforce the use of import attributes for JSON modules.

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

- [#6092](https://github.com/biomejs/biome/pull/6092) [`97ac0d0`](https://github.com/biomejs/biome/commit/97ac0d02f26ae3d902f6a3ec4ebc7fd29b664b02) Thanks [@Conaclos](https://github.com/Conaclos)! - [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) now handles numeric keys and is declared as being the same rule as the ESLint [no-useless-computed-key](https://eslint.org/docs/latest/rules/no-useless-computed-key) rule.

- [#5529](https://github.com/biomejs/biome/pull/5529) [`241a440`](https://github.com/biomejs/biome/commit/241a4406b418711d7e903c5d6ddaa4b74689a74a) Thanks [@Conaclos](https://github.com/Conaclos)! - `useNamingConmvention` now ignores unused variables prefixed with an underscore `_`.

  This avoids conflicts with the unsafe fix of `noUnusedVariables`.
  The following code is now accepted because the variable is unused and prefixed with an underscore.

  ```js
  const _Unknown_Style = 0;
  ```

- [#4760](https://github.com/biomejs/biome/pull/4760) [`d469189`](https://github.com/biomejs/biome/commit/d469189298a2358989ee7e906b840f1d30fe5ad5) Thanks [@ematipico](https://github.com/ematipico)! - The package now requires `v2` of the WebAssembly packages. The internal APIs of Workspace are now `camelCase`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - The rule [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now provides a code fix.

  ```diff
  - const xs = new Array();
  + const xs = [];
  ```

  The code fix is currently marked as unsafe.
  We plan to make it safe in a future release of Biome.

- [#5986](https://github.com/biomejs/biome/pull/5986) [`19e41b4`](https://github.com/biomejs/biome/commit/19e41b40d360d27deec4561112d85a728cc282ee) Thanks [@ematipico](https://github.com/ematipico)! - The command `migrate` is now able to migrate nested configuration files.

- [#5578](https://github.com/biomejs/biome/pull/5578) [`7872704`](https://github.com/biomejs/biome/commit/78727041ff58b156216b5a546afc89134e95de5b) Thanks [@mdevils](https://github.com/mdevils)! - Added the new rule [`noRestrictedElements`](https://biomejs.dev/linter/rules/no-restricted-elements), which prevents use of the specified HTML elements and components.

- [#5129](https://github.com/biomejs/biome/pull/5129) [`95a5407`](https://github.com/biomejs/biome/commit/95a54070c73d3e20d96979e247f841b649b47362) Thanks [@unvalley](https://github.com/unvalley)! - Added the new lint rule [`noAwaitInLoop`](https://biomejs.dev/linter/rules/no-await-in-loop).

### Patch Changes

- [#5014](https://github.com/biomejs/biome/pull/5014) [`028af9c`](https://github.com/biomejs/biome/commit/028af9c89af4ac62089907e5523584bef47639f9) Thanks [@vohoanglong0107](https://github.com/vohoanglong0107)! - Fix [#5001](https://github.com/biomejs/biome/issues/5001), where the CSS formatter removes whitespace from selector preceded by a comment

- [#5955](https://github.com/biomejs/biome/pull/5955) [`ac6ed6d`](https://github.com/biomejs/biome/commit/ac6ed6d7910b73ed4e1c5bb96925634c7a3f9db9) Thanks [@daivinhtran](https://github.com/daivinhtran)! - Fixed [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) rule to suggest unsafe fix for unused function declarations.

- [#6154](https://github.com/biomejs/biome/pull/6154) [`3b9c4cf`](https://github.com/biomejs/biome/commit/3b9c4cf217faaa62b3b568f0566644d1e3480c48) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where the ordering of the diagnostics wasn't predictable.

- [#6181](https://github.com/biomejs/biome/pull/6181) [`56e8afb`](https://github.com/biomejs/biome/commit/56e8afb4d6649d7ce126cfd31de18ee8b9e4e881) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where the environment variable `BIOME_CONFIG_PATH` wasn't correctly picked up.

- [#4949](https://github.com/biomejs/biome/pull/4949) [`7b91d19`](https://github.com/biomejs/biome/commit/7b91d19383c1e1ce9395f7c2049b8fbc353f2965) Thanks [@ematipico](https://github.com/ematipico)! - Biome logs a warning in case a folder contains `biome.json` and `biome.jsonc`, and it will use `biome.json` by default.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) is now able to bind read of value to a type-only import in ambient contexts ([#4526](https://github.com/biomejs/biome/issues/4526)).

  In the following code, `A` is now correctly bound to the type-only import.
  Previously, `A` was reported as an undeclared variable.

  ```ts
  import type { A } from "mod";

  declare class B extends A {}
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fix [#4317](https://github.com/biomejs/biome/issues/4317), setter parameter can contain a trailing comma, the following example will now parsed correctly:

  ```ts
  export class DummyClass {
    set input(value: string) {}
  }
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fix [#4575](https://github.com/biomejs/biome/issues/4575), don't wrap selector indentation after css comments.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fix [#4258](https://github.com/biomejs/biome/issues/4258), where fixed css parse error with @-moz-document url-prefix().

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4391](https://github.com/biomejs/biome/issues/4391): Some files from the `.vscode` directory are no longer incorrectly parsed as JSON.

- [#5719](https://github.com/biomejs/biome/pull/5719) [`020c0ed`](https://github.com/biomejs/biome/commit/020c0ed21306301625f46c1ddbfe38d8d46c5bb5) Thanks [@unvalley](https://github.com/unvalley)! - The `biome format` command now correctly handles the `--skip-errors` option, allowing it to skip files with syntax errors and continue formatting the remaining valid files.
  When this option is used, skipped syntax errors are reported as information, since the user is already aware of them.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - `biome migrate eslint` now correctly resolves the scoped package named `eslint-config`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3836](https://github.com/biomejs/biome/issues/3836): The CSS parser will now correctly parse the following:

  ```css
  .foo {
    color: red;
  }
  ```

- [#5769](https://github.com/biomejs/biome/pull/5769) [`5f321a9`](https://github.com/biomejs/biome/commit/5f321a9faac874bcc18053709fc24a0bdaab9992) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where the related diagnostics attached to the main diagnostics didn't have a correct message.

- [#6171](https://github.com/biomejs/biome/pull/6171) [`58e78fa`](https://github.com/biomejs/biome/commit/58e78fa7a4a62f51fac620868006beea7e39c570) Thanks [@emilyinure](https://github.com/emilyinure)! - Fixed `noAccumulatingSpread` not reporting calls to `Object.assign`. The following code will now be reported:

  ```js
  let a = [{ a: 1 }, { b: 2 }];
  a.reduce((acc, val) => Object.assign(acc, val), []);
  ```

- [#6002](https://github.com/biomejs/biome/pull/6002) [`b67a138`](https://github.com/biomejs/biome/commit/b67a138229e4f4fb02d88abe0ad7876dfb8538cc) Thanks [@ematipico](https://github.com/ematipico)! - The `summary` reporter doesn't take `--max-diagnostics` into account anymore.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4553](https://github.com/biomejs/biome/issues/4553): `noUselessFragments` will now correctly fix JSX attributes:

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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4528](https://github.com/biomejs/biome/issues/4528): `biome migrate eslint` now correctly handles shared ESLint configuration that don't follow the ESLint naming convention.

  ESLint recommends that a package that exports a shared configuration be prefixed with `eslint-config-` or simply named `eslint-config`.
  This is only a recommendation.
  Packages that export shared configurations can have arbitrary names.
  Biome is now able to load any package.

- [#5485](https://github.com/biomejs/biome/pull/5485) [`5572e4c`](https://github.com/biomejs/biome/commit/5572e4cb8acabe13f09452c2e356c91ee680c510) Thanks [@Xstoudi](https://github.com/Xstoudi)! - Fixed [#4993](https://github.com/biomejs/biome/issues/4993): [`useAwait`](https://biomejs.dev/linter/rules/use-await/) now correctly warn on functions with decorator with callback argument.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4756](https://github.com/biomejs/biome/issues/4756): `noDuplicateProperties` now throws lint errors properly when we use `@supports`.

- [#5988](https://github.com/biomejs/biome/pull/5988) [`73be9e1`](https://github.com/biomejs/biome/commit/73be9e1adfc4614a0773a5a32a547dec79e47a10) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#5981](https://github.com/biomejs/biome/issues/5981), where `noUnknownPseudoClass` didn't take `:global` into consideration when `cssModules` is enabled.

- [#5174](https://github.com/biomejs/biome/pull/5174) [`5f7dc3f`](https://github.com/biomejs/biome/commit/5f7dc3f0639a6f660d49e137997c50948dfe8353) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#2406](https://github.com/biomejs/biome/issues/2406): Biome longer expands properties of object type annotations in the only function parameter to align with Prettier.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4740](https://github.com/biomejs/biome/issues/4740): `biome migrate eslint` now correctly handles ESLint configuration with `null` values in file lists.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4202](https://github.com/biomejs/biome/issues/4202): Align with Prettier in formatting test functions.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now properly handles unterminated string literals, such as:

  ```jsx
  function Comp() {
    return (
        <a rel="
  ```

- [#6113](https://github.com/biomejs/biome/pull/6113) [`4ecdaea`](https://github.com/biomejs/biome/commit/4ecdaeae92337da12f14da94260606396c8229da) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where syntax rules didn't provide an automatic way to suppress the rule. Now the LSP will show supression actions if a syntax rule is violated.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a CSS parser error: `@-moz-document url-prefix(https://example.com)` and `@-moz-document domain(example.com)` are now valid.

- [#5170](https://github.com/biomejs/biome/pull/5170) [`890d31b`](https://github.com/biomejs/biome/commit/890d31b18a883c128838d756a665345d98aa02b7) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#4967](https://github.com/biomejs/biome/issues/4967): The fix for `useArrowFunction` no longer breaks function bodies starting with `{`.

- [#5999](https://github.com/biomejs/biome/pull/5999) [`ccbb1af`](https://github.com/biomejs/biome/commit/ccbb1aff86a6aca4e377a0e897781afafa4edfa6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#5998](https://github.com/biomejs/biome/issues/5998). The rule `noUnknownPseudoElement` now correctly cheks names
  of pseudo-element functions.

- [#5043](https://github.com/biomejs/biome/pull/5043) [`3868597`](https://github.com/biomejs/biome/commit/386859758287739ff00e7b0d9faa53ab9adb62af) Thanks [@Jayllyz](https://github.com/Jayllyz)! - Fixed [#5024](https://github.com/biomejs/biome/issues/5024): Added `useJsxKeyInIterable` rule to React domain.

- [#5627](https://github.com/biomejs/biome/pull/5627) [`b5c186e`](https://github.com/biomejs/biome/commit/b5c186ec99e0e500e212984e2d8b26f7115fe284) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#5410](https://github.com/biomejs/biome/issues/5410). Biome now correctly parse an `.editorconfig` that includes character classes in glob patterns.

- [#6006](https://github.com/biomejs/biome/pull/6006) [`f70dee5`](https://github.com/biomejs/biome/commit/f70dee5495cc71e2be4dc25df644fe433c25d2a9) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#2260](https://github.com/biomejs/biome/2260): The LSP server now returns correct text edits for the specified range in `textDocument/rangeFormatting` and `textDocument/onTypeFormatting` requests.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports top-level variables in a global declaration file as unused.

- [#4903](https://github.com/biomejs/biome/pull/4903) [`2a80687`](https://github.com/biomejs/biome/commit/2a8068733fa1fbffea4043d56123f00f2ddb8a35) Thanks [@fireairforce](https://github.com/fireairforce)! - Type exports now support renaming types to `default`.

  The following code is now parsed successfully:

  ```ts
  export { type A as default } from "./b.ts";
  ```

- [#5348](https://github.com/biomejs/biome/pull/5348) [`17f61d2`](https://github.com/biomejs/biome/commit/17f61d2da381bc0e4b4b7a1e15ef2e6d48091530) Thanks [@ematipico](https://github.com/ematipico)! - Added proper support for arrow functions in the lint rule https://biomejs.dev/linter/rules/use-explicit-type/

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer suggests renaming top-level variables in a global declaration file.

- [#5479](https://github.com/biomejs/biome/pull/5479) [`1f33d50`](https://github.com/biomejs/biome/commit/1f33d506a2e815d64b5a67d081b62e2770e1843e) Thanks [@akx](https://github.com/akx)! - Improved context in error messages when migrating Prettier configurations

- [#5298](https://github.com/biomejs/biome/pull/5298) [`68d1aa3`](https://github.com/biomejs/biome/commit/68d1aa3f5cfdddde52ae158c9c03d2d6de435f36) Thanks [@Pascalmh](https://github.com/Pascalmh)! - Allowed single spaces in `useConsistentCurlyBraces` rule.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4413](https://github.com/biomejs/biome/issues/4413): The GraphQL formatter no longer adds a new line at the start of block comments on Windows.

- [#5468](https://github.com/biomejs/biome/pull/5468) [`2b43f4a`](https://github.com/biomejs/biome/commit/2b43f4ab7e6d08e205f7a4e5e71ebfd1cbc4be37) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#5407](https://github.com/biomejs/biome/issues/5407). Now the `noUnusedImports` code fix correctly keeps top-level comments that were attached to lone imports.

- [#5780](https://github.com/biomejs/biome/pull/5780) [`837c98b`](https://github.com/biomejs/biome/commit/837c98b6c2307e2fd586783ecb9b49ec7be2bd30) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#3859](https://github.com/biomejs/biome/issues/3859): the `--skip-parse-errors` option is now applied to commands: `lint`, `check`, and `ci`.

- [#5841](https://github.com/biomejs/biome/pull/5841) [`042cd0d`](https://github.com/biomejs/biome/commit/042cd0d4cde48d8fce3255e6543039f9b84816cc) Thanks [@ematipico](https://github.com/ematipico)! - The `rage` command now prints the configuration path relative to the working directory, if applicable.

- [#5630](https://github.com/biomejs/biome/pull/5630) [`aa65304`](https://github.com/biomejs/biome/commit/aa653041532647830ba82a45fda9d1666a86b4d5) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#5606](https://github.com/biomejs/biome/issues/5606): We now correctly
  handle `.mjs` extensions in Node.js packages with `"type": "commonjs"`.

- [#4964](https://github.com/biomejs/biome/pull/4964) [`e750523`](https://github.com/biomejs/biome/commit/e750523d5b96e828246edd2945c3c09cff0de49b) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#1597](https://github.com/biomejs/biome/issues/1597): `useExhaustiveDependencies` no longer gets confused about the stability of dependencies by parentheses or type assertions.

- [#5338](https://github.com/biomejs/biome/pull/5338) [`82464aa`](https://github.com/biomejs/biome/commit/82464aabcec2a809008ed009db19569c51f9dff3) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#4751](https://github.com/biomejs/biome/issues/4751) by checking fragments inside `JSXElement` and conditional expressions.

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

- [#4863](https://github.com/biomejs/biome/pull/4863) [`846e4a4`](https://github.com/biomejs/biome/commit/846e4a4be6eb1f62c8b5baf52f98f8c1c39bdb9e) Thanks [@arendjr](https://github.com/arendjr)! - The rule `noFallthroughSwitchCase` no longer panics on some incomplete code snippets.

- [#5008](https://github.com/biomejs/biome/pull/5008) [`99f27a2`](https://github.com/biomejs/biome/commit/99f27a2b31fc15825a3175d342e2403e9764d22c) Thanks [@bushuai](https://github.com/bushuai)! - Fixed [#5007](https://github.com/biomejs/biome/issues/5007): Resolved false positives in `noMissingVarFunction` for `container-name`.

- [#4901](https://github.com/biomejs/biome/pull/4901) [`ba26e90`](https://github.com/biomejs/biome/commit/ba26e9003862a0ee4c202634729a5c70da1f6a31) Thanks [@bushuai](https://github.com/bushuai)! - Fixed [#4841](https://github.com/biomejs/biome/issues/4841): Shebang and top leading comments in `.cjs` files are now handled correctly

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

- [#6243](https://github.com/biomejs/biome/pull/6243) [`3cc1629`](https://github.com/biomejs/biome/commit/3cc162990364e82df3e091beed79cd5d3ee28558) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixes [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) missing dependencies being defined after the hook itself failure.

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

- [#4714](https://github.com/biomejs/biome/pull/4714) [`e3ec2e2`](https://github.com/biomejs/biome/commit/e3ec2e2cf494bc72d9097624dc610b5c984d5bd6) Thanks [@fireairforce](https://github.com/fireairforce)! - Fixed [#4714](https://github.com/biomejs/biome/pull/4714): Suppression comments no longer fail on functions that themselves contain suppression comments.

  This now works correctly:

  ```ts
  // biome-ignore lint/complexity/useArrowFunction: this suppression now works
  const foo0 = function (bar: string) {
    // biome-ignore lint/style/noParameterAssign: even if there are other suppressions inside
    bar = "baz";
  };
  ```

- [#5872](https://github.com/biomejs/biome/pull/5872) [`14551ba`](https://github.com/biomejs/biome/commit/14551bacd2fcdf434ddf310c6342eb72a38f22e6) Thanks [@drwpow](https://github.com/drwpow)! - Add @vitest/eslint-plugin to list of Biome rule sources

- [#6185](https://github.com/biomejs/biome/pull/6185) [`368d2c5`](https://github.com/biomejs/biome/commit/368d2c5a9736d968c8378f08858a79d52e7b73a0) Thanks [@mdevils](https://github.com/mdevils)! - Fixed `useHookAtTopLevel` rule to properly detect React components wrapped in `memo` and `forwardRef`, and correctly handle property accessors in control flow analysis.

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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - [noMisleadingCharacterClass](https://biomejs.dev/linter/rules/no-misleading-character-class/) no longer panics on malformed escape sequences that end with a multi-byte character ([#4587](https://github.com/biomejs/biome/issues/4587)).

- [#5130](https://github.com/biomejs/biome/pull/5130) [`0cbbbe6`](https://github.com/biomejs/biome/commit/0cbbbe62a9dd4de69ca6ff84952fb318acc3118c) Thanks [@siketyan](https://github.com/siketyan)! - Fixed the flag `--bracket-spacing` that was duplicated between the global configuration and the language-specific override for JavaScript.

- [#5909](https://github.com/biomejs/biome/pull/5909) [`7965780`](https://github.com/biomejs/biome/commit/7965780dcf44e8da6e91554ca6917f91b4408c2d) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#4715](https://github.com/biomejs/biome/issues/4715): The `useJsxKeyInIterable` rule now reports missing keys inside `switch` and `if` statements.

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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4121](https://github.com/biomejs/biome/issues/4326): The CSS formatter no longer indents a selector when it has leading comments.

- [#5099](https://github.com/biomejs/biome/pull/5099) [`9280cba`](https://github.com/biomejs/biome/commit/9280cbacbf429a4ab074e8890e6f7b1a85ae8e01) Thanks [@fireairforce](https://github.com/fireairforce)! - Fixed [#4982](https://github.com/biomejs/biome/issues/4982): the JavaScript parser now throws a syntax error for the following code:

  ```ts
  type T = import;
  type U = typeof import;
  ```

- [#5841](https://github.com/biomejs/biome/pull/5841) [`042cd0d`](https://github.com/biomejs/biome/commit/042cd0d4cde48d8fce3255e6543039f9b84816cc) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug with the `--verbose` CLI flag. Now the printed paths are **relative** to the working directory.

- [#5548](https://github.com/biomejs/biome/pull/5548) [`b8c2c47`](https://github.com/biomejs/biome/commit/b8c2c4708db3201c834457379fc2cb3a2a9fc795) Thanks [@dyc3](https://github.com/dyc3)! - Fixed [`noNoninteractiveElementToInteractiveRole`](https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role/) mistakenly flagging `<li role="treeitem">`,

- [#5198](https://github.com/biomejs/biome/pull/5198) [`b0046bf`](https://github.com/biomejs/biome/commit/b0046bf686be96271f3b0dbe005c89e187d6e676) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#4622](https://github.com/biomejs/biome/issues/4622): Our JavaScript parser can now gracefully handle situations where we detect the parser to have stalled.

  This means we don't fail with an assertion anymore, but invalid code can trigger a regular diagnostic in such cases.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now correctly handles invalid object member names, such as:

  ```js
  ({
    params: { [paramName: string]: number } = {}
  })
  ```

- [#6215](https://github.com/biomejs/biome/pull/6215) [`b1efce0`](https://github.com/biomejs/biome/commit/b1efce0caa9cdba3b4a540793d38b8f23a056bda) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#6211](https://github.com/biomejs/biome/issues/6211): previously the
  import organizer emitted broken code when it merged an import at the start of
  the file with another import and placed the merged result after a third import.

  The following code is now correctly organized:

  ```diff
  - import { B } from "bc";
  - import { C } from "bc";
    import { A } from "a";
  + import { B, C } from "bc";
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4334](https://github.com/biomejs/biome/issues/4334): The formatter no longer inserts trailing a comma inside dynamic `import` expressions.

- [#5681](https://github.com/biomejs/biome/pull/5681) [`57a240a`](https://github.com/biomejs/biome/commit/57a240aa3985df07087dc5aa317d2ea3dfb5d87d) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#5629](https://github.com/biomejs/biome/issues/5629): useHookAtTopLevel no longer report false-positives where the hook is at the top-level in a class method.

- [#5903](https://github.com/biomejs/biome/pull/5903) [`d8a99a8`](https://github.com/biomejs/biome/commit/d8a99a8cff4b4385bac80537ec8a7ee9d903dd3b) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#5900](https://github.com/biomejs/biome/issues/5900): `biome migrate eslint` now support a nested `files` property in ESLint flat configs.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3895](https://github.com/biomejs/biome/issues/3895): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) no longer reports used values imported as types in an external module.

- [#5550](https://github.com/biomejs/biome/pull/5550) [`cfec946`](https://github.com/biomejs/biome/commit/cfec94659f43e5f95de34cbda36bb73886d70dc9) Thanks [@dyc3](https://github.com/dyc3)! - Fixed a case where the code fix for `noUselessFragments` would remove more than just the fragment.

- [#5920](https://github.com/biomejs/biome/pull/5920) [`65e1267`](https://github.com/biomejs/biome/commit/65e126793c122c3161991aec57b39104d27fb00b) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#5919](https://github.com/biomejs/biome/issues/5919). Now Biome correctly loads the configuration passed via `--config-path` when its path starts with `./` e.g. `--confi-path=./project/biome.json`

- [#5052](https://github.com/biomejs/biome/pull/5052) [`1099147`](https://github.com/biomejs/biome/commit/109914706b4eed535e4c6aab4968f0cf46940f82) Thanks [@ah-yu](https://github.com/ah-yu)! - Fixed [#5031](https://github.com/biomejs/biome/issues/5031): CSS formatting has been improved for numbers:

  ```diff
  .class {
  -	padding: .5em;
  -	marding: 1.0;
  +	padding: 0.5em;
  +	marding: 1;
  }
  ```

- [#5992](https://github.com/biomejs/biome/pull/5992) [`bf35fe7`](https://github.com/biomejs/biome/commit/bf35fe7fe09593f324f94887fe029912df75b029) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#5989](https://github.com/biomejs/biome/issues/5989) where large octal escape sequences led to an overflow.

- [#5782](https://github.com/biomejs/biome/pull/5782) [`4a5ef84`](https://github.com/biomejs/biome/commit/4a5ef84930344ae54f3877da36888a954711f4a6) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Implement improved error handling for the supports at rule

- [#5066](https://github.com/biomejs/biome/pull/5066) [`56527db`](https://github.com/biomejs/biome/commit/56527db372a56a9c20df7a67bc9663667a7d32ae) Thanks [@ematipico](https://github.com/ematipico)! - Fix [#5053](https://github.com/biomejs/biome/issues/5053), now the rule correctly handles `console.log` inside arrow function expressions.

- [#6132](https://github.com/biomejs/biome/pull/6132) [`af5fbc4`](https://github.com/biomejs/biome/commit/af5fbc45625d6d46475bbc7fa717e0276887f974) Thanks [@fireairforce](https://github.com/fireairforce)! - Fix [#6105](https://github.com/biomejs/biome/issues/6105): css lint rules `useSortedProperties` should skip unknown properties.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3229](https://github.com/biomejs/biome/issues/3229): Made formatting of compound selectors more consistent.

- [#5794](https://github.com/biomejs/biome/pull/5794) [`107d632`](https://github.com/biomejs/biome/commit/107d6327bf59fdefe22f12d7808f9f165f0cc61a) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where passing `--max-diagnostics=0` would return a zero code even when errors were emitted.

- [#5544](https://github.com/biomejs/biome/pull/5544) [`344622b`](https://github.com/biomejs/biome/commit/344622b892170dc03701b6af2fbab17c6b532c57) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where Biome didn't report any error when `--stdin-file-path` didn't have any extension.
  Now Biome returns an error if `--stdin-file-path` doesn't have an extension.

- [#5781](https://github.com/biomejs/biome/pull/5781) [`5f6a375`](https://github.com/biomejs/biome/commit/5f6a3758323347a4319bfe8aa7ce6e58651b8199) Thanks [@Hideyasu-Ozawa](https://github.com/Hideyasu-Ozawa)! - Fixed [#5601](https://github.com/biomejs/biome/issues/5601): The [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) rule now properly preserves the original JSX quote style when sorting utility classes, preventing syntax errors.

- [#4998](https://github.com/biomejs/biome/pull/4998) [`f0e6521`](https://github.com/biomejs/biome/commit/f0e65211457ec71df17b041976665032079a2e03) Thanks [@mehm8128](https://github.com/mehm8128)! - The fix for `useSelfClosingElements` was marked as safe and the error message was improved.

- [#6236](https://github.com/biomejs/biome/pull/6236) [`89a9519`](https://github.com/biomejs/biome/commit/89a9519fa007577728363610f531e9e0b4c4c94a) Thanks [@dyc3](https://github.com/dyc3)! - Fixed overrides that include language-specific settings from having an effect for some languages

- [#6158](https://github.com/biomejs/biome/pull/6158) [`ff72658`](https://github.com/biomejs/biome/commit/ff72658ad7b6744bceea7e229ac08427d9bcde21) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#6144](https://github.com/biomejs/biome/issues/6144): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) reported incorrectly imports that were used as the type of parameters with the same name.
  In the following code, the import `name` was reported as unused.

  ```ts
  import name from "mod";
  function f(name: name.Readable): void {}
  ```

- [#5177](https://github.com/biomejs/biome/pull/5177) [`5b212f5`](https://github.com/biomejs/biome/commit/5b212f5e8c7410ef2f17f7ba3a001dde0ffc7c68) Thanks [@Conaclos](https://github.com/Conaclos)! - The lint rules [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) and [`useFilenamingConvention`](https://biomejs.dev/linter/rules/use-filenaming-convention/) now accept character escapes at the start of a regex group.

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

- [#5676](https://github.com/biomejs/biome/pull/5676) [`5f890fb`](https://github.com/biomejs/biome/commit/5f890fb432d138c167c096299cca8d4ee94ddb28) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#5617](https://github.com/biomejs/biome/issues/5617): [noDuplicateObjectKeys](https://biomejs.dev/linter/rules/no-duplicate-object-keys/) now transfers the leading comments of the removed member.

- [#5559](https://github.com/biomejs/biome/pull/5559) [`a444901`](https://github.com/biomejs/biome/commit/a4449018b9edb3cdb56e57c8c770d6ffb3781804) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#5409](https://github.com/biomejs/biome/issues/5409): [noParameterAssign](https://biomejs.dev/linter/rules/no-parameter-assign) now reports reassigned parameter of unparenthesized arrow functions.

  The following code is now reported as invalid.

  ```js
  const f = (param) => {
    param = {}; // Reassigning a function parameter is confusing.
  };
  ```

- [#5023](https://github.com/biomejs/biome/pull/5023) [`4d0a797`](https://github.com/biomejs/biome/commit/4d0a79769e50596ce3ebae76469a55307c2aca43) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4875](https://github.com/biomejs/biome/issues/4875): Relative file paths are now clickable in the Jetbrains IDE terminal.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4719](https://github.com/biomejs/biome/issues/4719): `bracketSameLine` now performs as expected when a comment is placed before the last JSX attribute.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4564](https://github.com/biomejs/biome/issues/4564): Biome no longer panics when a multi-byte character is found in a unicode escape sequence.

- [#5234](https://github.com/biomejs/biome/pull/5234) [`4634a8a`](https://github.com/biomejs/biome/commit/4634a8a2ca3877a4d838f74acef8210a0ab36b51) Thanks [@bushuai](https://github.com/bushuai)! - Fixed [#4950](https://github.com/biomejs/biome/issues/4950): Resolved a false positive of character class range operators in regular expressions.

- [#5935](https://github.com/biomejs/biome/pull/5935) [`6c4e24c`](https://github.com/biomejs/biome/commit/6c4e24ccc7927dc9ea547746d926b610233c2e1d) Thanks [@sterliakov](https://github.com/sterliakov)! - Fixed handling of top-level variables by `useExplicitType` rule ([#5932](https://github.com/biomejs/biome/issues/5932)). Biome now allows all variables with explicit annotations, as well as variables with trivial RHS. Biome no longer emits duplicated errors when an untyped function is assigned to an untyped variable.

- [#5085](https://github.com/biomejs/biome/pull/5085) [`65c5b7a`](https://github.com/biomejs/biome/commit/65c5b7a18d33f7e42f8c4a97bc7e95e710a8f341) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#4947](https://github.com/biomejs/biome/issues/4947): The `useTemplate` lint rule now ignores concatenated literals folded to multiple lines.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4568](https://github.com/biomejs/biome/issues/4568): Broken import statements no longer can cause a panic in `useExhaustiveDependencies`.

- [#6044](https://github.com/biomejs/biome/pull/6044) [`a82a1f2`](https://github.com/biomejs/biome/commit/a82a1f2e50c5c9b22cc696960bc167631d1de455) Thanks [@lucasweng](https://github.com/lucasweng)! - Fixed [#6042](https://github.com/biomejs/biome/pull/6042): [`noUselessEscapeInString`](https://next.biomejs.dev/linter/rules/no-useless-escape-in-string/) now reports useless escapes after skipping \${ in template literals.

- [#6282](https://github.com/biomejs/biome/pull/6282) [`19181d2`](https://github.com/biomejs/biome/commit/19181d23db55cd4bcc57a03a355f0b3db91cbfcf) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6229](https://github.com/biomejs/biome/issues/6229) where the fix of `noUnusedImports` emitted an invalid syntax. Now the following case emits a code fix that is syntactically correct:

  ```js
  import Used, { NotUsed } from "foo";

  Used();
  ```

- [#5695](https://github.com/biomejs/biome/pull/5695) [`6a82140`](https://github.com/biomejs/biome/commit/6a821409be26c75a86db043bbe2915309f07b87c) Thanks [@siketyan](https://github.com/siketyan)! - Fix [#5682](https://github.com/biomejs/biome/issues/5682): Object patterns with a nested assignment pattern no longer break properties.

  For example, the following code:

  ```js
  const { foo: { bar } = { bar: false } } = props;
  ```

  is used to be formatted into:

  ```js
  const { foo: { bar } = { bar: false } } = props;
  ```

  , while Prettier does not expand properties in this case.

- [#5648](https://github.com/biomejs/biome/pull/5648) [`230825d`](https://github.com/biomejs/biome/commit/230825d5b2b0901e55b17346f758fec0f0ae4be7) Thanks [@mdevils](https://github.com/mdevils)! - Fixed #5620, [noConsole](https://biomejs.dev/linter/rules/no-console/) rule now correctly handles indirect `console.log` calls and references.

- [#5268](https://github.com/biomejs/biome/pull/5268) [`c72de51`](https://github.com/biomejs/biome/commit/c72de51c24884f78c0225004efd4ebcd5ef43d34) Thanks [@ematipico](https://github.com/ematipico)! - When pulling code actions from the LSP, now the first choice suggested by the client will be the safe fix.

- [#6024](https://github.com/biomejs/biome/pull/6024) [`aa4749c`](https://github.com/biomejs/biome/commit/aa4749cf4b76f2a51d0eb47531563132ef4b9fe4) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#6022](https://github.com/biomejs/biome/issues/6022), now the rule `noDuplicateProperties` does't trigger properties defined inside the `@keyframes` at rule

- [#5497](https://github.com/biomejs/biome/pull/5497) [`7519ff7`](https://github.com/biomejs/biome/commit/7519ff7deab2b23eb7f00d9b96b1b0353723aaad) Thanks [@ematipico](https://github.com/ematipico)! - Enhanced the error message of the diagnostics emitted when Biome can't parse a suppression comment.

- [#5746](https://github.com/biomejs/biome/pull/5746) [`f9df3ba`](https://github.com/biomejs/biome/commit/f9df3bafe851763d71f591cdf894ec573ef70fdd) Thanks [@tyndria](https://github.com/tyndria)! - Fixed link to the docs inside CLI markup

- [#5776](https://github.com/biomejs/biome/pull/5776) [`4874007`](https://github.com/biomejs/biome/commit/4874007c15e88a1a9c0dd298db5a8f8a8b568d34) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a bug where a suppression comment with an empty explanation was valid.

  Now a suppression comment `// biome-ignore lint:` will raise a **warning** diagnostic.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4026](https://github.com/biomejs/biome/issues/4026): Comments in `grid-template` are no longer moved by the formatter.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3394](https://github.com/biomejs/biome/issues/3394): Resolved a false positive in `useSortedClasses`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#342](https://github.com/biomejs/biome/issues/342) and [#4562](https://github.com/biomejs/biome/issues/4562): Biome no longer crashes when a `declare` statement is followed by an unexpected token.

- [#5404](https://github.com/biomejs/biome/pull/5404) [`772dcf5`](https://github.com/biomejs/biome/commit/772dcf565d95f14e06bbd12a18afdf38ecdee4d6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed false positive in the rule [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function) where the [`tech`](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face/src#tech) function was incorrectly flagged as an unknown function.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4511](https://github.com/biomejs/biome/issues/4511): [noLabelWithoutControl](https://biomejs.dev/linter/rules/no-label-without-control/) now detects `<button>` tags as input.

- [#6042](https://github.com/biomejs/biome/pull/6042) [`014ee7d`](https://github.com/biomejs/biome/commit/014ee7d72fb224965664590d84fadd09f82470ed) Thanks [@lucasweng](https://github.com/lucasweng)! - Fixed [#6039](https://github.com/biomejs/biome/issues/6039): [`noUselessEscapeInString`](https://next.biomejs.dev/linter/rules/no-useless-escape-in-string/) no longer reports `\${` escape in template literals.

- [#5993](https://github.com/biomejs/biome/pull/5993) [`5013025`](https://github.com/biomejs/biome/commit/501302566eaa88258ba1f58b8a93aa7c6d61748d) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#5985](https://github.com/biomejs/biome/issues/5985), which caused the import organizer to fail the merging of a default import with a named import.
  The following code is now correctly organized:

  ```diff
  - import moment from 'moment';
  - import { Moment } from 'moment';
  + import moment, { Moment } from 'moment';
  ```

- [#6114](https://github.com/biomejs/biome/pull/6114) [`f21ade5`](https://github.com/biomejs/biome/commit/f21ade5aea6586b990f9c77c8f9c2f5e7532c3b5) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where the `explain` command didn't the diagnostic category when a rule was explained.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Added `RegExpStringIterator` to the analyzer globals.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4208](https://github.com/biomejs/biome/issues/4208): [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) now handles `JsxAttributeInitializerClause`, ensuring that fragments inside expressions like `<A b=<></> />` are preserved.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4533](https://github.com/biomejs/biome/issues/4533): `noUnknownPseudoClass` no longer reports pseudo classes after a webkit scrollbar pseudo element.

  The following code will no longer report a diagnostic:

  ```css
  ::-webkit-scrollbar-thumb:hover {
  }
  ```

- [#5773](https://github.com/biomejs/biome/pull/5773) [`e6c512a`](https://github.com/biomejs/biome/commit/e6c512a680dea1878f684d50dd1f52d6ed31a438) Thanks [@DerTimonius](https://github.com/DerTimonius)! - Updates the [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/) rule to more closely match the behavior of the ESLint plugin (e.g. mark the whole fragment as incorrect when no key is present). This also adds the option to check shorthand fragments (`<></>`)

- [#5403](https://github.com/biomejs/biome/pull/5403) [`adaa65c`](https://github.com/biomejs/biome/commit/adaa65ce06b7882accd60a97207ad4403bde23cf) Thanks [@ematipico](https://github.com/ematipico)! - Renamed the rule `noDuplicatedFields` to `noDuplicateFields`. Run the command `biome migrate` to update your configuration.

- [#6052](https://github.com/biomejs/biome/pull/6052) [`4309234`](https://github.com/biomejs/biome/commit/43092347c3815179316688b7cbca58802ad444fd) Thanks [@ematipico](https://github.com/ematipico)! - Fixed an issue where ignored files were incorrectly tracked by the Daemon.

- [#5118](https://github.com/biomejs/biome/pull/5118) [`afe2131`](https://github.com/biomejs/biome/commit/afe21317ba11380ac3a484bd43084bf1c41c2534) Thanks [@Th3S4mur41](https://github.com/Th3S4mur41)! - Fixed [#5116](https://github.com/biomejs/biome/issues/5116): [noUnknownPseudoElement](https://biomejs.dev/linter/rules/no-unknown-pseudo-element/) now supports `::slotted`.

- [#6192](https://github.com/biomejs/biome/pull/6192) [`b10cd49`](https://github.com/biomejs/biome/commit/b10cd492456bc867b9fe217b473cdb79b753de4a) Thanks [@unvalley](https://github.com/unvalley)! - Fixed [#5979](https://github.com/biomejs/biome/issues/5979): `biome search` now correctly skips files that don't match the pattern's target language.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4323](https://github.com/biomejs/biome/issues/4258): Fixed the case where `useSemanticElement` accidentally showed recommendations for `role="searchbox"` instead of `role="search"`.

- [#5713](https://github.com/biomejs/biome/pull/5713) [`b22713a`](https://github.com/biomejs/biome/commit/b22713aec07d12697e2255e009ba6c9f915d1e88) Thanks [@fireairforce](https://github.com/fireairforce)! - Support setting `indent_size` to `tab` in `.editorconfig`, the following config will not cause error:

  ```editorconfig
  root = true
  [*]
  indent_size = tab
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4565](https://github.com/biomejs/biome/issues/4565): [noControlCharactersInRegex](https://biomejs.dev/linter/rules/no-control-characters-in-regex) no longer panics when it encounters an unterminated unicode escape sequence.

- [#5795](https://github.com/biomejs/biome/pull/5795) [`d75326b`](https://github.com/biomejs/biome/commit/d75326b0e5249d5b8fd8cdfc90ff2c3958794f8e) Thanks [@bushuai](https://github.com/bushuai)! - Fixed [#5770](https://github.com/biomejs/biome/issues/5770), Biome's configuration file is now respected by the `migrate` command during migration

- [#6191](https://github.com/biomejs/biome/pull/6191) [`d42c38c`](https://github.com/biomejs/biome/commit/d42c38ca437f12a47651e98cf9108fff31860ac1) Thanks [@fireairforce](https://github.com/fireairforce)! - Fixed an issue where the lexer didn't report errors for unterminated regex or string literals, such as the following cases:

  ```js
  "string
  'str
  /\\217483
  ```

- [#5507](https://github.com/biomejs/biome/pull/5507) [`6fedec4`](https://github.com/biomejs/biome/commit/6fedec436c10a46eb7f2416d3dead656f7e6b7bd) Thanks [@unvalley](https://github.com/unvalley)! - The [`useKeyWithClickEvents`](https://biomejs.dev/linter/rules/use-key-with-click-events/) rule has been improved with better support for ARIA roles.

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

- [#6048](https://github.com/biomejs/biome/pull/6048) [`aecdbc7`](https://github.com/biomejs/biome/commit/aecdbc7a7cbba9e5a96ea52170c0365443a418a9) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#6029](https://github.com/biomejs/biome/issues/6029): A new line before the semicolon in the previous statement is now kept after formatting.

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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now reports all expressions using the `Array` constructors.

  Previously, the rule reported only use of the `Array` constructor in expressions statements.

  ```js
  // This was reported
  new Array();
  // This was not reported
  const xs = new Array();
  ```

- [#5244](https://github.com/biomejs/biome/pull/5244) [`02ea03f`](https://github.com/biomejs/biome/commit/02ea03f50cd65fbe345cbc66873849830d33e3c2) Thanks [@denbezrukov](https://github.com/denbezrukov)! - Improved error handling for the container at-rule.

- [#6189](https://github.com/biomejs/biome/pull/6189) [`426cee5`](https://github.com/biomejs/biome/commit/426cee5331dca8de6bef9d19b8815d54e954224f) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#4665](https://github.com/biomejs/biome/issues/4665): the LSP previously
  identified `.cjs` files as ESM files, making rules like `noRedundantUseStrict`
  reports incorrectly valid `"use strict"` directives.

- [#5396](https://github.com/biomejs/biome/pull/5396) [`e9e8267`](https://github.com/biomejs/biome/commit/e9e82674a1a294da75195b46705695b6e0f3e088) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#5382](https://github.com/biomejs/biome/issues/5382): `useExportType` no longer reports an identifier that bound by both a variable and a type.

- [#5834](https://github.com/biomejs/biome/pull/5834) [`18ce3d2`](https://github.com/biomejs/biome/commit/18ce3d2346554110be4cda17c4be8cc12406338e) Thanks [@lucasweng](https://github.com/lucasweng)! - Fixed [#5826](https://github.com/biomejs/biome/issues/5826): [`useNumericSeparators`](https://next.biomejs.dev/linter/rules/use-numeric-separators/) no longer reports single-digit `0`.

- [#5334](https://github.com/biomejs/biome/pull/5334) [`df48582`](https://github.com/biomejs/biome/commit/df485829fdb343dbdd1be1bd83d25c89610d17b5) Thanks [@wanghaoPolar](https://github.com/wanghaoPolar)! - Fixed [#5307](https://github.com/biomejs/biome/issues/5307), where CSS value lists were wrapped in a way that did not preserve semantic structure.

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

- [#4771](https://github.com/biomejs/biome/pull/4771) [`8d1062f`](https://github.com/biomejs/biome/commit/8d1062f45562df441acc8fc59e460c3f814e5f45) Thanks [@dyc3](https://github.com/dyc3)! - `tsconfig.*.json` files will now be treated the same as `tsconfig.json` files.

- [#6002](https://github.com/biomejs/biome/pull/6002) [`b67a138`](https://github.com/biomejs/biome/commit/b67a138229e4f4fb02d88abe0ad7876dfb8538cc) Thanks [@ematipico](https://github.com/ematipico)! - The `summary` reporter now prints the files processed and the files fixed when passing the `--verbose` flag.

- [#5716](https://github.com/biomejs/biome/pull/5716) [`ab2ae41`](https://github.com/biomejs/biome/commit/ab2ae4165ea2ba6e5b9e82b4d1a6ebb384325655) Thanks [@Conaclos](https://github.com/Conaclos)! - Fixed [#5693](https://github.com/biomejs/biome/issues/5693): [`useRegexLiterals`](https://biomejs.dev/linter/rules/use-regex-literals/) now correctly handle useless escaped character in string literals.

- [#5549](https://github.com/biomejs/biome/pull/5549) [`f32b7f0`](https://github.com/biomejs/biome/commit/f32b7f0bbf59c35fd3f48db01636d3f046949c36) Thanks [@Conaclos](https://github.com/Conaclos)! - [useRegexLiterals](https://biomejs.dev/linter/rules/use-regex-literals) now suggests a correct fix when the pattern contains an escaped anti-slash `\/`.

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

- [#6267](https://github.com/biomejs/biome/pull/6267) [`eafd7f4`](https://github.com/biomejs/biome/commit/eafd7f477103041cd7a3c4455f00d9bfda98a954) Thanks [@vladimir-ivanov](https://github.com/vladimir-ivanov)! - Fixed [`useConsistentCurlyBraces breaks react/no-unescaped-entities rule`](https://github.com/biomejs/biome/issues/5391)

  Added a check for forbidden characters: `>`, `"`, `'` and `}`.
  If any of these characters are detected, curly braces will be preserved.

  Example:

  ```jsx
  function MyComponent() {
    return <Foo>Jupiter {">"} Venus</Foo>;
  }
  ```

- [#4955](https://github.com/biomejs/biome/pull/4955) [`0bf4eaa`](https://github.com/biomejs/biome/commit/0bf4eaa83ba79710687d1f57c05b31f30e43538e) Thanks [@Conaclos](https://github.com/Conaclos)! - The rule [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now suggests a rename that preserves uppercase if possible.

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

- [#5938](https://github.com/biomejs/biome/pull/5938) [`7b298c2`](https://github.com/biomejs/biome/commit/7b298c2f838bfc1ee05afce91a1aac553e88597e) Thanks [@fireairforce](https://github.com/fireairforce)! - Fix a parsing error when a `JsxElementName` is `JsxMemberExpression`, and a `JsLogicalExpreesion` before it without a semicolon.

  The following case will now not throw error:

  ```jsx
  import React from "react";

  let b = 0;

  function A() {
    const a = b > 0 && b < 1;

    return <React.Fragment>{a}</React.Fragment>;
  }
  ```

- [#5067](https://github.com/biomejs/biome/pull/5067) [`7243cce`](https://github.com/biomejs/biome/commit/7243cce87617988a09527c09e7296238e79de8ed) Thanks [@dyc3](https://github.com/dyc3)! - Fixed Biome being unable to parse `insert_final_newline = unset` in EditorConfig files.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4530](https://github.com/biomejs/biome/issues/4530): [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/) now preserves directives.

  Previously the rule removed the directives when a function expression was turned into an arrow function.
  The rule now correctly keeps the directives.

  ```diff
  - const withDirective = function () {
  + const withDirective = () => {
      "use server";
      return 0;
    }
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4855](https://github.com/biomejs/biome/issues/4855): [useSortedClasses](https://biomejs.dev/linter/rules/use-sorted-classes/) now suggests code fixes that match the JSX quote style of the formatter.
