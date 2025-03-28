# @biomejs/biome

## 2.0.0

### Major Changes

- [#4823](https://github.com/biomejs/biome/pull/4823) [`d3b7b2d`](https://github.com/biomejs/biome/commit/d3b7b2d734a3cb40b7e75c65d9e04b1a7f30f2fb) Thanks [@ematipico](https://github.com/ematipico)! - Biome now resolves globs and paths from the configuration. Before, paths and globs were resolved from the working directory.

- [#5235](https://github.com/biomejs/biome/pull/5235) [`7037c0f`](https://github.com/biomejs/biome/commit/7037c0f56491676709face0d13b791fba2f818ec) Thanks [@siketyan](https://github.com/siketyan)! - Removed the `--config-path` argument from the `biome lsp-proxy` and `biome start` commands.

  The option was overriding the configuration path for all workspaces opened in the Biome daemon, which led to a configuration mismatch problem when multiple projects are opened in some editors or IDEs.

  If you are using one of our official plugins for IDEs or editors, it is recommended to update it to the latest version of the plugin, or you will get unexpected behavior.

  If you are a developer of a plugin, please update your plugin to use the `workspace/configuration` response instead of using the `--config-path` argument. Biome's LSP will resolve a configuration in the workspace automatically, so it is recommended to keep it empty unless you are using a custom configuration path.

- [#5226](https://github.com/biomejs/biome/pull/5226) [`983ab6f`](https://github.com/biomejs/biome/commit/983ab6f0792af23a7d6abe68b262ea1423759635) Thanks [@Conaclos](https://github.com/Conaclos)! - Previously the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` checked both regular expression literals like `/regex/` and dynamically built regular expressions like `new RegExp("regex")`.

  Checking dynamically built regular expressions has many limitations, edge cases, and complexities.
  In addition, other rules that lint regular expressions don't check dynamically built regular expressions.

  Rather than add support for other rules and have half-baked checking, we decided to remove support for dynamically built regular expressions.

  Now the lint rules `noControlCharactersInRegex` and `noMisleadingCharacterClass` only check literals of regular expressions.

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

- [#5127](https://github.com/biomejs/biome/pull/5127) [`baf0927`](https://github.com/biomejs/biome/commit/baf09274e4017f0cef8df5452789788a1dcdfaed) Thanks [@fireairforce](https://github.com/fireairforce)! - Enable `.editorconfig` by default, default value of [`formatter.useEditorconfig`](https://biomejs.dev/reference/configuration/#formatteruseeditorconfig) set to `true`.

  It will follow the following rules:

  - Formatting settings in `biome.json` always take precedence over `.editorconfig` files.
  - `.editorconfig` files that exist higher up in the hierarchy than a `biome.json` file are already ignored. This is to avoid loading formatting settings from someone's home directory into a project with a `biome.json` file.
  - Nested `.editorconfig` files aren't supported.

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

- [#4730](https://github.com/biomejs/biome/pull/4730) [`a478377`](https://github.com/biomejs/biome/commit/a478377129491ee20acd5e122228994a062b8b00) Thanks [@ematipico](https://github.com/ematipico)! - The `style` rules aren't recommended anymore.

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

- [#5351](https://github.com/biomejs/biome/pull/5351) [`07775c7`](https://github.com/biomejs/biome/commit/07775c7a2c4aa12b15d8fd958e0d195f84724aac) Thanks [@ematipico](https://github.com/ematipico)! - Renamed the rule `noMultipleSpacesInRegularExpressionLiterals` to `noAdjacentSpacesInRegex`.

  Use `biome migrate --write` to update the configuration.

### Minor Changes

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

- [#5076](https://github.com/biomejs/biome/pull/5076) [`279311a`](https://github.com/biomejs/biome/commit/279311a213e71f1b5f7dca5c04459076065ef47e) Thanks [@siketyan](https://github.com/siketyan)! - Add a new lint rule `noConstantBinaryExpression`.
  This rule is inspired from ESLint's [no-constant-binary-expression](https://eslint.org/docs/latest/rules/no-constant-binary-expression) rule.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3574](https://github.com/biomejs/biome/issues/3574): `noUnusedImports` now reports empty named imports and suggests their removal.

  The rule now suggests the removal of empty named imports such as:

  ```diff
  - import {} from "mod";
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

- [#5083](https://github.com/biomejs/biome/pull/5083) [`7aa79e7`](https://github.com/biomejs/biome/commit/7aa79e79268fd5450dc791838e91812c59220ca2) Thanks [@siketyan](https://github.com/siketyan)! - The formatter option `bracketSpacing` is now also supported in JSON files.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - `useValidTypeof` now accepts comparisons with variables.

  Previously, the rule required to compare a `typeof` expression against another `typeof` expression or a valid string literal. We now accept more cases, notably comparison against a variable:

  ```js
  if (typeof foo === bar) {
    // ...
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

- [#5230](https://github.com/biomejs/biome/pull/5230) [`7c1e505`](https://github.com/biomejs/biome/commit/7c1e505aea0f7abb7ff4f7eb73357633daf09a1d) Thanks [@tim-we](https://github.com/tim-we)! - Added options to `suspicious/noConfusingLabels` to allow specific labels.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4549](https://github.com/biomejs/biome/issues/4549): [noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property/) now accepts more known CSS properties.

  ```diff
  - ['anchor-default', 'anchor-scroll', 'inset-area', 'position-animation', 'position-fallback', 'position-fallback-bounds', 'position-try-options']
  + ['anchor-scope', 'interpolate-size', 'line-fit-edge', 'masonry', 'masonry-auto-tracks', 'masonry-direction', 'masonry-fill', 'masonry-flow', 'masonry-slack', 'masonry-template-areas', 'masonry-template-tracks', 'position-anchor', 'position-area', 'position-try-fallbacks', 'position-visibility', 'scroll-start-target', 'text-box', 'view-transition-class', 'view-transition-group']
  ```

  This change replaces deprecated properties, improving CSS validation.

- [#5093](https://github.com/biomejs/biome/pull/5093) [`766492f`](https://github.com/biomejs/biome/commit/766492f3535aef3a0a9bef47d607d07f8b70c749) Thanks [@siketyan](https://github.com/siketyan)! - LSP clients can now override the configuration path for each workspace, by responding to
  `workspace/configuration` requests.

- [#4796](https://github.com/biomejs/biome/pull/4796) [`e7dd706`](https://github.com/biomejs/biome/commit/e7dd706b93b44c5febe6710ba0cfa3b6365fccaf) Thanks [@MaxtuneLee](https://github.com/MaxtuneLee)! - Biome now emits a warning diagnostic if the configuration contains an out-of-sync schema URL.

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

- [#5337](https://github.com/biomejs/biome/pull/5337) [`bab955b`](https://github.com/biomejs/biome/commit/bab955bc52feb61259e1dd0d696843aa3aa30fc6) Thanks [@ematipico](https://github.com/ematipico)! - Added the new CLI option called `--threads` to the `ci` command. It allows to control the numbers of threads that can be used when using the Biome CLI.

  It's possible to use the environment variable `BIOME_THREADS` as an alternatives.

  This feature is useful when running the CLI in environments that have limited resources, for example CI/CD.

  ```shell
  biome ci --threads=1
  BIOME_THREADS=1 biome ci
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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4416](https://github.com/biomejs/biome/pull/4416): The rules [`useExportType`](https://biomejs.dev/linter/rules/use-export-type/) and [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now ignore TypeScript declaration files.

- [#5203](https://github.com/biomejs/biome/pull/5203) [`d95df40`](https://github.com/biomejs/biome/commit/d95df40a86c8debb369fdc9070c91642325bfe1f) Thanks [@fireairforce](https://github.com/fireairforce)! - Added the new rule [`useForComponent`](https://biomejs.dev/linter/rules/use-for-component/).

  This rule enforces usage of Solid's `<For />` component for mapping an array to JSX elements.

- [#4760](https://github.com/biomejs/biome/pull/4760) [`d469189`](https://github.com/biomejs/biome/commit/d469189298a2358989ee7e906b840f1d30fe5ad5) Thanks [@ematipico](https://github.com/ematipico)! - The package now requires `v2` of the WebAssembly packages. The internal APIs of Workspace are now `camelCase`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - The rule [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now provides a code fix.

  ```diff
  - const xs = new Array();
  + const xs = [];
  ```

  The code fix is currently marked as unsafe.
  We plan to make it safe in a future release of Biome.

- [#5129](https://github.com/biomejs/biome/pull/5129) [`95a5407`](https://github.com/biomejs/biome/commit/95a54070c73d3e20d96979e247f841b649b47362) Thanks [@unvalley](https://github.com/unvalley)! - Added the new lint rule [`noAwaitInLoop`](https://biomejs.dev/linter/rules/no-await-in-loop).

### Patch Changes

- [#5014](https://github.com/biomejs/biome/pull/5014) [`028af9c`](https://github.com/biomejs/biome/commit/028af9c89af4ac62089907e5523584bef47639f9) Thanks [@vohoanglong0107](https://github.com/vohoanglong0107)! - Fix [#5001](https://github.com/biomejs/biome/issues/5001), where the CSS formatter removes whitespace from selector preceded by a comment

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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - `biome migrate eslint` now correctly resolves the scoped package named `eslint-config`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3836](https://github.com/biomejs/biome/issues/3836): The CSS parser will now correctly parse the following:

  ```css
  .foo {
    color: red;
  }
  ```

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

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4756](https://github.com/biomejs/biome/issues/4756): `noDuplicateProperties` now throws lint errors properly when we use `@supports`.

- [#5174](https://github.com/biomejs/biome/pull/5174) [`5f7dc3f`](https://github.com/biomejs/biome/commit/5f7dc3f0639a6f660d49e137997c50948dfe8353) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#2406](https://github.com/biomejs/biome/issues/2406): Biome longer expands properties of object type annotations in the only function parameter to align with Prettier.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4740](https://github.com/biomejs/biome/issues/4740): `biome migrate eslint` now correctly handles ESLint configuration with `null` values in file lists.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4202](https://github.com/biomejs/biome/issues/4202): Align with Prettier in formatting test functions.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now properly handles unterminated string literals, such as:

  ```jsx
  function Comp() {
    return (
        <a rel="
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed a CSS parser error: `@-moz-document url-prefix(https://example.com)` and `@-moz-document domain(example.com)` are now valid.

- [#5170](https://github.com/biomejs/biome/pull/5170) [`890d31b`](https://github.com/biomejs/biome/commit/890d31b18a883c128838d756a665345d98aa02b7) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#4967](https://github.com/biomejs/biome/issues/4967): The fix for `useArrowFunction` no longer breaks function bodies starting with `{`.

- [#5043](https://github.com/biomejs/biome/pull/5043) [`3868597`](https://github.com/biomejs/biome/commit/386859758287739ff00e7b0d9faa53ab9adb62af) Thanks [@Jayllyz](https://github.com/Jayllyz)! - Fixed [#5024](https://github.com/biomejs/biome/issues/5024): Added `useJsxKeyInIterable` rule to React domain.

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

- [#4714](https://github.com/biomejs/biome/pull/4714) [`e3ec2e2`](https://github.com/biomejs/biome/commit/e3ec2e2cf494bc72d9097624dc610b5c984d5bd6) Thanks [@fireairforce](https://github.com/fireairforce)! - Fixed [#4714](https://github.com/biomejs/biome/pull/4714): Suppression comments no longer fail on functions that themselves contain suppression comments.

  This now works correctly:

  ```ts
  // biome-ignore lint/complexity/useArrowFunction: this suppression now works
  const foo0 = function (bar: string) {
    // biome-ignore lint/style/noParameterAssign: even if there are other suppressions inside
    bar = "baz";
  };
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - [noMisleadingCharacterClass](https://biomejs.dev/linter/rules/no-misleading-character-class/) no longer panics on malformed escape sequences that end with a multi-byte character ([#4587](https://github.com/biomejs/biome/issues/4587)).

- [#5130](https://github.com/biomejs/biome/pull/5130) [`0cbbbe6`](https://github.com/biomejs/biome/commit/0cbbbe62a9dd4de69ca6ff84952fb318acc3118c) Thanks [@siketyan](https://github.com/siketyan)! - Fixed the flag `--bracket-spacing` that was duplicated between the global configuration and the language-specific override for JavaScript.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4121](https://github.com/biomejs/biome/issues/4326): The CSS formatter no longer indents a selector when it has leading comments.

- [#5099](https://github.com/biomejs/biome/pull/5099) [`9280cba`](https://github.com/biomejs/biome/commit/9280cbacbf429a4ab074e8890e6f7b1a85ae8e01) Thanks [@fireairforce](https://github.com/fireairforce)! - Fixed [#4982](https://github.com/biomejs/biome/issues/4982): the JavaScript parser now throws a syntax error for the following code:

  ```ts
  type T = import;
  type U = typeof import;
  ```

- [#5198](https://github.com/biomejs/biome/pull/5198) [`b0046bf`](https://github.com/biomejs/biome/commit/b0046bf686be96271f3b0dbe005c89e187d6e676) Thanks [@arendjr](https://github.com/arendjr)! - Fixed [#4622](https://github.com/biomejs/biome/issues/4622): Our JavaScript parser can now gracefully handle situations where we detect the parser to have stalled.

  This means we don't fail with an assertion anymore, but invalid code can trigger a regular diagnostic in such cases.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now correctly handles invalid object member names, such as:

  ```js
  ({
    params: { [paramName: string]: number } = {}
  })
  ```

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4334](https://github.com/biomejs/biome/issues/4334): The formatter no longer inserts trailing a comma inside dynamic `import` expressions.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3895](https://github.com/biomejs/biome/issues/3895): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) no longer reports used values imported as types in an external module.

- [#5052](https://github.com/biomejs/biome/pull/5052) [`1099147`](https://github.com/biomejs/biome/commit/109914706b4eed535e4c6aab4968f0cf46940f82) Thanks [@ah-yu](https://github.com/ah-yu)! - Fixed [#5031](https://github.com/biomejs/biome/issues/5031): CSS formatting has been improved for numbers:

  ```diff
  .class {
  -	padding: .5em;
  -	marding: 1.0;
  +	padding: 0.5em;
  +	marding: 1;
  }
  ```

- [#5066](https://github.com/biomejs/biome/pull/5066) [`56527db`](https://github.com/biomejs/biome/commit/56527db372a56a9c20df7a67bc9663667a7d32ae) Thanks [@ematipico](https://github.com/ematipico)! - Fix [#5053](https://github.com/biomejs/biome/issues/5053), now the rule correctly handles `console.log` inside arrow function expressions.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3229](https://github.com/biomejs/biome/issues/3229): Made formatting of compound selectors more consistent.

- [#4998](https://github.com/biomejs/biome/pull/4998) [`f0e6521`](https://github.com/biomejs/biome/commit/f0e65211457ec71df17b041976665032079a2e03) Thanks [@mehm8128](https://github.com/mehm8128)! - The fix for `useSelfClosingElements` was marked as safe and the error message was improved.

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

- [#5023](https://github.com/biomejs/biome/pull/5023) [`4d0a797`](https://github.com/biomejs/biome/commit/4d0a79769e50596ce3ebae76469a55307c2aca43) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4875](https://github.com/biomejs/biome/issues/4875): Relative file paths are now clickable in the Jetbrains IDE terminal.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4719](https://github.com/biomejs/biome/issues/4719): `bracketSameLine` now performs as expected when a comment is placed before the last JSX attribute.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4564](https://github.com/biomejs/biome/issues/4564): Biome no longer panics when a multi-byte character is found in a unicode escape sequence.

- [#5234](https://github.com/biomejs/biome/pull/5234) [`4634a8a`](https://github.com/biomejs/biome/commit/4634a8a2ca3877a4d838f74acef8210a0ab36b51) Thanks [@bushuai](https://github.com/bushuai)! - Fixed [#4950](https://github.com/biomejs/biome/issues/4950): Resolved a false positive of character class range operators in regular expressions.

- [#5085](https://github.com/biomejs/biome/pull/5085) [`65c5b7a`](https://github.com/biomejs/biome/commit/65c5b7a18d33f7e42f8c4a97bc7e95e710a8f341) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#4947](https://github.com/biomejs/biome/issues/4947): The `useTemplate` lint rule now ignores concatenated literals folded to multiple lines.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4568](https://github.com/biomejs/biome/issues/4568): Broken import statements no longer can cause a panic in `useExhaustiveDependencies`.

- [#5268](https://github.com/biomejs/biome/pull/5268) [`c72de51`](https://github.com/biomejs/biome/commit/c72de51c24884f78c0225004efd4ebcd5ef43d34) Thanks [@ematipico](https://github.com/ematipico)! - When pulling code actions from the LSP, now the first choice suggested by the client will be the safe fix.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4026](https://github.com/biomejs/biome/issues/4026): Comments in `grid-template` are no longer moved by the formatter.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#3394](https://github.com/biomejs/biome/issues/3394): Resolved a false positive in `useSortedClasses`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#342](https://github.com/biomejs/biome/issues/342) and [#4562](https://github.com/biomejs/biome/issues/4562): Biome no longer crashes when a `declare` statement is followed by an unexpected token.

- [#5404](https://github.com/biomejs/biome/pull/5404) [`772dcf5`](https://github.com/biomejs/biome/commit/772dcf565d95f14e06bbd12a18afdf38ecdee4d6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed false positive in the rule [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function) where the [`tech`](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face/src#tech) function was incorrectly flagged as an unknown function.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4511](https://github.com/biomejs/biome/issues/4511): [noLabelWithoutControl](https://biomejs.dev/linter/rules/no-label-without-control/) now detects `<button>` tags as input.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Added `RegExpStringIterator` to the analyzer globals.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4208](https://github.com/biomejs/biome/issues/4208): [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) now handles `JsxAttributeInitializerClause`, ensuring that fragments inside expressions like `<A b=<></> />` are preserved.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4533](https://github.com/biomejs/biome/issues/4533): `noUnknownPseudoClass` no longer reports pseudo classes after a webkit scrollbar pseudo element.

  The following code will no longer report a diagnostic:

  ```css
  ::-webkit-scrollbar-thumb:hover {
  }
  ```

- [#5403](https://github.com/biomejs/biome/pull/5403) [`adaa65c`](https://github.com/biomejs/biome/commit/adaa65ce06b7882accd60a97207ad4403bde23cf) Thanks [@ematipico](https://github.com/ematipico)! - Renamed the rule `noDuplicatedFields` to `noDuplicateFields`. This rules belongs to the `nursery` group, so no migration is provided.

- [#5118](https://github.com/biomejs/biome/pull/5118) [`afe2131`](https://github.com/biomejs/biome/commit/afe21317ba11380ac3a484bd43084bf1c41c2534) Thanks [@Th3S4mur41](https://github.com/Th3S4mur41)! - Fixed [#5116](https://github.com/biomejs/biome/issues/5116): [noUnknownPseudoElement](https://biomejs.dev/linter/rules/no-unknown-pseudo-element/) now supports `::slotted`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4323](https://github.com/biomejs/biome/issues/4258): Fixed the case where `useSemanticElement` accidentally showed recommendations for `role="searchbox"` instead of `role="search"`.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - Fixed [#4565](https://github.com/biomejs/biome/issues/4565): [noControlCharactersInRegex](https://biomejs.dev/linter/rules/no-control-characters-in-regex) no longer panics when it encounters an unterminated unicode escape sequence.

- [#5044](https://github.com/biomejs/biome/pull/5044) [`bff5068`](https://github.com/biomejs/biome/commit/bff5068451507665b3bb2f9ea15bae9987d8aad6) Thanks [@ematipico](https://github.com/ematipico)! - [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now reports all expressions using the `Array` constructors.

  Previously, the rule reported only use of the `Array` constructor in expressions statements.

  ```js
  // This was reported
  new Array();
  // This was not reported
  const xs = new Array();
  ```

- [#5396](https://github.com/biomejs/biome/pull/5396) [`e9e8267`](https://github.com/biomejs/biome/commit/e9e82674a1a294da75195b46705695b6e0f3e088) Thanks [@siketyan](https://github.com/siketyan)! - Fixed [#5382](https://github.com/biomejs/biome/issues/5382): `useExportType` no longer reports an identifier that bound by both a variable and a type.

- [#4771](https://github.com/biomejs/biome/pull/4771) [`8d1062f`](https://github.com/biomejs/biome/commit/8d1062f45562df441acc8fc59e460c3f814e5f45) Thanks [@dyc3](https://github.com/dyc3)! - `tsconfig.*.json` files will now be treated the same as `tsconfig.json` files.

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
