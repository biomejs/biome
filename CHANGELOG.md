# Biome changelog

This project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Due to the nature of Biome as a toolchain,
it can be unclear what changes are considered major, minor, or patch.
Read our [guidelines](https://biomejs.dev/internals/versioning) to categorize a change.

New entries must be placed in a section entitled `Unreleased`.

## [Unreleased]

### Analyzer

#### BREAKING CHANGES

- The organize imports feature now groups import statements by "distance".
  Modules "farther" from the user are put on the top, and modules "closer" to the user are placed on the bottom.
  Check the [documentation](https://docs.rome.tools/analyzer/) for more information about it.
- The organize imports tool is enabled by default. If you don't want to use it, you need to disable it explicitly:
  ```json
  {
    "organizeImports": {
      "enabled": false
    }
  }
  ```


### CLI

#### BREAKING CHANGES

- The CLI now exists with an error when there's an error inside the configuration.

  Previously, rome would raise warnings and continue the execution by applying its defaults.

  This could have been better for users because this could have created false positives in linting or formatted
  code with a configuration that wasn't the user's.
- The command `rome check` now shows formatter diagnostics when checking the code.

  The diagnostics presence will result in an error code when the command finishes.

  This aligns with semantic and behaviour meant for the command `rome check`.

#### Other changes

- Fix [#4670](https://github.com/rome/tools/issues/4670), don't crash at empty default export.
- Fix [#4556](https://github.com/rome/tools/issues/4556), which correctly handles new lines in the
  `.gitignore` file across OS.
- Add a new option to ignore unknown files `--files-ignore-unknown`:

    ```shell
    rome format --files-ignore-unknown ./src
    ```

  Doing so, Biome won't emit diagnostics for files that doesn't know how to handle.
- Add a new option `--no-errors-on-unmatched`:

    ```shell
    rome format --no-errors-on-unmatched ./src
    ```

  Biome doesn't exit with an error code if no files were processed in the given paths.

- Fixed the diagnostics emitted when running the `rome format` command;

- Biome no longer warns when discovering (possibly infinite) symbolic links between directories.
  This fixes [#4193](https://github.com/rome/tools/issues/4193) which resulted in incorrect warnings
  when a single file or directory was pointed at by multiple symbolic links. Symbolic links to other
  symbolic links do still trigger warnings if they are too deeply nested.
- Introduced a new command called `rome lint`, which will only run lint rules against the code base.
- Biome recognises known files as "JSON files with comments allowed":
  - `typescript.json`;
  - `tsconfig.json`;
  - `jsconfig.json`;
  - `tslint.json`;
  - `babel.config.json`;
  - `.babelrc.json`;
  - `.ember-cli`;
  - `typedoc.json`;
  - `.eslintrc.json`;
  - `.eslintrc`;
  - `.jsfmtrc`;
  - `.jshintrc`;
  - `.swcrc`;
  - `.hintrc`;
  - `.babelrc`;

### Configuration

#### Other changes

- Add a new option to ignore unknown files:

    ```json
    {
       "files": {
          "ignoreUnknown": true
       }
    }
    ```
  Doing so, Biome won't emit diagnostics for file that it doesn't know how to handle.

- Add a new `"javascript"` option to support the unsafe/experimental
  parameter decorators:

    ```json
    {
       "javascript": {
          "parser": {
             "unsafeParameterDecoratorsEnabled": true
          }
       }
    }
    ```
- Add a new `"extends"` option, useful to split the configuration file in
  multiple files:

  ```json
  {
    "extends": ["../sharedFormatter.json", "linter.json"]
  }
  ```

  The resolution of the files is file system based, Biome doesn't know how to
  resolve dependencies yet.

- The commands `rome check` and `rome lint` now show the remaining diagnostics even when
  `--apply-safe` or `--apply-unsafe` are passed.

- Fix the commands `rome check` and `rome lint`, they won't exit with an error code
  if no error diagnostics are emitted.

- Add a new option `--error-on-warnings`, which instructs Biome to exit with an error code when warnings are emitted.

  ```shell
  rome check --error-on-wanrings ./src
  ```

- Add a configuration to enable parsing comments inside JSON files:

  ```json
  {
    "json": {
      "parser": {
        "allowComments": true
      }
    }
  }
  ```

### Editors

#### Other changes

- The Biome LSP can now show diagnostics belonging to JSON lint rules.
- Fix [#4564](https://github.com/rome/tools/issues/4564); files too large don't emit errors.
- The Biome LSP sends client messages when files are ignored or too big.

### Formatter

- Add a new option called `--jsx-quote-style` to the formatter. This option lets you choose between single and double quotes for JSX attributes. [#4486](https://github.com/rome/tools/issues/4486)

- Add an option called `--arrow-parentheses` to the formatter. This option allows you to set the parentheses style for arrow functions. [#4666](https://github.com/rome/tools/issues/4666)

- The JSON formatter can now format `.json` files with comments.

### Linter

- [`noDuplicateParameters`](https://docs.rome.tools/lint/rules/noduplicateparameters/): enhanced rule to manage constructor parameters.

#### Removed rules

- Remove `complexity/noExtraSemicolon` ([#4553](https://github.com/rome/tools/issues/4553))

  The _Biome_ formatter takes care of removing extra semicolons.
  Thus, there is no need for this rule.

- Remove `useCamelCase`

  Use [`useNamingConvention`](https://docs.rome.tools/lint/rules/useCamelCase/) instead.

#### New rules

- Add [`noExcessiveComplexity`](https://docs.rome.tools/lint/rules/noExcessiveComplexity/)
- Add [`useImportRestrictions`](https://docs.rome.tools/lint/rules/useImportRestrictions/)
- Add [`noFallthroughSwitchClause`](https://docs.rome.tools/lint/rules/noFallthroughSwitchClause/)

- Add [`noGlobalIsFinite`](https://docs.rome.tools/lint/rules/noglobalisfinite/)

  This rule recommends using `Number.isFinite` instead of the global and unsafe `isFinite` that attempts a type coercion.

- Add [`noGlobalIsNan`](https://docs.rome.tools/lint/rules/noglobalisnan/)

  This rule recommends using `Number.isNaN` instead of the global and unsafe `isNaN` that attempts a type coercion.

- Add [`noUnsafeDeclarationMerging`](https://docs.rome.tools/lint/rules/noUnsafeDeclarationMerging/)

  This rule disallows declaration merging between an interface and a class.

- Add [`useArrowFunction`](https://docs.rome.tools/lint/rules/usearrowfunction/)

  This rule proposes turning function expressions into arrow functions.
  Function expressions that use `this` are ignored.

- Add [`noDuplicateJsonKeys`](https://docs.rome.tools/lint/rules/noDuplicateJsonKeys/)

  This rule disallow duplicate keys in a JSON object.

- Add [`noVoid`](https://docs.rome.tools/lint/rules/novoid/)

  This rules disallow the use of `void`.

- Add [`noNonoctalDecimalEscape`](https://docs.rome.tools/lint/rules/nononoctaldecimalescape/)

  This rule disallows `\8` and `\9` escape sequences in string literals.

- Add [`noUselessEmptyExport`](https://docs.rome.tools/lint/rules/noUselessEmptyExport/)

  This rule disallows useless `export {}`.

- Add [`useIsArray`](https://docs.rome.tools/lint/rules/useIsArray/)

  This rule proposes using `Array.isArray()` instead of `instanceof Array`.

#### Promoted rules

New rules are promoted, please check [#4750](https://github.com/rome/tools/discussions/4750) for more details:

- [`a11y/useHeadingContent`](https://docs.rome.tools/lint/rules/useHeadingContent/)
- [`complexity/noForEach`](https://docs.rome.tools/lint/rules/noForEach/)
- [`complexity/useLiteralKeys`](https://docs.rome.tools/lint/rules/useLiteralKeys/)
- [`complexity/useSimpleNumberKeys`](https://docs.rome.tools/lint/rules/useSimpleNumberKeys/)
- [`correctness/useIsNan`](https://docs.rome.tools/lint/rules/useIsNan/)
- [`suspicious/noConsoleLog`](https://docs.rome.tools/lint/rules/noConsoleLog/)
- [`suspicious/noDuplicateJsxProps`](https://docs.rome.tools/lint/rules/noDuplicateJsxProps/)

The following rules are now recommended:

- [`noUselessFragments`](https://docs.rome.tools/lint/rules/noUselessFragments/)
- [`noRedundantUseStrict`](https://docs.rome.tools/lint/rules/noRedundantUseStrict/)
- [`useExponentiationOperator`](https://docs.rome.tools/lint/rules/useExponentiationOperator/)


#### Other changes

- Add new TypeScript globals (`AsyncDisposable`, `Awaited`, `DecoratorContext`, and others) [4643](https://github.com/rome/tools/issues/4643).

- [`noRedeclare`](https://docs.rome.tools/lint/rules/noredeclare/): allow redeclare of index signatures are in different type members [#4478](https://github.com/rome/tools/issues/4478)

- Improve [`noConsoleLog`](https://docs.rome.tools/lint/rules/noconsolelog/), [`noGlobalObjectCalls`](https://docs.rome.tools/lint/rules/noglobalobjectcalls/), [`useIsNan`](https://docs.rome.tools/lint/rules/useisnan/), and [`useNumericLiterals`](https://docs.rome.tools/lint/rules/usenumericliterals/) by handling `globalThis` and `window` namespaces.

  For instance, the following code is now reported by `noConsoleLog`:

  ```js
  globalThis.console.log("log")
  ```

- Fix a crash in the [`NoParameterAssign`](https://docs.rome.tools/lint/rules/noparameterassign/) rule that occurred when there was a bogus binding. [#4323](https://github.com/rome/tools/issues/4323)

- Fix [`useExhaustiveDependencies`](https://docs.rome.tools/lint/rules/useexhaustivedependencies/) rule in the following cases [#4330](https://github.com/rome/tools/issues/4330)
    - when the first argument of hooks is a named function
    - inside an export default function
    - for React.use* hooks

- Fix [`noInvalidConstructorSuper`](https://docs.rome.tools/lint/rules/noinvalidconstructorsuper/) rule that erroneously reported generic parents [#4624](https://github.com/rome/tools/issues/4624).

-  Fix [`noDuplicateCase`](https://docs.rome.tools/lint/rules/noDuplicateCase/) rule that erroneously reported as equals the strings literals `"'"` and `'"'` [#4706](https://github.com/rome/tools/issues/4706).

- Improve [`noInnerDeclarations`](https://docs.rome.tools/lint/rules/noInnerDeclarations/)

  Now, the rule doesn't report false-positives about ambient _TypeScript_ declarations.
  For example, the following code is no longer reported by the rule:

  ```ts
  declare var foo;
  ```

- Improve [`useEnumInitializers`](https://docs.rome.tools/lint/rules/useEnumInitializers/)

  The rule now reports all uninitialized members of an enum in a single diagnostic.

  Moreover, ambient enum declarations are now ignored.
  This avoids reporting ambient enum declarations in _TypeScript_ declaration files.

  ```ts
  declare enum Weather {
    Rainy,
    Sunny,
  }
  ```

- Relax [`noBannedTypes`](https://docs.rome.tools/lint/rules/nobannedtypes/) and improve documentation

  The rule no longer reports a user type that reuses a banned type name.
  The following code is now allowed:

  ```ts
  import { Number } from "a-lib";
  declare const v: Number;
  ```

  The rule now allows the use of the type `{}` to denote a non-nullable generic type:

  ```ts
  function f<T extends {}>(x: T) {
      assert(x != null);
  }
  ```

  And in a type intersection for narrowing a type to its non-nullable equivalent type:

  ```ts
  type NonNullableMyType = MyType & {};
  ```

- Improve the diagnostic and the code action of [`useDefaultParameterLast`](https://docs.rome.tools/lint/rules/usedefaultparameterlast/).

  The diagnostic now reports the last required parameter which should precede optional and default parameters.

  The code action now removes any whitespace between the parameter name and its initialization.

- Relax [noConfusingArrow](https://docs.rome.tools/lint/rules/noconfusingarrow/)

  All arrow functions that enclose its parameter with parenthesis are allowed.
  Thus, the following snippet no longer trigger the rule:

  ```js
  var x = (a) => 1 ? 2 : 3;
  ```

  The following snippet still triggers the rule:

  ```js
  var x = a => 1 ? 2 : 3;
  ```

- Relax [`useLiteralEnumMembers`](https://docs.rome.tools/lint/rules/useLiteralEnumMembers/)

  Enum members that refer to previous enum members are now allowed.
  This allows a common pattern in enum flags like in the following example:

  ```ts
  enum FileAccess {
    None = 0,
    Read = 1,
    Write = 1 << 1,
    All = Read | Write,
  }
  ```

  Arbitrary numeric constant expressions are also allowed:

  ```ts
  enum FileAccess {
    None = 0,
    Read = 2**0,
    Write = 2**1,
    All = Read | Write,
  }
  ```

- Improve [useLiteralKeys](https://docs.rome.tools/lint/rules/useLiteralKeys/).

  Now, the rule suggests simplifying computed properties to string literal properties:

  ```diff
  {
  -  ["1+1"]: 2,
  +  "1+1": 2,
  }
  ```

  It also suggests simplifying string literal properties to static properties:

  ```diff
  {
  -  "a": 0,
  +  a: 0,
  }
  ```

  These suggestions are made in object literals, classes, interfaces, and object types.

- Improve [`noNewSymbol`](https://docs.rome.tools/lint/rules/noNewSymbol/).

  The rule now handles cases where `Symbol` is namespaced with the global `globalThis` or `window`.

- The rules [`useExhaustiveDependencies`](https://docs.rome.tools/lint/rules/useexhaustivedependencies/) and [`useHookAtTopLevel`](https://docs.rome.tools/lint/rules/usehookattoplevel/) accept a different shape of options

  Old configuration

  ```json
  {
    "linter": {
       "rules": {
          "nursery": {
             "useExhaustiveDependencies": {
                "level": "error",
                "options": {
                   "hooks": [
                      ["useMyEffect", 0, 1]
                   ]
                }
             }
          }
       }
    }
  }
  ```

  New configuration

  ```json
  {
    "linter": {
       "rules": {
          "nursery": {
             "useExhaustiveDependencies": {
                "level": "error",
                "options": {
                   "hooks": [
                      {
                         "name": "useMyEffect",
                         "closureIndex": 0,
                         "dependenciesIndex": 1
                      }
                   ]
                }
             }
          }
       }
    }
  }
  ```

- [noRedundantUseStrict](https://docs.rome.tools/lint/rules/noredundantusestrict/) check only `'use strict'` directive to resolve false positive diagnostics.

  React introduced new directives, "use client" and "use server".
  The rule raises false positive errors about these directives.

- Fix false positive diagnostics ([#4483](https://github.com/rome/tools/issues/4483)) that  [`NoUnreachableSuper`](https://docs.rome.tools/lint/rules/nounreachablesuper/) caused to nested if statement.

  The rule no longer reports `This constructor calls super() in a loop`
  when using nested if statements in a constructor.

- Fix [useHookAtTopLevel](https://docs.rome.tools/lint/rules/usehookattoplevel/) 's false positive diagnostics ([#4637](https://github.com/rome/tools/issues/4637))

  The rule no longer reports false positive diagnostics when accessing properties directly from a hook and calling a hook inside function arguments.

- Fix [noUselessFragments](https://docs.rome.tools/lint/rules/nouselessfragments/)'s panics when running `rome check --apply-unsafe` ([#4637](https://github.com/rome/tools/issues/4639))

  This rule's code action emits an invalid AST, so I fixed using JsxString instead of JsStringLiteral

- Fix [noUndeclaredVariables](https://docs.rome.tools/lint/rules/noundeclaredvariables/)'s false positive diagnostics ([#4675](https://github.com/rome/tools/issues/4675))

  The semantic analyzer no longer handles `this` reference identifier.

- Fix [noUnusedVariables](https://docs.rome.tools/lint/rules/nounusedvariables/)'s false positive diagnostics ([#4688](https://github.com/rome/tools/issues/4688))

  The semantic analyzer handles ts export declaration clause correctly.

### Parser

- Add support for decorators in class method parameters, example:

    ```js
    class AppController {
       get(@Param() id) {}
       // ^^^^^^^^ new supported syntax
    }
    ```

  This syntax is only supported via configuration, because it's a non-standard
  syntax.

    ```json
    {
       "//": "rome.json file",
       "javascript": {
          "parser": {
             "unsafeParameterDecoratorsEnabled": true
          }
       }
    }
    ```
- Add support for parsing comments inside JSON files:

  ```json
  {
    "json": {
      "parser": {
        "allowComments": true
      }
    }
  }
  ```
- Add support for the new `using` syntax

  ```js
  const using = resource.lock();
  ```
