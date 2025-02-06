# Biome changelog

This project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Due to the nature of Biome as a toolchain,
it can be unclear what changes are considered major, minor, or patch.
Read our [guidelines to categorize a change](https://biomejs.dev/internals/versioning).

New entries must be placed in a section entitled `Unreleased`.
Read
our [guidelines for writing a good changelog entry](https://github.com/biomejs/biome/blob/main/CONTRIBUTING.md#changelog).

## v1.9.4 (2024-10-17)

### Analyzer

#### Bug fixes

- Implement [GraphQL suppression action](https://github.com/biomejs/biome/pull/4312). Contributed by @vohoanglong0107

- Improved the message for unused suppression comments. Contributed by @dyc3

- Fix [#4228](https://github.com/biomejs/biome/issues/4228), where the rule `a11y/noInteractiveElementToNoninteractiveRole` incorrectly reports a `role` for non-interactive elements. Contributed by @eryue0220

- `noSuspiciousSemicolonInJsx` now catches suspicious semicolons in React fragments. Contributed by @vasucp1207

- The syntax rule `noTypeOnlyImportAttributes` now ignores `.cts` files ([#4361](https://github.com/biomejs/biome/issues/4361)).

  Since TypeScript 5.3, type-only imports can be associated to an import attribute in CommonJS-enabled files.
  See the [TypeScript docs](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-3.html#stable-support-resolution-mode-in-import-types).

  The following code is no longer reported as a syntax error:

  ```cts
  import type { TypeFromRequire } from "pkg" with {
      "resolution-mode": "require"
  };
  ```

  Note that this is only allowed in files ending with the `cts` extension.

  Contributed by @Conaclos

### CLI

#### Enhancements

- The `--summary` reporter now reports parsing diagnostics too. Contributed by @ematipico

- Improved performance of GritQL queries by roughly 25-30%. Contributed by @arendjr

### Configuration

#### Bug fixes

- Fix an issue where the JSON schema marked lint rules options as mandatory. Contributed by @ematipico

### Editors

### Formatter

#### Bug fixes

- Fix [#4121](https://github.com/biomejs/biome/issues/4121). Respect line width when printing multiline strings. Contributed by @ah-yu
- Fix [#4384](https://github.com/biomejs/biome/issues/4384). Keep `@charset` dobule quote under any situation for css syntax rule. Contributed by @fireairforce

### JavaScript APIs

### Linter

#### New features

- Add [useGuardForIn](https://biomejs.dev/linter/rules/use-guard-for-in/). Contributed by @fireairforce
- Add [noDocumentCookie](https://biomejs.dev/linter/rules/no-document-cookie/). Contributed by @tunamaguro
- Add [noDocumentImportInPage](https://biomejs.dev/linter/rules/no-document-import-in-page/). Contributed by @kaioduarte
- Add [noDuplicateProperties](https://biomejs.dev/linter/rules/no-duplicate-properties/). Contributed by @togami2864
- Add [noHeadElement](https://biomejs.dev/linter/rules/no-head-element/). Contributed by @kaioduarte
- Add [noHeadImportInDocument](https://biomejs.dev/linter/rules/no-head-import-in-document/). Contributed by @kaioduarte
- Add [noImgElement](https://biomejs.dev/linter/rules/no-img-element/). Contributed by @kaioduarte
- Add [noUnknownTypeSelector](https://biomejs.dev/linter/rules/no-unknown-type-selector/). Contributed by @Kazuhiro-Mimaki
- Add [useAtIndex](https://biomejs.dev/linter/rules/use-at-index/). Contributed by @GunseiKPaseri
- Add [noUselessStringRaw](https://biomejs.dev/linter/rules/no-useless-string-raw/). Contributed by @fireairforce
- Add [nursery/useCollapsedIf](https://biomejs.dev/linter/rules/use-collapsed-if/). Contributed by @siketyan
- Add [useGoogleFontDisplay](https://biomejs.dev/linter/rules/use-google-font-display/). Contributed by @kaioduarte
- Add [useExportsLast](https://biomejs.dev/linter/rules/use-exports-last/). Contributed by @tommymorgan

#### Bug Fixes

- Biome no longer crashes when it encounters a string that contain a multibyte character ([#4181](https://github.com/biomejs/biome/issues/4181)).

  This fixes a regression introduced in Biome 1.9.3
  The regression affected the following linter rules:

  - `nursery/useSortedClasses`
  - `nursery/useTrimStartEnd`
  - `style/useTemplate`
  - `suspicious/noMisleadingCharacterClass`

  Contributed by @Conaclos

- Fix [#4190](https://github.com/biomejs/biome/issues/4190), where the rule `noMissingVarFunction` wrongly reported a variable as missing when used inside a `var()`  function that was a newline. Contributed by @ematipico

- Fix [#4041](https://github.com/biomejs/biome/issues/4041). Now the rule `useSortedClasses` won't be triggered if `className` is composed only by inlined variables. Contributed by @ematipico

- [useImportType](https://biomejs.dev/linter/rules/use-import-type/) and [useExportType](https://biomejs.dev/linter/rules/use-export-type/) now report useless inline type qualifiers ([#4178](https://github.com/biomejs/biome/issues/4178)).

  The following fix is now proposed:

  ```diff
  - import type { type A, B } from "";
  + import type { A, B } from "";

  - export type { type C, D };
  + export type { C, D };
  ```

  Contributed by @Conaclos

- [useExportType](https://biomejs.dev/linter/rules/use-export-type/) now reports ungrouped `export from`.

  The following fix is now proposed:

  ```diff
  - export { type A, type B } from "";
  + export type { A, B } from "";
  ```

  Contributed by @Conaclos

- [noVoidTypeReturn](https://biomejs.dev/linter/rules/no-void-type-return/) now accepts `void` expressions in return position ([#4173](https://github.com/biomejs/biome/issues/4173)).

  The following code is now accepted:

  ```ts
  function f(): void {
    return void 0;
  }
  ```

  Contributed by @Conaclos

- [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) now correctly handles fragments containing HTML escapes (e.g. `&nbsp;`) inside expression escapes `{ ... }` ([#4059](https://github.com/biomejs/biome/issues/4059)).

  The following code is no longer reported:

  ```jsx
  function Component() {
    return (
      <div key={index}>{line || <>&nbsp;</>}</div>
    )
  }
  ```

  Contributed by @fireairforce

- [noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters/) and [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports a parameter as unused when another parameter has a constructor type with the same parameter name ([#4227](https://github.com/biomejs/biome/issues/4227)).

  In the following code, the `name` parameter is no longer reported as unused.

  ```ts
  export class Foo {
    bar(name: string, _class: new (name: string) => any) {
      return name
    }
  }
  ```

  Contributed by @Conaclos

- [noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies/) now accepts dependency names with dots. Contributed by @Conaclos

- [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) now correctly handles renamed exports ([#4254](https://github.com/biomejs/biome/issues/4254)).

  The rule allows the filename to be named as one of the exports of the module.
  For instance, the file containing the following export can be named `Button`.

  ```js
  class Button {}
  export { Button }
  ```

  The rule now correctly handles the renaming of an export.
  For example, the file containing the following export can only be named `Button`.
  Previously the rule expected the file to be named `A`.

  ```js
  class A {}
  export { A as Button }
  ```

  Contributed by @Conaclos

- [useConsistentMemberAccessibility](https://biomejs.dev/linter/rules/use-consistent-member-accessibility/) now ignore private class members such as `#property` ([#4276](https://github.com/biomejs/biome/issues/4276)). Contributed by @Conaclos

- [noUnknownFunction](https://biomejs.dev/linter/rules/no-unknown-function/) correctly handles `calc-size` function ([#4212](https://github.com/biomejs/biome/issues/4212)).

   The following code `calc-size` is no longer reported as unknown:

   ```css
   .a { height: calc-size(0px); }
   ```

   Contributed by @fireairforce

 - [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) now allows configuring conventions for readonly index signatures.

  Contributed by @sepruko

- [noDuplicateCustomProperties](https://biomejs.dev/linter/rules/no-duplicate-custom-properties/) now correctly handles custom properties and ignores non-custom properties.
  Previously, the rule incorrectly reported duplicates for all properties, including non-custom ones. Contributed by @togami2864

### Parser

#### Bug Fixes

- The CSS parser now accepts more emoji in identifiers ([#3627](https://github.com/biomejs/biome/issues/3627#issuecomment-2392388022)).

  Browsers accept more emoji than the standard allows.
  Biome now accepts these additional emojis.

  The following code is now correctly parsed:

  ```css
  p {
    --✨-color: red;
    color: var(--✨-color);
  }
  ```

  Contributed by @Conaclos

- Add support for parsing typescript's `resolution-mode` in Import Types([#2115](https://github.com/biomejs/biome/issues/2115))

  ```ts
  export type Fs = typeof import('fs', { with: { 'resolution-mode': 'import' } });
  export type TypeFromRequire =
    import("pkg", { with: { "resolution-mode": "require" } }).TypeFromRequire;
  export type TypeFromImport =
    import("pkg", { with: { "resolution-mode": "import" } }).TypeFromImport;
  ```

  Contributed by @fireairforce

## v1.9.3 (2024-10-01)

### CLI

#### New features

- GritQL queries that match functions or methods will now match async functions or methods as well.

  If this is not what you want, you can capture the `async` keyword (or its absence) in a metavariable and assert its emptiness:

  ```grit
  $async function foo() {} where $async <: .
  ```

  Contributed by @arendjr

#### Bug fixes

- Fix [#4077](https://github.com/biomejs/biome/issues/4077): Grit queries no longer need to match the statement's trailing semicolon. Contributed by @arendjr

- Fix [#4102](https://github.com/biomejs/biome/issues/4102). Now the CLI command `lint` doesn't exit with an error code when using `--write`/`--fix`. Contributed by @ematipico

### Configuration

#### Bug fixes
- Fix [#4125](https://github.com/biomejs/biome/issues/4125), where `noLabelWithoutControl` options where incorrectly marked as mandatory. Contributed by @ematipico

### Editors

- Fix a case where CSS files weren't correctly linted using the default configuration. Contributed by @ematipico

#### Bug fixes

- Fix [#4116](https://github.com/biomejs/biome/issues/4116). Unify LSP code action kinds. Contributed by @vitallium

### Formatter

#### Bug fixes

- Fix [#3924](https://github.com/biomejs/biome/issues/3924) where GraphQL formatter panics in block comments with empty line. Contributed by @vohoanglong0107
- Fix [#3364](https://github.com/biomejs/biome/issues/3364) where the `useSelfClosingElements` rule forces the `script` tag to be self-closing. Previously, this rule applies to all elements and cannot be disabled for native HTML elements.

  Now, this rule accepts a `ignoreHtmlElements` option, which when set to `true`, ignores native HTML elements and allows them to be non-self-closing.

  Contributed by @abidjappie

- Fix a case where raw values inside `url()` functions weren't properly trimmed.
  ```diff
  .value {
  -  background: url(
  -   whitespace-around-string
  -  );
  + background: url(whitespace-around-string);
  }
  ```
  Contributed by @ematipico

- Fixed [#4076](https://github.com/biomejs/biome/issues/4076), where a media query wasn't correctly formatted:
  ```diff
  .class {
  -  @media (1024px <= width <=1280px) {
  +  @media (1024px <= width <= 1280px) {
     color: red;
     }
  }
  ```
  Contributed by @blaze-d83

### JavaScript API

#### Bug fixes

- Fix [#3881](https://github.com/biomejs/biome/issues/3881), by updating the APIs to use the latest WASM changes. Contributed by @ematipico

### Linter

#### New features

- Add [noDescendingSpecificity](https://biomejs.dev/linter/rules/no-descending-specificity/). Contributed by @tunamaguro

- Add [noNestedTernary](https://biomejs.dev/linter/rules/no-nested-ternary/). Contributed by @kaykdm

- Add [noTemplateCurlyInString](https://biomejs.dev/linter/rules/no-template-curly-in-string/). Contributed by @fireairforce

- Add [noOctalEscape](https://biomejs.dev/linter/rules/no-octal-escape/). Contributed by @fireairforce

#### Enhancements

- Add an option `reportUnnecessaryDependencies` to [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/).

  Defaults to true. When set to false, errors will be suppressed for React hooks that declare dependencies but do not use them.

  Contributed by @simon-paris

- Add an option `reportMissingDependenciesArray` to [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/). Contributed by @simon-paris

#### Bug fixes

- [noControlCharactersInRegex](https://www.biomejs.dev/linter/rules/no-control-characters-in-regex) no longer panics on regexes with incomplete escape sequences. Contributed by @Conaclos

- [noMisleadingCharacterClass](https://biomejs.dev/linter/rules/no-misleading-character-class/) no longer reports issues outside of character classes.

  The following code is no longer reported:

  ```js
  /[a-z]👍/;
  ```

  Contributed by @Conaclos

- [noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies/) no longer reports Node.js builtin modules as undeclared dependencies.

  The rule no longer reports the following code:

  ```js
  import * as fs from "fs";
  ```

  Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer panics when suggesting the renaming of a variable at the start of a file ([#4114](https://github.com/biomejs/biome/issues/4114)). Contributed by @Conaclos

- [noUselessEscapeInRegex](https://biomejs.dev/linter/rules/no-useless-escape-in-regex/) no longer panics on regexes that start with an empty character class. Contributed by @Conaclos

- [noUselessStringConcat](https://biomejs.dev/linter/rules/no-useless-string-concat/) no longer panics when it encounters malformed code. Contributed by @Conaclos

- [noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters/) no longer reports unused parameters inside an object pattern with a rest parameter.

  In the following code, the rule no longer reports `a` as unused.

  ```js
  function f({ a, ...rest }) {
    return rest;
  }
  ```

  This matches the behavior of [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/).

  Contributed by @Conaclos

- [useButtonType](https://biomejs.dev/linter/rules/use-button-type/) no longer reports dynamically created button with a valid type ([#4072](https://github.com/biomejs/biome/issues/4072)).

  The following code is no longer reported:

  ```js
  React.createElement("button", { type: "button" }, "foo")
  ```

  Contributed by @Conaclos

- [useSemanticElements](https://biomejs.dev/linter/rules/use-semantic-elements/) now ignores elements with the `img` role ([#3994](https://github.com/biomejs/biome/issues/3994)).

  [MDN recommends](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role) using `role="img"` for grouping images or creating an image from other elements.
  The following code is no longer reported:

  ```jsx
  <div role="img" aria-label="That cat is so cute">
    <p>&#x1F408; &#x1F602;</p>
  </div>
  ```

  Contributed by @Conaclos

- [useSemanticElements](https://biomejs.dev/linter/rules/use-semantic-elements/) now ignores `alert` and `alertdialog` roles ([#3858](https://github.com/biomejs/biome/issues/3858)). Contributed by @Conaclos

- [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) don't create invaild JSX code when Fragments children contains JSX Expression and in a LogicalExpression. Contributed by @fireairforce

### Parser

#### Bug fixes

- Forbid undefined as type name for typescript parser. Contributed by @fireairforce

## v1.9.2 (2024-09-19)

### CLI

#### New features

- Added support for custom GritQL definitions, including:
  - Pattern and predicate definitions: https://docs.grit.io/guides/patterns
  - Function definitions: https://docs.grit.io/language/functions#function-definitions

  Contributed by @arendjr

#### Bug fixes

- Fix [#3917](https://github.com/biomejs/biome/issues/3917), where the fixed files were incorrectly computed. Contributed by @ematipico
- Fixed an issue that caused GritQL `contains` queries to report false positives when the matched
  node appeared inside a sibling node. Contributed by @arendjr

### Editors

#### Bug fixes

- Fix [#3923](https://github.com/biomejs/biome/issues/3923). Now the `.editorconfig` is correctly parsed by the LSP, and the options are correctly applied to files when formatting is triggered.
  Plus, the Biome LSP now watches for any change to the `.editorconfig`, and updates the formatting settings.
- Reduced the number of log files generated by the LSP server. Now the maximum number of logs saved on disk is **seven**. Contributed by @ematipico
- Fix the code actions capabilities available in the LSP Biome server. Before, the LSP was using the default capabilities, which resulted in pulling code actions even when they were disabled by the editor.

  This means that the code actions are pulled by the client **only** when the editor enables `quickfix.biome`, `source.organizeImports.biome` and `source.fixAll.biome`.

  Now, if you enable `organizeImports.enabled: true` in the `biome.json`, and then you configure your editor with the following code action `source.organizeImports.biome: false`, the editor **won't** sort the imports.

  Contributed by @ematipico

### Linter

#### New features

- Add [nursery/noMissingVarFunction](https://biomejs.dev/linter/rules/no-missing-var-function). Contributed by @michellocana
- Add [nursery/useComponentExportOnlyModules](https://biomejs.dev/linter/rules/use-component-export-only-modules). Use this rule in React projects to enforce a code styling that fits React Refresh. Contributed by @GunseiKPaseri

#### Bug fixes

- [noLabelWithoutControl](https://biomejs.dev/linter/rules/no-label-without-control/) now accept JSX expression as label value ([#3875](https://github.com/biomejs/biome/issues/3875)). Contributed by @Conaclos

- [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) no longer suggests names with a disallowed case ([#3952](https://github.com/biomejs/biome/issues/3952)). Contributed by @Conaclos

- [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) now recognizes file names starting with ASCII digits as lowercase ([#3952](https://github.com/biomejs/biome/issues/3952)).

  Thus, `2024-09-17-filename`, `2024_09_17_filename` and `20240917FileName` are in `kebab-case`, `snake_case`, and `camelCase` respectively.

  Contributed by @Conaclos

- [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) now applies the configured formats to the file extensions ([#3650](https://github.com/biomejs/biome/discussions/3650)). Contributed by @Conaclos

### Parser

#### Bug fixes

- [useStrictMode](https://biomejs.dev/linter/rules/use-strict-mode/) now reports Script files with some directives, but without the `use strict` directive. Contributed by @Conaclos

- The CSS parser now accepts the characters U+FFDCF and U+FFFD in identifiers. Contributed by @Conaclos

## v1.9.1 (2024-09-15)

### CLI

#### Bug fixes

- `useEditorConfig` now loads the editorconfig when running `biome ci` [#3864](https://github.com/biomejs/biome/issues/3864). Contributed by @dyc3

### Editors

#### Bug fixes

- Revert [#3731](https://github.com/biomejs/biome/pull/3731) to fix broken quick fixes and code actions. Contributed by @nhedger

### Linter

#### New Features

- Add [nursery/noProcessEnv](https://biomejs.dev/linter/rules/no-process-env/). Contributed by @unvalley

#### Bug fixes

- [noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies/) now ignores `@/` imports and recognizes type imports from Definitely Typed and `bun` imports. Contributed by @Conaclos

## v1.9.0 (2024-09-12)

### Analyzer

- Implement the [semantic model for CSS](https://github.com/biomejs/biome/pull/3546). Contributed by @togami2864

### CLI

#### New features

- Add `--graphql-linter-enabled` option, to control whether the linter should be enabled or not for GraphQL files. Contributed by @ematipico

- New EXPERIMENTAL `search` command. The search command allows you to search a Biome project using [GritQL syntax](https://biomejs.dev/reference/gritql).

  GritQL is a powerful language that lets you do _structural_ searches on your codebase. This means that trivia such as whitespace or even the type of strings quotes used will be ignored in your search query. It also has many features for querying the structure of your code, making it much more elegant for searching code than regular expressions.

  While we believe this command may already be useful to users in some situations (especially when integrated in the IDE extensions!), we also had an ulterior motive for adding this command: We intend to utilize GritQL for our plugin efforts, and by allowing our users to try it out in a first iteration, we hope to gain insight in the type of queries you want to do, as well as the bugs we need to focus on.

  For now, the `search` command is explicitly marked as EXPERIMENTAL, since many bugs remain. Keep this in mind when you try it out, and please [let us know](https://github.com/biomejs/biome/issues) your issues!

  Note: GritQL escapes code snippets using backticks, but most shells interpret backticks as command invocations. To avoid this, it's best to put _single quotes_ around your Grit queries.

  ```shell
  biome search '`console.log($message)`' # find all `console.log` invocations
  ```

  Contributed by @arendjr and @BackupMiles

- The option `--max-diagnostics` now accept a `none` value, which lifts the limit of diagnostics shown. Contributed by @ematipico
  - Add a new reporter `--reporter=gitlab`, that emits diagnostics for using the [GitLab Code Quality report](https://docs.gitlab.com/ee/ci/testing/code_quality.html#implement-a-custom-tool).

    ```json
    [
      {
        "description": "Use === instead of ==. == is only allowed when comparing against `null`",
        "check_name": "lint/suspicious/noDoubleEquals",
        "fingerprint": "6143155163249580709",
        "severity": "critical",
        "location": {
          "path": "main.ts",
          "lines": {
            "begin": 4
          }
        }
      }
    ]
    ```

    Contributed by @NiclasvanEyk

- Add new options to the `lsp-proxy` and `start` commands:
  - `--log-path`: a directory where to store the daemon logs. The commands also accepts the environment variable `BIOME_LOG_PATH`.
  - `--log-prefix-name`: a prefix that's added to the file name of the logs. It defaults to `server.log`. The commands also accepts the environment variable `BIOME_LOG_PREFIX_NAME`.

  @Contributed by @ematipico

#### Enhancements

- When a `--reporter` is provided, and it's different from the default one, the value provided by via `--max-diagnostics` is ignored and **the limit is lifted**. Contributed by @ematipico

- `biome init` now generates a new config file with more options set.
  This change intends to improve discoverability of the options and to set the more commonly used options to their default values.
  Contributed by @Conaclos

- The `--verbose` flag now reports the list of files that were evaluated, and the list of files that were fixed.
  The **evaluated** files are the those files that can be handled by Biome, files that are ignored, don't have an extension or have an extension that Biome can't evaluate are excluded by this list.
  The **fixed** files are those files that were handled by Biome and *changed*. Files that stays the same after the process are excluded from this list.

  ```shell
   VERBOSE  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ℹ Files processed:

    - biome/biome.json
    - biome/packages/@biomejs/cli-win32-arm64/package.json
    - biome/packages/tailwindcss-config-analyzer/package.json

   VERBOSE  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    ℹ Files fixed:

    - biome/biome/packages/tailwindcss-config-analyzer/src/generate-tailwind-preset.ts
  ```

  Contributed by @ematipico

- Allow passing `nursery` to the `--only` and `--skip` filters.

  The `--only` option allows you to run a given rule or rule group.
  The `--skip` option allows you to skip the execution of a given group or a given rule.

  Previously, it was not possible to pass `nursery`.
  This restriction is now removed, as it may make sense to skip the nursery rules that a project has enabled.

  Contributed by @Conaclos

- The CLI now returns an error code when calling a command in `stdin` mode, and the contents of the files aren't fixed. For example, the following example will result in an error code of `1` because the `lint` command triggers some lint rules:

  ```shell
  echo "let x = 1" | biome lint --stdin-file-path=stdin.js
  ```

  Contributed by @ematipico

#### Bug fixes

- `biome lint --write` now takes `--only` and `--skip` into account ([#3470](https://github.com/biomejs/biome/issues/3470)). Contributed by @Conaclos

- Fix [#3368](https://github.com/biomejs/biome/issues/3368), now the reporter `github` tracks the diagnostics that belong to formatting and organize imports. Contributed by @ematipico

- Fix [#3545](https://github.com/biomejs/biome/issues/3545), display a warning, 'Avoid using unnecessary Fragment,' when a Fragment contains only one child element that is placed on a new line. Contributed by @satojin219

- Migrating from Prettier or ESLint no longer overwrite the `overrides` field from the configuration ([#3544](https://github.com/biomejs/biome/issues/3544)). Contributed by @Conaclos

- Fix JSX expressions for `noAriaHiddenOnFocusable` ([#3708](https://github.com/biomejs/biome/pull/3708)). Contributed by @anthonyshew

- Fix edge case for `<canvas>` elements that use `role="img"` ([#3728](https://github.com/biomejs/biome/pull/3728)). Contributed by @anthonyshew

- Fix [#3633](https://github.com/biomejs/biome/issues/3633), where diagnostics where incorrectly printed if the code has errors. Contributed by @ematipico

- Allow `aria-label` on heading to prevent `useHeadingContent` diagnostic ([#3767](https://github.com/biomejs/biome/pull/3767)). Contributed by @anthonyshew

- Fix edge case [#3791](https://github.com/biomejs/biome/issues/3791) for rule `noFocusedTests` being used with non-string-like expressions ([#3793](https://github.com/biomejs/biome/pull/3793)). Contributed by @h-a-n-a

- Fix optional ARIA properties for `role="separator"` in `useAriaPropsForRole` ([#3856](https://github.com/biomejs/biome/pull/3856)). Contributed by @anthonyshew

### Configuration

- Add support for loading configuration from `.editorconfig` files ([#1724](https://github.com/biomejs/biome/issues/1724)).

  Configuration supplied in `.editorconfig` will be overridden by the configuration in `biome.json`. Support is disabled by default and can be enabled by adding the following to your formatter configuration in `biome.json`:

  ```json
  {
    "formatter": {
      "useEditorconfig": true
    }
  }
  ```

  Contributed by @dyc3

- `overrides` from an extended configuration is now merged with the `overrides` of the extension.

  Given the following shared configuration `biome.shared.json`:

  ```json5
  {
    "overrides": [
      {
        "include": ["**/*.json"],
        // ...
      }
    ]
  }
  ```

  and the following configuration:

  ```json5
  {
    "extends": ["./biome.shared.json"],
    "overrides": [
      {
        "include": ["**/*.ts"],
        // ...
      }
    ]
  }
  ```

  Previously, the `overrides` from `biome.shared.json` was overwritten.
  It is now merged and results in the following configuration:

  ```json5
  {
    "extends": ["./biome.shared.json"],
    "overrides": [
      {
        "include": ["**/*.json"],
        // ...
      },
      {
        "include": ["**/*.ts"],
        // ...
      }
    ]
  }
  ```

  Contributed by @Conaclos

### Editors

- Fix [#3577](https://github.com/biomejs/biome/issues/3577), where the update of the configuration file was resulting in the creation of a new internal project. Contributed by @ematipico

- Fix [#3696](https://github.com/biomejs/biome/issues/3696), where `biome.jsonc` was incorrectly parsed with incorrect options. Contributed by @ematipico

### Formatter

- The CSS formatter is enabled by default. Which means that you don't need to opt-in anymore using the configuration file `biome.json`:

  ```diff
  {
  -  "css": {
  -    "formatter": {
  -      "enabled": true
  -    }
  -  }
  }
  ```

  Contributed by @ematipico

- Add parentheses for nullcoalescing in ternaries.

  This change aligns on [Prettier 3.3.3](https://github.com/prettier/prettier/blob/main/CHANGELOG.md#333).
  This adds clarity to operator precedence.

  ```diff
  - foo ? bar ?? foo : baz;
  + foo ? (bar ?? foo) : baz;
  ```

  Contributed by @Conaclos

- Keep the parentheses around `infer ... extends` declarations in type unions and type intersections ([#3419](https://github.com/biomejs/biome/issues/3419)). Contributed by @Conaclos

- Keep parentheses around a `yield` expression inside a type assertion.

  Previously, Biome removed parentheses around some expressions that require them inside a type assertion.
  For example, in the following code, Biome now preserves the parentheses.

  ```ts
  function* f() {
    return <T>(yield 0);
  }
  ```

  Contributed by @Conaclos

- Remove parentheses around expressions that don't need them inside a decorator.

  Biome now matches Prettier in the following cases:

  ```diff
    class {
  -   @(decorator)
  +   @decorator
      method() {}
    },
    class {
  -   @(decorator())
  +   @decorator()
      method() {}
    },
    class {
      @(decorator?.())
      method() {}
    },
  ```

  Contributed by @Conaclos

- Keep parentheses around objects preceded with a `@satisfies` comment.

  In the following example, parentheses are no longer removed.

  ```ts
  export const PROPS = /** @satisfies {Record<string, string>} */ ({
    prop: 0,
  });
  ```

  Contributed by @Conaclos

### Linter

#### Promoted rules

New rules are incubated in the nursery group.
Once stable, we promote them to a stable group.

The following CSS rules are promoted:

- [a11y/useGenericFontNames](https://biomejs.dev/linter/rules/use-generic-font-names/)
- [correctness/noInvalidDirectionInLinearGradient](https://biomejs.dev/linter/rules/no-invalid-direction-in-linear-gradient/)
- [correctness/noInvalidGridAreas](https://biomejs.dev/linter/rules/no-invalid-grid-areas/)
- [correctness/noInvalidPositionAtImportRule](https://biomejs.dev/linter/rules/no-invalid-position-at-import-rule/)
- [correctness/noUnknownFunction](https://biomejs.dev/linter/rules/no-unknown-function/)
- [correctness/noUnknownMediaFeatureName](https://biomejs.dev/linter/rules/no-unknown-media-feature-name/)
- [correctness/noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property/)
- [correctness/noUnknownUnit](https://biomejs.dev/linter/rules/no-unknown-unit/)
- [correctness/noUnmatchableAnbSelector](https://biomejs.dev/linter/rules/no-unmatchable-anb-selector/)
- [suspicious/noDuplicateAtImportRules](https://biomejs.dev/linter/rules/no-duplicate-at-import-rules/)
- [suspicious/noDuplicateFontNames](https://biomejs.dev/linter/rules/no-duplicate-font-names/)
- [suspicious/noDuplicateSelectorsKeyframeBlock](https://biomejs.dev/linter/rules/no-duplicate-selectors-keyframe-block/)
- [suspicious/noEmptyBlock](https://biomejs.dev/linter/rules/no-empty-block/)
- [suspicious/noImportantInKeyframe](https://biomejs.dev/linter/rules/no-important-in-keyframe/)
- [suspicious/noShorthandPropertyOverrides](https://biomejs.dev/linter/rules/no-shorthand-property-overrides/)

The following JavaScript rules are promoted:

- [a11y/noLabelWithoutControl](https://biomejs.dev/linter/rules/no-label-without-control/)
- [a11y/useFocusableInteractive](https://biomejs.dev/linter/rules/use-focusable-interactive/)
- [a11y/useSemanticElements](https://biomejs.dev/linter/rules/use-semantic-elements/)
- [complexity/noUselessStringConcat](https://biomejs.dev/linter/rules/no-useless-string-concat/)
- [complexity/noUselessUndefinedInitialization](https://biomejs.dev/linter/rules/no-useless-undefined-initialization/)
- [complexity/useDateNow](https://biomejs.dev/linter/rules/use-date-now/)
- [correctness/noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies/)
- [correctness/noInvalidBuiltinInstantiation](https://biomejs.dev/linter/rules/no-invalid-builtin-instantiation/)
- [correctness/noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters/)
- [correctness/useImportExtensions](https://biomejs.dev/linter/rules/use-import-extensions/)
- [performance/useTopLevelRegex](https://biomejs.dev/linter/rules/use-top-level-regex/)
- [style/noDoneCallback](https://biomejs.dev/linter/rules/no-done-callback/)
- [style/noYodaExpression](https://biomejs.dev/linter/rules/no-yoda-expression/)
- [style/useConsistentBuiltinInstantiation](https://biomejs.dev/linter/rules/use-consistent-builtin-instantiation/)
- [style/useDefaultSwitchClause](https://biomejs.dev/linter/rules/use-default-switch-clause/)
- [style/useExplicitLengthCheck](https://biomejs.dev/linter/rules/use-explicit-length-check/)
- [style/useThrowNewError](https://biomejs.dev/linter/rules/use-throw-new-error/)
- [style/useThrowOnlyError](https://biomejs.dev/linter/rules/use-throw-only-error/)
- [suspicious/noConsole](https://biomejs.dev/linter/rules/no-console/)
- [suspicious/noEvolvingTypes](https://biomejs.dev/linter/rules/no-evolving-types/)
- [suspicious/noMisplacedAssertion](https://biomejs.dev/linter/rules/no-misplaced-assertion/)
- [suspicious/noReactSpecificProps](https://biomejs.dev/linter/rules/no-react-specific-props/)
- [suspicious/useErrorMessage](https://biomejs.dev/linter/rules/use-error-message/)
- [suspicious/useNumberToFixedDigitsArgument](https://biomejs.dev/linter/rules/use-number-to-fixed-digits-argument/)

#### Deprecated rules

- `correctness/noInvalidNewBuiltin` is deprecated. Use [correctness/noInvalidBuiltinInstantiation](https://biomejs.dev/linter/rules/no-invalid-builtin-instantiation/) instead.
- `style/useSingleCaseStatement` is deprecated. Use [correctness/noSwitchDeclarations](https://biomejs.dev/linter/rules/no-switch-declarations/) instead.
- `suspicious/noConsoleLog` is deprecated. Use [suspicious/noConsole](https://biomejs.dev/linter/rules/no-console/) instead.

#### New features

- Implement [css suppression action](https://github.com/biomejs/biome/issues/3278). Contributed by @togami2864

- Add support for GraphQL linting. Contributed by @ematipico

- Add [nursery/noCommonJs](https://biomejs.dev/linter/rules/no-common-js/). Contributed by @minht11

- Add [nursery/noDuplicateCustomProperties](https://biomejs.dev/linter/rules/no-duplicate-custom-properties/). Contributed by @chansuke

- Add [nursery/noEnum](https://biomejs.dev/linter/rules/no-enum/). Contributed by @nickfla1

- Add [nursery/noDynamicNamespaceImportAccess](https://biomejs.dev/linter/no-dynamic-namespace-import-access/). Contributed by @minht11

- Add [nursery/noIrregularWhitespace](https://biomejs.dev/linter/rules/no-irregular-whitespace). Contributed by @michellocana

- Add [nursery/noRestrictedTypes](https://biomejs.dev/linter/no-restricted-types/). Contributed by @minht11

- Add [nursery/noSecrets](https://biomejs.dev/linter/rules/no-secrets/). Contributed by @SaadBazaz

- Add [nursery/noUselessEscapeInRegex](https://biomejs.dev/linter/rules/no-useless-escape-in-regex/). Contributed by @Conaclos

- Add [nursery/noValueAtRule](https://biomejs.dev/linter/rules/no-value-at-rule/). Contributed by @rishabh3112

- Add [nursery/useAriaPropsSupportedByRole](https://biomejs.dev/linter/rules/use-aria-props-supported-by-role/). Contributed by @ryo-ebata

- Add [nursery/useConsistentMemberAccessibility](https://biomejs.dev/linter/rules/use-consistent-member-accessibility/). Contributed by @seitarof

- Add [nursery/useStrictMode](https://biomejs.dev/linter/rules/use-strict-mode/). Contributed by @ematipico

- Add [nursery/useTrimStartEnd](https://biomejs.dev/linter/rules/use-trim-start-end/). Contributed by @chansuke

- Add [nursery/noIrregularWhitespace](https://biomejs.dev/linter/rules/no-irreguluar-whitespace/). Contributed by @DerTimonius

#### Enhancements

- Rename `nursery/noUnknownSelectorPseudoElement` to `nursery/noUnknownPseudoElement`. Contributed by @togami2864

- The CSS linter is now enabled by default. Which means that you don't need to opt-in anymore using the configuration file `biome.json`:

  ```diff
  {
  -  "css": {
  -    "linter": {
  -      "enabled": true
  -    }
  -  }
  }
  ```

  Contributed by @ematipico

- The JavaScript linter recognizes TypeScript 5.5 and 5.6 globals. Contributed by @Conaclos

- [noBlankTarget](https://biomejs.dev/linter/rules/no-blank-target/) now supports an array of allowed domains.

  The following configuration allows `example.com` and `example.org` as blank targets.

  ```json
  "linter": {
    "rules": {
      "a11y": {
        "noBlankTarget": {
        "level": "error",
          "options": {
             "allowDomains": ["example.com", "example.org"]
            }
          }
        }
      }
    }
  ```

  Contributed by @Jayllyz

- [noConsole](https://biomejs.dev/linter/rules/no-console/) now accepts an option that specifies some allowed calls on `console`. Contributed by @Conaclos

- Add an `ignoreNull` option for [noDoubleEquals](https://biomejs.dev/linter/rules/no-double-equals/).

  By default the rule allows loose comparisons against `null`.
  The option `ignoreNull` can be set to `false` for reporting loose comparison against `null`.

  Contributed by @peaBerberian.

- [noDuplicateObjectKeys](https://biomejs.dev/linter/rules/no-duplicate-object-keys/) now works for JSON and JSONC files. Contributed by @ematipico

- [noInvalidUseBeforeDeclaration](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration) now reports direct use of an enum member before its declaration.

  In the following code, `A` is reported as use before its declaration.

  ```ts
  enum E {
    B = A << 1,
    A = 1,
  }
  ```

  Contributed by @Conaclos

- [noNodejsModules](https://biomejs.dev/linter/rules/no-nodejs-modules/) now ignores imports of a package which has the same name as a Node.js module. Contributed by @Conaclos

- [noNodejsModules](https://biomejs.dev/linter/rules/no-nodejs-modules/) now ignores type-only imports ([#1674](https://github.com/biomejs/biome/issues/1674)).

  The rule no longer reports type-only imports such as:

  ```ts
  import type assert from "assert";
  import type * as assert2 from "assert";
  ```

  Contributed by @Conaclos

- [noRedundantUseStrict](https://biomejs.dev/linter/rules/no-redundant-use-strict/) no longer reports `"use strict"` directives when the `package.json` marks explicitly the file as a script using the field `"type": "commonjs"`. Contributed by @ematipico

- [noStaticOnlyClass](https://biomejs.dev/linter/rules/no-static-only-class/) no longer reports a class that extends another class ([#3612](https://github.com/biomejs/biome/issues/3612)). Contributed by @errmayank

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) no longer reports a direct reference to an enum member ([#2974](https://github.com/biomejs/biome/issues/2974)).

  In the following code, the `A` reference is no longer reported as an undeclared variable.

  ```ts
  enum E {
    A = 1,
    B = A << 1,
  }
  ```

  Contributed by @Conaclos

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) recognized Svelte 5 runes in Svelte components and svelte files.

  Svelte 5 introduced runes.
  The rule now recognizes Svelte 5 runes in files ending with the `.svelte`, `.svelte.js` or `.svelte.ts` extensions.

  Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) now checks TypeScript declaration files.

  This allows to report a type that is unused because it isn't exported.
  Global declarations files (declarations files without exports and imports) are still ignored.

  Contributed by @Conaclos

- [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) now supports [unicase](https://en.wikipedia.org/wiki/Unicase) letters.

  [unicase](https://en.wikipedia.org/wiki/Unicase) letters have a single case: they are neither uppercase nor lowercase.
  Biome now accepts filenames in unicase.
  For example, the filename `안녕하세요` is now accepted.

  We still reject a name that mixes unicase characters with lowercase or uppercase characters.
  For example, the filename `A안녕하세요` is rejected.

  This change also fixes [#3353](https://github.com/biomejs/biome/issues/3353).
  Filenames consisting only of numbers are now accepted.

  Contributed by @Conaclos

- [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) now supports Next.js/Nuxt/Astro dynamic routes ([#3465](https://github.com/biomejs/biome/issues/3465)).

  [Next.js](https://nextjs.org/docs/pages/building-your-application/routing/dynamic-routes#catch-all-segments), [SolidStart](https://docs.solidjs.com/solid-start/building-your-application/routing#renaming-index), [Nuxt](https://nuxt.com/docs/guide/directory-structure/server#catch-all-route), and [Astro](https://docs.astro.build/en/guides/routing/#rest-parameters) support dynamic routes such as `[...slug].js` and `[[...slug]].js`.

  Biome now recognizes this syntax. `slug` must contain only alphanumeric characters.

  Contributed by @Conaclos

- [useExportType](https://biomejs.dev/linter/rules/use-export-type/) no longer reports empty `export` ([#3535](https://github.com/biomejs/biome/issues/3535)).

  An empty `export {}` allows you to force TypeScript to consider a file with no imports and exports as an EcmaScript module.
  While `export type {}` is valid, it is more common to use `export {}`.
  Users may find it confusing that the linter asks them to convert it to `export type {}`.
  Also, a bundler should be able to remove `export {}` as well as `export type {}`.
  So it is not so useful to report `export {}`.

  Contributed by @Conaclos

#### Bug fixes

- [noControlCharactersInRegex](https://www.biomejs.dev/linter/rules/no-control-characters-in-regex) now corretcly handle `\u` escapes in unicode-aware regexes.

  Previously, the rule didn't consider regex with the `v` flags as unicode-aware regexes.
  Moreover, `\uhhhh` was not handled in unicode-aware regexes.

  Contributed by @Conaclos

- [noControlCharactersInRegex](https://www.biomejs.dev/linter/rules/no-control-characters-in-regex) now reports control characters and escape sequence of control characters in string regexes. Contributed by @Conaclos

- `noExcessiveNestedTestSuites`: fix an edge case where the rule would alert on heavily nested zod schemas. Contributed by @dyc3

- `noExtraNonNullAssertion` no longer reports a single non-null assertion enclosed in parentheses ([#3352](https://github.com/biomejs/biome/issues/3352)). Contributed by @Conaclos

- [noMultipleSpacesInRegularExpressionLiterals](https://biomejs.dev/linter/rules/no-multiple-spaces-in-regular-expression-literals/) now correctly provides a code fix when Unicode characters are used. Contributed by @Conaclos

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/) no longer report redeclartions for lexically scoped function declarations [#3664](https://github.com/biomejs/biome/issues/3664).

  In JavaScript strict mode, function declarations are lexically scoped:
  they cannot be accessed outside the block where they are declared.

  In non-strict mode, function declarations are hoisted to the top of the enclosing function or global scope.

  Previously Biome always hoisted function declarations.
  It now takes into account whether the code is in strict or non strict mode.

  Contributed by @Conaclos

- [noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies/) now ignores self package imports.

  Given teh following `package.json`:

  ```json
  {
    "name": "my-package",
    "main": "index.js"
  }
  ```

  The following import is no longer reported by the rule:

  ```js
  import * as mod from "my-package";
  ```

  Contributed by @Conaclos

- Fix [[#3149](https://github.com/biomejs/biome/issues/3149)] crashes that occurred when applying the `noUselessFragments` unsafe fixes in certain scenarios. Contributed by @unvalley

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/) no longer reports a variable named as the function expression where it is declared. Contributed by @Conaclos

- `useAdjacentOverloadSignatures` no longer reports a `#private` class member and a public class member that share the same name ([#3309](https://github.com/biomejs/biome/issues/3309)).

  The following code is no longer reported:

  ```js
  class C {
    #f() {}
    g() {}
    f() {}
  }
  ```

  Contributed by @Conaclos

- [useAltText](https://www.biomejs.dev/linter/rules/use-alt-text) n olonger requests alt text for elements hidden from assistive technologies ([#3316](https://github.com/biomejs/biome/issues/3316)). Contributed by @robintown

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) now accepts applying custom convention on abstract classes. Contributed by @Conaclos

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) no longer suggests an empty fix when a name doesn't match strict Pascal case ([#3561](https://github.com/biomejs/biome/issues/3561)).

  Previously the following code led `useNamingConvention` to suggest an empty fix.
  The rule no longer provides a fix for this case.

  ```ts
  type AAb = any
  ```

  Contributed by @Conaclos

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) no longer provides fixes for global TypeScript declaration files.

  Global TypeScript declaration files have no epxorts and no imports.
  All the declared types are available in all files of the project.
  Thus, it is not safe to propose renaming only in the declaration file.

  Contributed by @Conaclos

- [useSortedClasses](https://biomejs.dev/linter/rules/use-sorted-classes/) lint error with Template literals ([#3394](https://github.com/biomejs/biome/issues/3394)). Contributed by @hangaoke1

- [useValidAriaValues](https://biomejs.dev/linter/rules/use-valid-aria-values/) now correctly check property types ([3748](https://github.com/biomejs/biome/issues/3748)).

  Properties that expect a string now accept arbitrary text.
  An identifiers can now be made up of any characters except ASCII whitespace.
  An identifier list can now be separated by any ASCII whitespace.

  Contributed by @Conaclos

### Parser

#### Enhancements

- The JSON parser now allows comments in `turbo.json` and `jest.config.json`. Contributed by @Netail and @Conaclos

- The JSON parser now allows comments in files with the `.json` extension under the `.vscode` and `.zed` directories.

  Biome recognizes are well known JSON files that allows comments and/or trailing commas.
  Previously, Biome did not recognize JSON files under the `.vscode` and the `.zed` directories as JSON files that allow comments.
  You had to configure Biome to recognize them:

  ```json
  {
    "overrides": [
      {
        "include": ["**/.vscode/*.json", "**/.zed/*.json"],
        "json": { "parser": { "allowComments": true } }
      }
    ]
  }
  ```

  This override is no longer needed!
  Note that JSON files under the `.vscode` and the `.zed` directories don't accept trailing commas.

  Contributed by @Conaclos

#### Bug fixes

- The CSS parser now accepts emoji in identifiers ([3627](https://github.com/biomejs/biome/issues/3627)).

  The following code is now correctly parsed:

  ```css
  p {
    --🥔-color: red;
    color: var(--🥔-color);
  }
  ```

  Contributed by @Conaclos

- Fix [#3287](https://github.com/biomejs/biome/issues/3287) nested selectors with pseudo-classes. Contributed by @denbezrukov

- Fix [#3349](https://github.com/biomejs/biome/issues/3349) allow CSS multiple ampersand support. Contributed by @denbezrukov

  ```css
  .class {
    && {
      color: red;
    }
  }
  ```

- Fix [#3410](https://github.com/biomejs/biome/issues/3410) by correctly parsing break statements containing keywords.
  ```js
  out: while (true) {
    break out;
  }
  ```
  Contributed by @ah-yu

- Fix [#3464](https://github.com/biomejs/biome/issues/3464) by enabling JSX in `.vue` files that use the `lang='jsx'` or `lang='tsx'` attribute. Contributed by @ematipico


## v1.8.3 (2024-06-27)

### CLI

#### Bug fixes

- Fix [#3104](https://github.com/biomejs/biome/issues/3104) by suppressing node warnings when using `biome migrate`. Contributed by @SuperchupuDev

- Force colors to be off when using the GitHub reporter to properly create annotations in GitHub actions ([#3148](https://github.com/biomejs/biome/issues/3148)). Contributed by @Sec-ant

### Parser

#### Bug fixes

- Implement [CSS unicode range](https://github.com/biomejs/biome/pull/3251). Contributed by @denbezrukov

### Formatter

#### Bug fixes

- Fix [#3184](https://github.com/biomejs/biome/issues/3184) CSS formatter converts custom identifiers to lowercase. Contributed by @denbezrukov
- Fix [#3256](https://github.com/biomejs/biome/issues/3256) constant crashes when editing css files #3256. Contributed by @denbezrukov

### Linter

#### New features

- Add `nursery/useDeprecatedReason` rule. Contributed by @vohoanglong0107.
- Add [nursery/noExportedImports](https://biomejs.dev/linter/rules/no-exported-imports/). Contributed by @Conaclos

#### Enhancements

- Implement [suggestedExtensions option](https://github.com/biomejs/biome/pull/3274) for `useImportExtensions` rule. Contributed by @drdaemos

#### Bug fixes

- `useConsistentArrayType` and `useShorthandArrayType` now ignore `Array` in the `extends` and `implements` clauses. Fix [#3247](https://github.com/biomejs/biome/issues/3247). Contributed by @Conaclos
- Fixes [#3066](https://github.com/biomejs/biome/issues/3066) by taking into account the dependencies declared in the `package.json`. Contributed by @ematipico
- The code action of the `useArrowFunction` rule now preserves a trailing comma when there is only a single type parameter in the arrow function and JSX is enabled. Fixes [#3292](https://github.com/biomejs/biome/issues/3292). Contributed by @Sec-ant

#### Enhancements
- Enhance tailwind sorting lint rule [#1274](https://github.com/biomejs/biome/issues/1274) with variant support.

  Every preconfigured variant is assigned a `weight` that concurs on establishing the output sorting order.
  Since nesting variants on the same utility class is possible, the resulting `weight` is the Bitwise XOR of all the variants weight for that class.
  Dynamic variants (e.g. `has-[.custom-class]`, `group-[:checked]`) are also supported and they take the `weight` of their base variant name the custom value attached (e.g. `has-[.custom-class]` takes `has` weight).
  Arbitrary variants (e.g. `[&nth-child(2)]`) don't have a weight assigned and they are placed after every known variant.
  Classes with the same amount of arbitrary variants follow lexicographical order. The class that has the highest number of nested arbitrary variants is placed last.
  Screen variants (e.g. `sm:`, `max-md:`, `min-lg:`) are not supported yet.

  Contributed by @lutaok

## v1.8.2 (2024-06-20)

### CLI

#### Bug fixes

- Fix [#3201](https://github.com/biomejs/biome/issues/3201) by correctly injecting the source code of the file when printing the diagnostics. Contributed by @ematipico
- Fix [#3179](https://github.com/biomejs/biome/issues/3179) where comma separators are not correctly removed after running `biome migrate` and thus choke the parser. Contributed by @Sec-ant
- Fix [#3232](https://github.com/biomejs/biome/issues/3232) by correctly using the colors set by the user. Contributed by @ematipico

#### Enhancement

- Reword the reporter message `No fixes needed` to `No fixes applied`.

  The former message is misleading when there're still errors or warnings in the files that should be taken care of manually. For example:

  ```block
  Checked 2 files in <TIME>. No fixes needed.
  Found 2 errors.
  ```

  The new message suits better in these cases.

  Contributed by @Sec-ant

### Configuration

#### Bug fixes

- Don't conceal previous overrides ([#3176](https://github.com/biomejs/biome/issues/3176)).

  Previously, each override inherited the unset configuration of the base configuration.
  This means that setting a configuration in an override can be concealed by a subsequent override that inherits of the value from the base configuration.

  For example, in the next example, `noDebugger` was disabled for the `index.js` file.

  ```json
  {
    "linter": {
      "rules": {
        "suspicious": { "noDebugger": "off" }
      }
    },
    "overrides": [
      {
        "include": ["index.js"],
        "linter": {
          "rules": {
            "suspicious": { "noDebugger": "warn" }
          }
        }
      }, {
        "include": ["index.js"],
        "linter": {
          "rules": {
            "suspicious": { "noDoubleEquals": "off" }
          }
        }
      }
    ]
  }
  ```

  The rule is now correctly enabled for the `index.js` file.

  Contributed by @Conaclos

### Formatter

#### Bug fixes

- Fix [#3103](https://github.com/biomejs/biome/issues/3103) by correctly resolving CSS formatter options. Contributed by @ah-yu
- Fix [#3192](https://github.com/biomejs/biome/issues/3192) don't add an extra whitespace within :has. Contributed by @denbezrukov

### JavaScript APIs

#### Bug fixes

- Fix a regression introduced by the release of `v1.8.0`

### Linter

#### New features

- Add [nursery/noSubstr](https://biomejs.dev/linter/rules/no-substr/). Contributed by @chansuke

- Add [nursery/useConsistentCurlyBraces](https://biomejs.dev/linter/rules/use-consistent-curly-braces/). Contributed by @dyc3

- Add [nursery/useValidAutocomplete](https://biomejs.dev/linter/rules/use-valid-autocomplete/). Contributed by @unvalley

#### Enhancements

- Add a code action for [noUselessCatch](https://biomejs.dev/linter/rules/no-useless-catch/). Contributed by @chansuke

#### Bug fixes

- Add [nursery/noShorthandPropertyOverrides](https://biomejs.dev/linter/rules/no-shorthand-property-overrides). [#2958](https://github.com/biomejs/biome/issues/2958) Contributed by @neokidev
- Fix [[#3084](https://github.com/biomejs/biome/issues/3084)] false positive by correctly recognize parenthesized return statement. Contributed by @unvalley
- [useImportExtensions](https://biomejs.dev/linter/rules/use-import-extensions/) now suggests a correct fix for `import '.'` and `import './.'`. Contributed by @minht11
- Fix [useDateNow](https://biomejs.dev/linter/rules/use-date-now/) false positive when new Date object has arguments `new Date(0).getTime()`. Contributed by @minht11.
- The [`noUnmatchableAnbSelector`](https://biomejs.dev/linter/rules/no-unmatchable-anb-selector/) rule is now able to catch unmatchable `an+b` selectors like `0n+0` or `-0n+0`. Contributed by @Sec-ant.
- The [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) rule now recognizes properties named as hooks like `foo.useFoo()`. Contributed by @ksnyder9801
- Fix [#3092](https://github.com/biomejs/biome/issues/3092), prevent warning for `Custom properties (--*)`. Contributed by @chansuke
- Fix a false positive in the [`useLiteralKeys`](https://biomejs.dev/linter/rules/use-literal-keys/) rule. ([#3160](https://github.com/biomejs/biome/issues/3160))

  This rule now ignores the following kind of computed member name:

  ```js
  const a = {
    [`line1
    line2`]: true,
  };
  ```

  Contributed by @Sec-ant

- The [noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property/) rule now ignores the `composes` property often used in css modules. [#3000](https://github.com/biomejs/biome/issues/3000) Contributed by @chansuke

- Fix false positives of the [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) rule.

  The component itself is considered stable when it is used recursively inside a hook closure defined inside of it:

  ```jsx
  import { useMemo } from "react";

  function MyRecursiveComponent() {
    // MyRecursiveComponent is stable, we don't need to add it to the dependencies list.
    const children = useMemo(() => <MyRecursiveComponent />, []);
    return <div>{children}</div>;
  }
  ```

  Also, `export default function` and `export default class` are considered stable now because they can only appear at the top level of a module.

  Contributed by @Sec-ant

- Fix missing `withDefaults` macro in vue files for globals variables. Contributed by @Shyam-Chen

### Parser

#### Bug fixes

- Fix CSS modules settings mapping. Contributed by @denbezrukov

## v1.8.1 (2024-06-10)

### CLI

#### Bug fixes

- Fix [#3069](https://github.com/biomejs/biome/issues/3069), prevent overwriting paths when using `--staged` or `--changed` options. Contributed by @unvalley
- Fix a case where the file link inside a diagnostic wasn't correctly displayed inside a terminal run by VSCode. Contributed by @uncenter

### Configuration

#### Bug fixes

- Fix [#3067](https://github.com/biomejs/biome/issues/3067), by assigning the correct default value to `indentWidth`. Contributed by @ematipico

### Formatter

#### Bug fixes
- Fix the bug where whitespace after the & character in CSS nesting was incorrectly trimmed, ensuring proper targeting of child classes [#3061](https://github.com/biomejs/biome/issues/3061). Contributed by @denbezrukov
- Fix [#3068](https://github.com/biomejs/biome/issues/3068) where the CSS formatter was inadvertently converting variable declarations and function calls to lowercase. Contributed by @denbezrukov
- Fix the formatting of CSS grid layout properties. Contributed by @denbezrukov

### Linter

#### New features

- Add [noUnknownPseudoClass](https://biomejs.dev/linter/rules/no-unknown-pseudo-class/). Contributed by  @tunamaguro

#### Bug fixes

- The `noEmptyBlock` css lint rule now treats empty blocks containing comments as valid ones. Contributed by @Sec-ant

- [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) no longer reports quoted member names ([#3085](https://github.com/biomejs/biome/issues/3085)).

  Previously [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) reported quoted member names that can be unquoted.
  For example, the rule suggested the following fix:

  ```diff
  - const x = { "prop": 0 };
  + const x = { prop: 0 };
  ```

  This conflicted with the option [quoteProperties](https://biomejs.dev/reference/configuration/#javascriptformatterquoteproperties) of our formatter.

  The rule now ignores quoted member names.

  Contributed by @Conaclos

- [noEmptyInterface](https://biomejs.dev/linter/rules/no-empty-interface/) now ignores empty interfaces in ambient modules ([#3110](https://github.com/biomejs/biome/issues/3110)). Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) and [noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters/) no longer report the parameters of a constructor type ([#3135](https://github.com/biomejs/biome/issues/3135)).

  Previously, `arg` was reported as unused in a constructor type like:

  ```ts
  export type Classlike = new (arg: unknown) => string;
  ```

  Contributed by @Conaclos

- [noStringCaseMismatch](https://biomejs.dev/linter/rules/no-string-case-mismatch/) now ignores escape sequences ([#3134](https://github.com/biomejs/biome/issues/3134)).

  The following code is no longer reported by the rule:

  ```js
  s.toUpperCase() === "\u001b";
  ```

  Contributed by @Conaclos

### Parser

#### Bug fixes

- Implemented CSS Unknown At-Rule parsing, allowing the parser to gracefully handle unsupported or unrecognized CSS at-rules. Contributed by @denbezrukov
- Fix [#3055](https://github.com/biomejs/biome/issues/3055) CSS: Layout using named grid lines is now correctly parsed. Contributed by @denbezrukov
- Fix [#3091](https://github.com/biomejs/biome/issues/3091). Allows the parser to handle nested style rules and at-rules properly, enhancing the parser's compatibility with the CSS Nesting Module. Contributed by @denbezrukov

## 1.8.0 (2024-06-04)

### Analyzer

#### New features

- Allow suppression comments to suppress individual instances of rules. This is
  used for the lint rule `useExhaustiveDependencies`, which is now able to
  suppress specific dependencies. Fixes #2509. Contributed by @arendjr

#### Enhancements

- Assume `Astro` object is always a global when processing `.astro` files. Contributed by @minht11
- Assume Vue compiler macros are globals when processing `.vue` files. ([#2771](https://github.com/biomejs/biome/pull/2771)) Contributed by @dyc3

### CLI

#### New features

- New `clean` command. Use this new command to clean after the `biome-logs` directory, and remove all the log files.

  ```shell
  biome clean
  ```

- Add two new options `--only` and `--skip` to the command `biome lint` ([#58](https://github.com/biomejs/biome/issues/58)).

  The `--only` option allows you to run a given rule or rule group,
  For example, the following command runs only the `style/useNamingConvention` and `style/noInferrableTypes` rules.
  If the rule is disabled in the configuration, then its severity level is set to `error` for a recommended rule or `warn` otherwise.

  ```shell
  biome lint --only=style/useNamingConvention --only=style/noInferrableTypes
  ```

  Passing a group does not change the severity level of the rules in the group.
  All the disabled rules in the group will remain disabled.
  To ensure that the group is run, the `recommended` field of the group is enabled.
  The `nursery` group cannot be passed, as no rules are enabled by default in the nursery group.

  The `--skip` option allows you to skip the execution of a given group or a given rule.
  For example, the following command skips the `style` group and the `suspicious/noExplicitAny` rule.

  ```shell
  biome lint --skip=style --skip=suspicious/noExplicitAny
  ```

  You can also use `--only` and `--skip` together. `--skip` overrides `--only`.
  The following command executes only the rules from the `style` group, but the `style/useNamingConvention` rule.

  ```shell
  biome lint --only=style --skip=style/useNamingConvention
  ```

  These options are compatible with other options such as `--write` (previously `--apply`), and `--reporter`.

  Contributed by @Conaclos

- Add new command `biome clean`. Use this command to purge all the logs emitted by the Biome daemon. This command is really useful, because the Biome daemon tends
  log many files and contents during its lifecycle. This means that if your editor is open for hours (or even days), the `biome-logs` folder could become quite heavy. Contributed by @ematipico

- Add support for formatting and linting CSS files from the CLI. These operations are **opt-in** for the time being.

  If you don't have a configuration file, you can enable these features with `--css-formatter-enabled` and `--css-linter-enabled`:

  ```shell
  biome check --css-formatter-enabled=true --css-linter-enabled=true ./
  ```
  Contributed by @ematipico

- Add new CLI options to control the CSS formatting. Check the [CLI reference page](https://biomejs.dev/reference/cli/) for more details. Contributed by @ematipico

- Add new options `--write`, `--fix` (alias of `--write`) and `--unsafe` to the command `biome lint` and `biome check`.
  Add a new option `--fix` (alias of `--write`) to the command `biome format` and `biome migrate`.

  ```shell
  biome <lint|check> --<write|fix> [--unsafe]
  biome format --<write|fix>
  biome migrate --<write|fix>
  ```

  The `biome <lint|check> --<write|fix>` has the same behavior as `biome <lint|check> --apply`.
  The `biome <lint|check> --<write|fix> --unsafe` has the same behavior as `biome <lint|check> --apply-unsafe`.
  The `biome format --fix` has the same behavior as `biome format --write`.
  The `biome migrate --fix` has the same behavior as `biome migrate --write`.

  This change allows these commands to write modifications in the same options.
  With this change, the `--apply` and `--apply-unsafe` options are deprecated.

  Contributed by @unvalley

#### Enhancements

- Biome now executes commands (lint, format, check and ci) on the working directory by default. [#2266](https://github.com/biomejs/biome/issues/2266) Contributed by @unvalley

  ```diff
  - biome check .
  + biome check    # You can run the command without the path
  ```

- `biome migrate eslint` now tries to convert ESLint ignore patterns into Biome ignore patterns.

  ESLint uses [gitignore patterns](https://git-scm.com/docs/gitignore#_pattern_format).
  Biome now tries to convert these patterns into Biome ignore patterns.

  For example, the gitignore pattern `/src` is a relative path to the file in which it appears.
  Biome now recognizes this and translates this pattern to `./src`.

  Contributed by @Conaclos

- `biome migrate eslint` now supports the `eslintIgnore` field in `package.json`.

  ESLint allows the use of `package.json` as an ESLint configuration file.
  ESLint supports two fields: `eslintConfig` and `eslintIgnore`.
  Biome only supported the former. It now supports both.

  Contributed by @Conaclos

- `biome migrate eslint` now propagates NodeJS errors to the user.

  This will help users to identify why Biome is unable to load some ESLint configurations.

  Contributed by @Conaclos

- Add a new `--reporter` called `summary`. This reporter will print diagnostics in a different way, based on the tools (formatter, linter, etc.) that are executed.
  Import sorting and formatter shows the name of the files that require formatting. Instead, the linter will group the number of rules triggered and the number of errors/warnings:

  ```
  Formatter ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  The following files needs to be formatted:
  main.ts
  index.ts

  Organize Imports ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  The following files needs to have their imports sorted:
  main.ts
  index.ts

  Analyzer ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Some analyzer rules were triggered

  Rule Name                                               Diagnostics
  lint/suspicious/noImplicitAnyLet                        12 (12 error(s), 0 warning(s), 0 info(s))
  lint/suspicious/noDoubleEquals                          8 (8 error(s), 0 warning(s), 0 info(s))
  lint/suspicious/noRedeclare                             12 (12 error(s), 0 warning(s), 0 info(s))
  lint/suspicious/noDebugger                              20 (20 error(s), 0 warning(s), 0 info(s))
  ```
  Contributed by @ematipico

- `biome ci` now enforces printing the output using colours. If you were previously using `--colors=force`, you can remove it because it's automatically set. Contributed by @ematipico
- Add a new `--reporter` called `github`. This reporter will print diagnostics using [GitHub workflow commands](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#about-workflow-commands):

  ```
  ::error title=lint/suspicious/noDoubleEquals,file=main.ts,line=4,endLine=4,col=3,endColumn=5::Use === instead of ==
  ::error title=lint/suspicious/noDebugger,file=main.ts,line=6,endLine=6,col=1,endColumn=9::This is an unexpected use of the debugger statement.
  ::error title=lint/nursery/noEvolvingAny,file=main.ts,line=8,endLine=8,col=5,endColumn=6::This variable's type is not allowed to evolve implicitly, leading to potential any types.
  ```
  Contributed by @ematipico
- Add a new `--reporter` called `junit`. This reporter will print diagnostics using [GitHub workflow commands](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#about-workflow-commands):

  ```xml
  <?xml version="1.0" encoding="UTF-8"?>
  <testsuites name="Biome" tests="16" failures="16" errors="20" time="<TIME>">
    <testsuite name="main.ts" tests="1" disabled="0" errors="0" failures="1" package="org.biome">
        <testcase name="org.biome.lint.suspicious.noDoubleEquals" line="4" column="3">
            <failure message="Use === instead of ==. == is only allowed when comparing against `null`">line 3, col 2, Use === instead of ==. == is only allowed when comparing against `null`</failure>
        </testcase>
    </testsuite>
    <testsuite name="main.ts" tests="1" disabled="0" errors="0" failures="1" package="org.biome">
        <testcase name="org.biome.lint.suspicious.noDebugger" line="6" column="1">
            <failure message="This is an unexpected use of the debugger statement.">line 5, col 0, This is an unexpected use of the debugger statement.</failure>
        </testcase>
    </testsuite>
    <testsuite name="main.ts" tests="1" disabled="0" errors="0" failures="1" package="org.biome">
        <testcase name="org.biome.lint.nursery.noEvolvingAny" line="8" column="5">
            <failure message="This variable&apos;s type is not allowed to evolve implicitly, leading to potential any types.">line 7, col 4, This variable&apos;s type is not allowed to evolve implicitly, leading to potential any types.</failure>
        </testcase>
    </testsuite>
  </testsuites>
  ```
  Contributed by @ematipico

#### Bug fixes

- Fix  [#3024](https://github.com/biomejs/biome/issues/3024), where running `biome init` would create `biome.json` even if `biome.jsonc` already exists.  Contributed by @minht11

### Configuration

#### New features

- Add an rule option `fix` to override the code fix kind of a rule ([#2882](https://github.com/biomejs/biome/issues/2882)).

  A rule can provide a safe or an **unsafe** code **action**.
  You can now tune the kind of code actions thanks to the `fix` option.
  This rule option takes a value among:

  - `none`: the rule no longer emits code actions.
  - `safe`: the rule emits safe code action.
  - `unsafe`: the rule emits unsafe code action.

  The following configuration disables the code actions of `noUnusedVariables`, makes the emitted code actions of `style/useConst` and `style/useTemplate` unsafe and safe respectively.

  ```json
  {
    "linter": {
      "rules": {
        "correctness": {
          "noUnusedVariables": {
            "level": "error",
            "fix": "none"
          },
          "style": {
            "useConst": {
              "level": "warn",
              "fix": "unsafe"
            },
            "useTemplate": {
              "level": "warn",
              "fix": "safe"
            }
          }
        }
      }
    }
  }
  ```

  Contributed by @Conaclos

- Add option `javascript.linter.enabled` to control the linter for JavaScript (and its super languages) files. Contributed by @ematipico
- Add option `json.linter.enabled` to control the linter for JSON (and its super languages) files. Contributed by @ematipico
- Add option `css.linter.enabled` to control the linter for CSS (and its super languages) files. Contributed by @ematipico
- Add option `css.formatter`, to control the formatter options for CSS (and its super languages) files. Contributed by @ematipico
- You can now change the severity of lint rules down to `"info"`. The `"info"` severity doesn't emit error codes, and it isn't affected by other options like `--error-on-warnings`:

  ```json
  {
    "linter": {
      "rules": {
        "suspicious": {
          "noDebugger": "info"
        }
      }
    }
  }
  ```
  Contributed by @ematipico

#### Enhancements

- The `javascript.formatter.trailingComma` option is deprecated and renamed to `javascript.formatter.trailingCommas`. The corresponding CLI option `--trailing-comma` is also deprecated and renamed to `--trailing-commas`. Details can be checked in [#2492](https://github.com/biomejs/biome/pull/2492). Contributed by @Sec-ant

#### Bug fixes

- Fix a bug where if the formatter was disabled at the language level, it could be erroneously enabled by an
  override that did not specify the formatter section [#2924](https://github.com/biomejs/biome/issues/2924). Contributed by @dyc3
- Fix [#2990](https://github.com/biomejs/biome/issues/2990), now Biome doesn't add a trailing comma when formatting `biome.json`. Contributed by @dyc3

### Editors

#### New features

- Add support for LSP Workspaces

#### Enhancements

- The LSP doesn't crash anymore when the configuration file contains errors. If the configuration contains errors, Biome now shows a pop-up to the user, and it will only parse files using the default configuration.
  Formatting and linting is disabled until the configuration file is fixed. Contributed by @ematipico

#### Bug fixes

- Fixes [#2781](https://github.com/biomejs/biome/issues/2781), by correctly computing the configuration to apply to a specific file. Contributed by @ematipico

### Formatter

#### Bug fixes

- Fix [#2470](https://github.com/biomejs/biome/issues/2470) by avoid introducing linebreaks in single line string interpolations. Contributed by @ah-yu
- Resolve deadlocks by narrowing the scope of locks. Contributed by @mechairoi
- Fix [#2782](https://github.com/biomejs/biome/issues/2782) by computing the enabled rules by taking the override settings into consideration. Contributed by @ematipico
- Fix [https://github.com/biomejs/biome/issues/2877] by correctly handling line terminators in JSX string. Contributed by @ah-yu

### Linter

#### Promoted rules

New rules are incubated in the nursery group. Once stable, we promote them to a stable group. The following rules are promoted:

- [useImportRestrictions](https://biomejs.dev/linter/rules/use-import-restrictions/)
- [noNodejsModules](https://biomejs.dev/linter/rules/no-nodejs-modules/)
- [useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/)
- [noConstantMathMinMaxClamp](https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp/)
- [noFlatMapIdentity](https://biomejs.dev/linter/rules/no-flat-map-identity/)

#### New features

- Add [nursery/useDateNow](https://biomejs.dev/linter/rules/use-date-now/). Contributed by @minht11
- Add [nursery/useErrorMessage](https://biomejs.dev/linter/rules/use-error-message/). Contributed by @minht11
- Add [nursery/useThrowOnlyError](https://biomejs.dev/linter/rules/use-throw-only-error/). Contributed by @minht11
- Add [nursery/useImportExtensions](https://biomejs.dev/linter/rules/use-import-extensions/). Contributed by @minht11

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) now supports an option to enforce custom conventions ([#1900](https://github.com/biomejs/biome/issues/1900)).

  For example, you can enforce the use of a prefix for private class members:

  ```json
  {
  	"linter": {
  		"rules": {
  			"style": {
  				"useNamingConvention": {
  					"level": "error",
  					"options": {
  						"conventions": [
  							{
  								"selector": {
  									"kind": "classMember",
  									"modifiers": ["private"]
  								},
  								"match": "_(.*)",
                  "formats": ["camelCase"]
  							}
  						]
  					}
  				}
  			}
  		}
  	}
  }
  ```

  Please, find more details in the [rule documentation](https://biomejs.dev/linter/rules/use-naming-convention/#options).

  Contributed by @Conaclos

- Add [nursery/useNumberToFixedDigitsArgument](https://biomejs.dev/linter/rules/use-number-to-fixed-digits-argument/).
  Contributed by @minht11

- Add [nursery/useThrowNewError](https://biomejs.dev/linter/rules/use-throw-new-error/).
  Contributed by @minht11
- Add [nursery/useTopLevelRegex](https://biomejs.dev/linter/rules/use-top-level-regex), which enforces defining regular expressions at the top level of a module. [#2148](https://github.com/biomejs/biome/issues/2148) Contributed by @dyc3.
- Add [nursery/noCssEmptyBlock](https://biomejs.dev/linter/rules/no-css-empty-block). [#2513](https://github.com/biomejs/biome/pull/2513) Contributed by @togami2864
- Add [nursery/noDuplicateAtImportRules](https://biomejs.dev/linter/rules/no-duplicate-at-import-rules). [#2658](https://github.com/biomejs/biome/pull/2658) Contributed by @DerTimonius
- Add [nursery/noDuplicateFontNames](https://biomejs.dev/linter/rules/no-duplicate-font-names). [#2308](https://github.com/biomejs/biome/pull/2308) Contributed by @togami2864
- Add [nursery/noDuplicateSelectorsKeyframeBlock](https://biomejs.dev/linter/rules/no-duplicate-selectors-keyframe-block). [#2534](https://github.com/biomejs/biome/pull/2534) Contributed by @isnakode
- Add [nursery/noImportantInKeyframe](https://biomejs.dev/linter/rules/no-important-in-keyframe). [#2542](https://github.com/biomejs/biome/pull/2542) Contributed by @isnakode
- Add [nursery/noInvalidPositionAtImportRule](https://biomejs.dev/linter/rules/no-invalid-position-at-import-rule). [#2717](https://github.com/biomejs/biome/issues/2717) Contributed by @t-shiratori
- Add [nursery/noUnknownFunction](https://biomejs.dev/linter/rules/no-unknown-function). [#2570](https://github.com/biomejs/biome/pull/2570) Contributed by @neokidev
- Add [nursery/noUnknownMediaFeatureName](https://biomejs.dev/linter/rules/no-unknown-media-feature-name). [#2751](https://github.com/biomejs/biome/issues/2751) Contributed by @Kazuhiro-Mimaki
- Add [nursery/noUnknownProperty](https://biomejs.dev/linter/rules/no-unknown-property). [#2755](https://github.com/biomejs/biome/pull/2755) Contributed by @chansuke
- Add [nursery/noUnknownPseudoElement](https://biomejs.dev/linter/rules/no-unknown-selector-pseudo-element). [#2655](https://github.com/biomejs/biome/issues/2655) Contributed by @keita-hino
- Add [nursery/noUnknownUnit](https://biomejs.dev/linter/rules/no-unknown-unit). [#2535](https://github.com/biomejs/biome/issues/2535) Contributed by @neokidev
- Add [nursery/noUnmatchableAnbSelector](https://biomejs.dev/linter/rules/no-unmatchable-anb-selector). [#2706](https://github.com/biomejs/biome/issues/2706) Contributed by @togami2864
- Add [nursery/useGenericFontNames](https://biomejs.dev/linter/rules/use-generic-font-names). [#2573](https://github.com/biomejs/biome/pull/2573) Contributed by @togami2864
- Add [nursery/noYodaExpression](https://biomejs.dev/linter/rules/no-yoda-expression/). Contributed by @michellocana
- Add [nursery/noUnusedFunctionParameters](https://biomejs.dev/linter/rules/no-unused-function-parameters/) Contributed by @printfn
- Add [nursery/UseSemanticElements](https://biomejs.dev/linter/rules/use-semantic-elements/). Contributed by @fujiyamaorange

#### Enhancements

- Add a code action for [noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type/) and improve the diagnostics.

  The rule now suggests using `undefined` instead of `void` in confusing places.
  The diagnosis is also clearer.

  Contributed by @Conaclos
- Improve code action for [nursery/noUselessUndefinedInitialization](https://biomejs.dev/linter/rules/no-useless-undefined-initialization/) to handle comments.

  The rule now places inline comments after the declaration statement, instead of removing them.
  The code action is now safe to apply.

  Contributed by @lutaok

- Make [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) report duplicate dependencies. Contributed by @tunamaguro

- Rename `noEvolvingAny` into `noEvolvingTypes` ([#48](https://github.com/biomejs/website/issues/48)). Contributed by @Conaclos

#### Bug fixes

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) and [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports) now correctly handle import namespaces ([#2796](https://github.com/biomejs/biome/issues/2796)).

  Previously, Biome bound unqualified type to import namespaces.
  Import namespaces can only be used as qualified names in a type (ambient) context.

  ```ts
  // Unused import
  import * as Ns1 from "";
  // This doesn't reference the import namespace `Ns1`
  type T1 = Ns1; // Undeclared variable `Ns1`

  // Unused import
  import type * as Ns2 from "";
  // This doesn't reference the import namespace `Ns2`
  type T2 = Ns2; // Undeclared variable `Ns2`

  import type * as Ns3 from "";
  // This references the import namespace because it is a qualified name.
  type T3 = Ns3.Inner;
  // This also references the import namespace.
  export type { Ns3 }
  ```

  Contributed by @Conaclos

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) now correctly handle ambient computed member names ([#2975](https://github.com/biomejs/biome/issues/2975)).

  A constant can be imported as a type and used in a computed member name of a member signature.
  Previously, Biome was unable to bind the value imported as a type to the computed member name.

  ```ts
  import type { NAME } from "./constants.js";
  type X = { [NAME]: number };
  ```

  Contributed by @Conaclos

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) now ignores `this` in JSX components ([#2636](https://github.com/biomejs/biome/issues/2636)).

  The rule no longer reports `this` as undeclared in following code.

  ```jsx
  import { Component } from 'react';

  export class MyComponent extends Component {
    render() {
      return <this.foo />
    }
  }
  ```

  Contributed by @printfn and @Conaclos

- `useJsxKeyInIterable` now handles more cases involving fragments. See the snippets below. Contributed by @dyc3
```jsx
// valid
[].map((item) => {
	return <>{item.condition ? <div key={item.id} /> : <div key={item.id}>foo</div>}</>;
});

// invalid
[].map((item) => {
	return <>{item.condition ? <div /> : <div>foo</div>}</>;
});
```
- `noExcessiveNestedTestSuites` no longer erroneously alerts on `describe` calls that are not invoking the global `describe` function. [#2599](https://github.com/biomejs/biome/issues/2599) Contributed by @dyc3
```js
// now valid
z.object({})
  .describe('')
  .describe('')
  .describe('')
  .describe('')
  .describe('')
  .describe('');
```
- `noEmptyBlockStatements` no longer reports empty constructors using typescript parameter properties. [#3005](https://github.com/biomejs/biome/issues/3005) Contributed by @dyc3
- `noEmptyBlockStatements` no longer reports empty private or protected constructors. Contributed by @dyc3

- [noExportsInTest](https://biomejs.dev/linter/rules/no-exports-in-test/) rule no longer treats files with in-source testing as test files https://github.com/biomejs/biome/issues/2859. Contributed by @ah-yu
- [useSortedClasses](https://biomejs.dev/linter/rules/use-sorted-classes/) now keeps leading and trailing spaces when applying the code action inside template literals:

  ```
  i Unsafe fix: Sort the classes.

    1 1 │   <>
    2   │ - → <div·class={`${variable}·px-2·foo·p-4·bar`}/>
      2 │ + → <div·class={`${variable}·foo·bar·p-4·px-2`}/>
    3 3 │   	<div class={`px-2 foo p-4 bar ${variable}`}/>
    4 4 │   </>
  ```
- [noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies/) is correctly triggered when running `biome ci`. Contributed by @ematipico
- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer panics when a certain combination of characters is typed. Contributed by @ematipico

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) no logger alerts on `arguments` object in a function scope. Contributed by @ah-yu
### Parser

#### Enhancements

- `lang="tsx"` is now supported in Vue Single File Components. [#2765](https://github.com/biomejs/biome/issues/2765) Contributed by @dyc3

#### Bug fixes

- The `const` modifier for type parameters is now accepted for TypeScript `new` signatures ([#2825](https://github.com/biomejs/biome/issues/2825)).

  The following code is now correctly parsed:

  ```ts
  interface I {
    new<const T>(x: T): T
  }
  ```

  Contributed by @Conaclos

- Some invalid TypeScript syntax caused the Biome parser to crash.

  The following invalid syntax no longer causes the Biome parser to crash:

  ```ts
  declare using x: null;
  declare qwait using x: null;
  ```

  Contributed by @Conaclos

## 1.7.3 (2024-05-06)

### CLI

#### Bug fixes

- The [stdin-file-path](https://biomejs.dev/guides/integrate-in-editor/#use-stdin) option now works correctly for Astro/Svelte/Vue files ([#2686](https://github.com/biomejs/biome/pull/2686))

  Fix [#2225](https://github.com/biomejs/biome/issues/2225) where lint output become empty for Vue files.

  Contributed by @tasshi-me

- `biome migrate eslint` now correctly resolve `@scope/eslint-config` ([#2705](https://github.com/biomejs/biome/issues/2705)). Contributed by @Conaclos

### Linter

#### New features

- Add [nursery/noUselessStringConcat](https://biomejs.dev/linter/rules/no-useless-string-concat/).
- Add [nursery/useExplicitLengthCheck](https://biomejs.dev/linter/rules/use-explicit-length-check/). Contributed by @minht11

- `useExhaustiveDependencies` now recognizes (some) dependencies that change on
  every render ([#2374](https://github.com/biomejs/biome/issues/2374)).
  Contributed by @arendjr

#### Bug fixes

- [noBlankTarget](https://biomejs.dev/linter/rules/no-blank-target/) no longer hangs when applying a code fix ([#2675](https://github.com/biomejs/biome/issues/2675)).

  Previously, the following code made Biome hangs when applying a code fix.

  ```jsx
  <a href="https://example.com" rel="" target="_blank"></a>
  ```

  Contributed by @Conaclos

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/) no longer panics on conditional type ([#2659](https://github.com/biomejs/biome/issues/2659)).

  This is a regression introduced by [#2394](https://github.com/biomejs/biome/issues/2394).
  This regression makes `noRedeclare` panics on every conditional types with `infer` bindings.

  Contributed by @Conaclos

- [noUnusedLabels](https://biomejs.dev/linter/rules/no-unused-labels/) and [noConfusingLabels](https://biomejs.dev/linter/rules/no-confusing-labels/) now ignore svelte reactive statements ([#2571](https://github.com/biomejs/biome/issues/2571)).

  The rules now ignore reactive Svelte blocks in Svelte components.

  ```svelte
  <script>
  $: { /* reactive block */ }
  </script>
  ```

  Contributed by @Conaclos

- [useExportType](https://biomejs.dev/linter/rules/use-export-type/) no longer removes leading comments ([#2685](https://github.com/biomejs/biome/issues/2685)).

  Previously, `useExportType` removed leading comments when it factorized the `type` qualifier.
  It now provides a code fix that preserves the leading comments:

  ```diff
  - export {
  + export type {
      /**leading comment*/
  -   type T
  +   T
    }
  ```

  Contributed by @Conaclos

- [useJsxKeyInIterable](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/) no longer reports false positive when iterating on non-jsx items ([#2590](https://github.com/biomejs/biome/issues/2590)).

  The following snipet of code no longer triggers the rule:

  ```jsx
  <>{data.reduce((total, next) => total + next, 0)}</>
  ```

  Contributed by @dyc3

- Fix typo by renaming `useConsistentBuiltinInstatiation` to `useConsistentBuiltinInstantiation`
  Contributed by @minht11
- Fix the rule `useSingleCaseStatement` including `break` statements when counting the number of statements in a `switch` statement (#2696)


## 1.7.2 (2024-04-30)

### Analyzer

#### Bug fixes

- Import sorting now ignores side effect imports ([#817](https://github.com/biomejs/biome/issues/817)).

  A side effect import consists now in its own group.
  This ensures that side effect imports are not reordered.

  Here is an example of how imports are now sorted:

  ```diff
    import "z"
  - import { D } from "d";
    import { C } from "c";
  + import { D } from "d";
    import "y"
    import "x"
  - import { B } from "b";
    import { A } from "a";
  + import { B } from "b";
    import "w"
  ```

  Contributed by @Conaclos

- Import sorting now adds spaces where needed ([#1665](https://github.com/biomejs/biome/issues/1665))
  Contributed by @Conaclos

### CLI

#### Bug fixes

- `biome migrate eslint` now handles cyclic references.

  Some plugins and configurations export objects with cyclic references.
  This causes `biome migrate eslint` to fail or ignore them.
  These edge cases are now handled correctly.

  Contributed by @Conaclos

### Formatter

#### Bug fixes

- Correctly handle placement of comments inside named import clauses. [#2566](https://github.com/biomejs/biome/pull/2566). Contributed by @ah-yu

### Linter

#### New features

- Add [nursery/noReactSpecificProps](https://biomejs.dev/linter/rules/no-react-specific-props/).
  Contributed by @marvin-j97

- Add [noUselessUndefinedInitialization](https://biomejs.dev/linter/rules/no-useless-undefined-initialization/).
  Contributed by @lutaok

- Add [nursery/useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/).
  Contributed by @Kazuhiro-Mimaki

- Add [nursery/useConsistentBuiltinInstatiation](https://biomejs.dev/linter/rules/use-consistent-builtin-instantiation/).
  Contributed by @minht11

- Add [nursery/useDefaultSwitchClause](https://biomejs.dev/linter/rules/use-default-switch-clause/).
  Contributed by @michellocana

#### Bug fixes

- [noDuplicateJsonKeys](https://biomejs.dev/linter/rules/no-duplicate-json-keys/) no longer crashes when a JSON file contains an unterminated string ([#2357](https://github.com/biomejs/biome/issues/2357)).
  Contributed by @Conaclos

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/) now reports redeclarations of parameters in a functions body ([#2394](https://github.com/biomejs/biome/issues/2394)).

  The rule was unable to detect redeclarations of a parameter or a type parameter in the function body.
  The following two redeclarations are now reported:

  ```ts
  function f<T>(a) {
    type T = number; // redeclaration
    const a = 0; // redeclaration
  }
  ```

  Contributed by @Conaclos

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/) no longer reports overloads in object types ([#2608](https://github.com/biomejs/biome/issues/2608)).

  The rule no longer report redeclarations in the following code:

  ```ts
  type Overloads = {
    ({ a }: { a: number }): number,
    ({ a }: { a: string }): string,
  };
  ```

  Contributed by @Conaclos

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/) now merge default function export declarations and types ([#2372](https://github.com/biomejs/biome/issues/2372)).

  The following code is no longer reported as a redeclaration:

  ```ts
  interface Foo {}
  export default function Foo() {}
  ```

  Contributed by @Conaclos

- [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) no longer reports variable-only and type-only exports ([#2637](https://github.com/biomejs/biome/issues/2637)).
  Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) no longer crash Biome when encountering a malformed conditional type ([#1695](https://github.com/biomejs/biome/issues/1695)).
  Contributed by @Conaclos

- [useConst](https://biomejs.dev/linter/rules/use-const/) now ignores a variable that is read before its assignment.

  Previously, the rule reported the following example:

  ```js
  let x;
  x; // read
  x = 0; // write
  ```

  It is now correctly ignored.

  Contributed by @Conaclos

- [useShorthandFunctionType](https://biomejs.dev/linter/rules/use-shorthand-function-type/) now suggests correct code fixes when parentheses are required ([#2595](https://github.com/biomejs/biome/issues/2595)).

  Previously, the rule didn't add parentheses when they were needed.
  It now adds parentheses when the function signature is inside an array, a union, or an intersection.

  ```diff
  - type Union = { (): number } | string;
  + type Union = (() => number) | string;
  ```

  Contributed by @Conaclos

- [useTemplate](https://biomejs.dev/linter/rules/use-template/) now correctly escapes strings ([#2580](https://github.com/biomejs/biome/issues/2580)).

  Previously, the rule didn't correctly escape characters preceded by an escaped character.

  Contributed by @Conaclos

- [noMisplacedAssertion](https://biomejs.dev/linter/rules/no-misplaced-assertion/) now allow these matchers

  - `expect.any()`
  - `expect.anything()`
  - `expect.closeTo`
  - `expect.arrayContaining`
  - `expect.objectContaining`
  - `expect.stringContaining`
  - `expect.stringMatching`
  - `expect.extend`
  - `expect.addEqualityTesters`
  - `expect.addSnapshotSerializer`

  Contributed by @fujiyamaorange

### Parser

#### Bug fixes

- The language parsers no longer panic on unterminated strings followed by a newline and a space ([#2606](https://github.com/biomejs/biome/issues/2606), [#2410](https://github.com/biomejs/biome/issues/2410)).

  The following example is now parsed without making Biome panics:

  ```
  "
   "
  ```

  Contributed by @Conaclos


## 1.7.1 (2024-04-22)

### Editors

#### Bug fixes

- Fix [#2403](https://github.com/biomejs/biome/issues/2403) by printing the errors in the client console. Contributed by @ematipico

### Formatter

#### Bug fixes

- Add parentheses for the return expression that has leading multiline comments. [#2504](https://github.com/biomejs/biome/pull/2504). Contributed by @ah-yu

- Correctly format dangling comments of continue statements. [#2555](https://github.com/biomejs/biome/pull/2555). Contributed by @ah-yu

- Prevent comments from being eaten by the formatter [#2578](https://github.com/biomejs/biome/pull/2578). Now the comments won't be eaten for the following code:
  ```js
  console.log((a,b/* comment */));
  ```
  Contributed by @ah-yu

- Correctly format nested union type to avoid reformatting issue. [#2628](https://github.com/biomejs/biome/pull/2628). Contributed by @ah-yu

### Linter

#### Bug fixes

- Fix case where `jsxRuntime` wasn't being respected by `useImportType` rule ([#2473](https://github.com/biomejs/biome/issues/2473)).Contributed by @arendjr
- Fix [#2460](https://github.com/biomejs/biome/issues/2460), where the rule `noUselessFragments` was crashing the linter in some cases. Now cases like these are correctly handled:
  ```jsx
  callFunction(<>{bar}</>)
  ```
  Contributed by @ematipico
- Fix [#2366](https://github.com/biomejs/biome/issues/2366), where `noDuplicateJsonKeys` incorrectly computed the kes to highlight. Contributed by @ematipico
#### Enhancements

- The rule `noMisplacedAssertions` now considers valid calling `expect` inside `waitFor`:
  ```js
  import { waitFor } from '@testing-library/react';

  await waitFor(() => {
    expect(111).toBe(222);
  });
  ```
  Contributed by @ematipico


## 1.7.0 (2024-04-15)

### Analyzer

#### Bug fixes

- Now Biome can detect the script language in Svelte and Vue script blocks more reliably ([#2245](https://github.com/biomejs/biome/issues/2245)). Contributed by @Sec-ant

- `useExhaustiveDependencies` no longer reports recursive calls as missing
  dependencies ([#2361](https://github.com/biomejs/biome/issues/2361)).
  Contributed by @arendjr

- `useExhaustiveDependencies` correctly reports missing dependencies declared
  using function declarations ([#2362](https://github.com/biomejs/biome/issues/2362)).
  Contributed by @arendjr

- Biome now can handle `.svelte` and `.vue` files with `CRLF` as the end-of-line sequence. Contributed by @Sec-ant

- `noMisplacedAssertion` no longer reports method calls by `describe`, `test`, `it` objects (e.g. `test.each([])()`) ([#2443](https://github.com/biomejs/biome/issues/2443)). Contributed by @unvalley.

- Biome now can handle `.vue` files with [generic components](https://vuejs.org/api/sfc-script-setup#generics) ([#2456](https://github.com/biomejs/biome/issues/2456)).
  ```vue
  <script generic="T extends Record<string, any>" lang="ts" setup>
  //...
  </script>
  ```
  Contributed by @Sec-ant

#### Enhancements

- Complete the well-known file lists for JSON-like files. Trailing commas are allowed in `.jsonc` files by default. Some well-known files like `tsconfig.json` and `.babelrc` don't use the `.jsonc` extension but still allow comments and trailing commas. While others, such as `.eslintrc.json`, only allow comments. Biome is able to identify these files and adjusts the `json.parser.allowTrailingCommas` option accordingly to ensure they are correctly parsed. Contributed by @Sec-ant

- Fix dedent logic inconsistent with prettier where the indent-style is space and the indent-width is not 2. Contributed by @mdm317

### CLI

#### New features

- Add a command to migrate from ESLint

  `biome migrate eslint` allows you to migrate an ESLint configuration to Biome.
  The command supports [legacy ESLint configurations](https://eslint.org/docs/latest/use/configure/configuration-files) and [new flat ESLint configurations](https://eslint.org/docs/latest/use/configure/configuration-files-new).
  Legacy ESLint configurations using the YAML format are not supported.

  When loading a legacy ESLint configuration, Biome resolves the `extends` field.
  It resolves both shared configurations and plugin presets!
  To do this, it invokes _Node.js_.

  Biome relies on the metadata of its rules to determine the [equivalent rule of an ESLint rule](https://biomejs.dev/linter/rules-sources/).
  A Biome rule is either inspired or roughly identical to an ESLint rules.
  By default, inspired and nursery rules are excluded from the migration.
  You can use the CLI flags `--include-inspired` and `--include-nursery` to migrate them as well.

  Note that this is a best-effort approach.
  You are not guaranteed to get the same behavior as ESLint.

  Given the following ESLint configuration:

  ```json
  {
        "ignore_patterns": ["**/*.test.js"],
        "globals": { "var2": "readonly" },
        "rules": {
            "eqeqeq": "error"
        },
        "overrides": [{
            "files": ["lib/*.js"],
            "rules": {
              "default-param-last": "off"
            }
        }]
  }
  ```

  `biome migrate eslint --write` changes the Biome configuration as follows:

  ```json
  {
    "linter": {
      "rules": {
        "recommended": false,
        "suspicious": {
          "noDoubleEquals": "error"
        }
      }
    },
    "javascript": { "globals": ["var2"] },
    "overrides": [{
      "include": ["lib/*.js"],
      "linter": {
        "rules": {
          "style": {
            "useDefaultParameterLast": "off"
          }
        }
      }
    }]
  }
  ```

  Also, if the working directory contains `.eslintignore`, then Biome migrates the glob patterns.
  Nested `.eslintignore` in subdirectories and negated glob patterns are not supported.

  If you find any issue, please don't hesitate to report them.

  Contributed by @Conaclos

- Added two new options to customise the emitted output of the CLI: `--reporter=json` and `--reporter=json-pretty`. With `--reporter=json`, the diagnostics and the
  summary will be printed in the **terminal** in JSON format. With `--reporter=json-pretty`, you can print the same information, but formatted using the same options of your configuration.

  NOTE: the shape of the JSON is considered experimental, and the shape of the JSON might change in the future.

  <details>
  <summary>Example of output when running `biome format` command</summary>
  ```json
  {
    "summary": {
      "changed": 0,
      "unchanged": 1,
      "errors": 1,
      "warnings": 0,
      "skipped": 0,
      "suggestedFixesSkipped": 0,
      "diagnosticsNotPrinted": 0
    },
    "diagnostics": [
      {
        "category": "format",
        "severity": "error",
        "description": "Formatter would have printed the following content:",
        "message": [
          {
            "elements": [],
            "content": "Formatter would have printed the following content:"
          }
        ],
        "advices": {
          "advices": [
            {
              "diff": {
                "dictionary": "  statement();\n",
                "ops": [
                  { "diffOp": { "delete": { "range": [0, 2] } } },
                  { "diffOp": { "equal": { "range": [2, 12] } } },
                  { "diffOp": { "delete": { "range": [0, 2] } } },
                  { "diffOp": { "equal": { "range": [12, 13] } } },
                  { "diffOp": { "delete": { "range": [0, 2] } } },
                  { "diffOp": { "insert": { "range": [13, 15] } } }
                ]
              }
            }
          ]
        },
        "verboseAdvices": { "advices": [] },
        "location": {
          "path": { "file": "format.js" },
          "span": null,
          "sourceCode": null
        },
        "tags": [],
        "source": null
      }
    ],
    "command": "format"
  }
  ```
  </details>

- Added new `--staged` flag to the `check`, `format` and `lint` subcommands.

  This new option allows users to apply the command _only_ to the files that are staged (the
  ones that will be committed), which can be very useful to simplify writing git hook scripts
  such as `pre-commit`. Contributed by @castarco

#### Enhancements

- Improve support of `.prettierignore` when migrating from Prettier

  Now, Biome translates most of the glob patterns in `.prettierignore` to the equivalent Biome ignore pattern.
  Only negated glob patterns are not supported.

  Contributed by @Conaclos

- Support JavaScript configuration files when migrating from Prettier

  `biome migrate prettier` is now able to migrate Prettier configuration files
  ending with `js`, `mjs`, or `cjs` extensions.
  To do this, Biome invokes Node.js.

  Also, embedded Prettier configurations in `package.json` are now supported.

  Contributed by @Conaclos

- Support `overrides` field in Prettier configuration files when migrating from Prettier.
  Contributed by @Conaclos

- Support passing a file path to the `--config-path` flag or the `BIOME_CONFIG_PATH` environment variable.

  Now you can pass a `.json`/`.jsonc` file path with any filename to the `--config-path` flag or the
  `BIOME_CONFIG_PATH` environment variable. This will disable the configuration auto-resolution and Biome
  will try to read the configuration from the said file path ([#2265](https://github.com/biomejs/biome/issues/2265)).

  ```shell
  biome format --config-path=../biome.json ./src
  ```

  Contributed by @Sec-ant

#### Bug fixes

- Biome now tags the diagnostics emitted by `organizeImports` and `formatter` with correct severity levels, so they will be properly filtered by the flag `--diagnostic-level` ([#2288](https://github.com/biomejs/biome/issues/2288)). Contributed by @Sec-ant

- Biome now correctly filters out files that are not present in the current directory when using the `--changed` flag [#1996](https://github.com/biomejs/biome/issues/1996). Contributed by @castarco

- Biome now skips traversing `fifo` or `socket` files ([#2311](https://github.com/biomejs/biome/issues/2311)). Contributed by @Sec-ant

- Biome now resolves configuration files exported from external libraries in `extends` from the working directory (CLI) or project root (LSP). This is the documented behavior and previous resolution behavior is considered as a bug ([#2231](https://github.com/biomejs/biome/issues/2231)). Contributed by @Sec-ant

### Configuration

#### Bug fixes

- Now setting group level `all` to `false` can disable recommended rules from that group when top level `recommended` is `true` or unset. Contributed by @Sec-ant

- Biome configuration files can correctly extends `.jsonc` configuration files now ([#2279](https://github.com/biomejs/biome/issues/2279)). Contributed by @Sec-ant

- Fixed the JSON schema for React hooks configuration ([#2396](https://github.com/biomejs/biome/issues/2396)). Contributed by @arendjr

#### Enhancements

- Biome now displays the location of a parsing error for its configuration file ([#1627](https://github.com/biomejs/biome/issues/1627)).

  Previously, when Biome encountered a parsing error in its configuration file,
  it didn't indicate the location of the error.
  It now displays the name of the configuration file and the range where the error occurred.

  Contributed by @Conaclos

- `options` is no longer required for rules without any options ([#2313](https://github.com/biomejs/biome/issues/2313)).

  Previously, the JSON schema required to set `options` to `null` when an object is used to set the diagnostic level of a rule without any option.
  However, if `options` is set to `null`, Biome emits an error.

  The schema is now fixed and it no longer requires specifying `options`.
  This makes the following configuration valid:

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "noDefaultExport": {
            "level": "off"
          }
        }
      }
    }
  }
  ```

  Contributed by @Conaclos

### Editors

#### Bug fixes

- Biome extension is now able to parse the JSX syntax in files that associated with the `javascript` [language identifier](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem). This is an ad hoc fix, because [in the React world, `.js` files are allowed to include JSX syntax](https://github.com/facebook/create-react-app/issues/87#issuecomment-234627904), and these files are often associated with the `javascript` language identifier in most of the editors. Plus, [some editor extensions](https://github.com/michaelgmcd/vscode-language-babel/blob/8b3a472748ad07c99dc022b66795c9eb46be4ccb/package.json#L63-L80) will also associate `.jsx` files with the `javascript` language identifier. Relative links: [discussion](https://github.com/biomejs/biome/discussions/838#discussioncomment-9047539), [#2085](https://github.com/biomejs/biome/issues/2085). Contributed by @Sec-ant

### Formatter

#### Bug fixes

- Fix [#2291](https://github.com/biomejs/biome/issues/2291) by correctly handle comment placement for JSX spread attributes and JSX spread children. Contributed by @ah-yu

### JavaScript APIs

### Linter

#### Promoted rules

New rules are incubated in the nursery group.
Once stable, we promote them to a stable group.
The following rules are promoted:

- [complecity/noExcessiveNestedTestSuites](https://biomejs.dev/linter/rules/no-excessive-nested-test-suites)
- [complexity/noUselessTernary](https://biomejs.dev/linter/rules/no-useless-ternary)
- [correctness/useJsxKeyInIterable](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable)
- [performance/noBarrelFile](https://biomejs.dev/linter/rules/no-barrel-file/)
- [performance/noReExportAll](https://biomejs.dev/linter/rules/no-re-export-all/)
- [style/noNamespaceImport](https://biomejs.dev/linter/rules/no-namespace-import/)
- [style/useNodeAssertStrict](https://biomejs.dev/linter/rules/use-node-assert-strict/)
- [suspicious/noDuplicateTestHooks](https://biomejs.dev/linter/rules/no-duplicate-test-hooks/)
- [suspicious/noExportsInTest](https://biomejs.dev/linter/rules/no-exports-in-test/)
- [suspicious/noFocusedTests](https://biomejs.dev/linter/rules/no-focused-tests/)
- [suspicious/noSkippedTests](https://biomejs.dev/linter/rules/no-skipped-tests/)
- [suspicious/noSuspiciousSemicolonInJsx](https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx)

#### New features

- Add a new option `jsxRuntime` to the `javascript` configuration. When set to `reactClassic`, the [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports) and [useImportType](https://biomejs.dev/linter/rules/use-import-type) rules use this information to make exceptions for the React global that is required by the React Classic JSX transform.

  This is only necessary for React users who haven't upgraded to the [new JSX transform](https://legacy.reactjs.org/blog/2020/09/22/introducing-the-new-jsx-transform.html).

  Contributed by @Conaclos and @arendjr

- Implement [#2043](https://github.com/biomejs/biome/issues/2043): The React rule [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) is now also compatible with Preact hooks imported from `preact/hooks` or `preact/compat`. Contributed by @arendjr

- Add rule [noFlatMapIdentity](https://biomejs.dev/linter/rules/no-flat-map-identity) to disallow unnecessary callback use on `flatMap`. Contributed by @isnakode

- Add rule [noConstantMathMinMaxClamp](https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp), which disallows using `Math.min` and `Math.max` to clamp a value where the result itself is constant. Contributed by @mgomulak

#### Enhancements

- [style/useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention/) now allows prefixing a filename with `+` ([#2341](https://github.com/biomejs/biome/issues/2341)).

  This is a convention used by [Sveltekit](https://kit.svelte.dev/docs/routing#page) and [Vike](https://vike.dev/route).

  Contributed by @Conaclos

- [style/useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) now accepts `PascalCase` for local and top-level variables.

  This allows supporting local variables that hold a component or a regular class.
  The following code is now accepted:

  ```tsx
  function loadComponent() {
    const Component = getComponent();
    return <Component />;
  }
  ```

  Contributed by @Conaclos

- [complexity/useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) no longer report computed properties named `__proto__` ([#2430](https://github.com/biomejs/biome/issues/2430)).

  In JavaScript, `{["__proto__"]: null}` and `{__proto__: null}` have not the same semantic.
  The first code set a regular property to `null`.
  The second one set the prototype of the object to `null`.
  See the [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto) for more details.

  The rule now ignores computed properties named `__proto__`.

  Contributed by @Conaclos

#### Bug fixes

- Lint rules `useNodejsImportProtocol`, `useNodeAssertStrict`, `noRestrictedImports`, `noNodejsModules` will no longer check `declare module` statements anymore. Contributed by @Sec-ant

- [style/useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) now accepts any case for variables from object destructuring ([#2332](https://github.com/biomejs/biome/issues/2332)).

  The following name is now ignored:

  ```js
  const { Strange_Style } = obj;
  ```

  Previously, the rule renamed this variable. This led to a runtime error.

  Contributed by @Conaclos

### Parser

#### Bug fixes

- Fixed an issue when Unicode surrogate pairs were encoded in JavaScript strings
  using an escape sequence ([#2384](https://github.com/biomejs/biome/issues/2384)).
  Contributed by @arendjr


## 1.6.4 (2024-04-03)

### Analyzer

#### Bug fixes

- An operator with no spaces around in a binary expression no longer breaks the js analyzer ([#2243](https://github.com/biomejs/biome/issues/2243)). Contributed by @Sec-ant

### CLI

#### Bug fixes

- Fix the printed error count ([#2048](https://github.com/biomejs/biome/issues/2048)). Contributed by @Sec-ant

### Configuration

#### Bug fixes

- Correctly calculate enabled rules in lint rule groups. Now a specific rule belonging to a group can be enabled even if its group-level preset option `recommended` or `all` is `false` ([#2191](https://github.com/biomejs/biome/issues/2191)). Contributed by @Sec-ant

### Editors

#### Bug fixes

- Fix the unexpected code deletion and repetition when `quickfix.biome` is enabled and some `import`-related rules are applied ([#2222](https://github.com/biomejs/biome/issues/2222), [#688](https://github.com/biomejs/biome/issues/688), [#1015](https://github.com/biomejs/biome/issues/1015)). Contributed by @Sec-ant

### Linter

#### New features

- Add [nursery/noMisplacedAssertion](https://biomejs.dev/linter/rules/no-misplaced-assertion/). COntributed by @ematipico

#### Bug fixes

- Fix [#2211](https://github.com/biomejs/biome/issues/2211). noChildrenProp should work fine when children pass as a prop in a new line. Contributed by @fireairforce

- Fix [#2248](https://github.com/biomejs/biome/issues/2248). `lint/a11y/useButtonType` should not trigger when button element with spread attribute. Contributed by @fireairforce

- Fix [#2216](https://github.com/biomejs/biome/issues/2216). `lint/style/useNamingConvention` should not ignore JSX Component name binding. Contributed by @fireairforce

#### Enhancements

- Add support for object property members in the rule `useSortedClasses`. Contributed by @ematipico

### Parser

- The parser doesn't throw any error when the frontmatter of `.astro` files contains an illegal return:

  ```astro
  ---
  const condition = true;
  if (condition) {
    return "Something";
  }
  ---
  <div></div>
  ```
  Contributed by @ematipico

## 1.6.3 (2024-03-25)

### CLI

#### Bug fixes

- Fix configuration resolution. Biome is now able to correctly find the `biome.jsonc` configuration file when `--config-path` is explicitly set ([#2164](https://github.com/biomejs/biome/issues/2164)). Contributed by @Sec-ant

- JavaScript/TypeScript files of different variants (`.ts`, `.js`, `.tsx`, `.jsx`) in a single workspace now have stable formatting behaviors when running the CLI command in paths of different nested levels or in different operating systems ([#2080](https://github.com/biomejs/biome/issues/2080), [#2109](https://github.com/biomejs/biome/issues/2109)). Contributed by @Sec-ant

### Configuration

#### Bug fixes

- Complete the documentation and overrides support for options `formatter.lineEnding`, `[language].formatter.lineEnding`, `formatter.attributePosition` and `javascript.formatter.attributePosition`. Contributed by @Sec-ant

### Formatter

#### Bug fixes

- Fix [#2172](https://github.com/biomejs/biome/issues/2172) by breaking long object destructuring patterns. Contributed by @ah-yu

### Linter

#### New features

- Add rule [noEvolvingTypes](https://biomejs.dev/linter/rules/no-evolving-any) to disallow variables from evolving into `any` type through reassignments. Contributed by @fujiyamaorange

#### Enhancements

- Rename `noSemicolonInJsx` to `noSuspiciousSemicolonInJsx`. Contributed by @fujiyamaorange

### LSP

#### Bug fixes

- Quickfix action no longer autofixes lint rule errors on save when `linter` is disabled ([#2161](https://github.com/biomejs/biome/issues/2161)). Contributed by @Sec-ant
- Range formatting for Astro/Svelte/Vue doesn't place code out of place, especially when formatting on paste is enabled. Contributed by @ematipico

## 1.6.2 (2024-03-22)

### Analyzer

#### Bug fixes

- The `noSuperWithoutExtends` rule now allows for calling `super()` in derived class constructors of class expressions ([#2108](https://github.com/biomejs/biome/issues/2108)). Contributed by @Sec-ant

- Fix discrepancies on file source detection. Allow module syntax in `.cts` files ([#2114](https://github.com/biomejs/biome/issues/2114)). Contributed by @Sec-ant

### CLI

#### Bug fixes

- Fixes [#2131](https://github.com/biomejs/biome/issues/2131), where folders were incorrectly ignored when running the command `check`. Now folders are correctly ignored based on their command. Contributed by @ematipico

- Smoother handling of `"endOfLine": "auto"` in prettier migration: falling back to `"lf"` ([#2145](https://github.com/biomejs/biome/pull/2145)). Contributed by @eMerzh

### Configuration

#### Bug fixes

- Fix enabled rules calculation. The precendence of individual rules, `all` and `recommend` presets in top-level and group-level configs is now correctly respected. More details can be seen in ([#2072](https://github.com/biomejs/biome/pull/2072)) ([#2028](https://github.com/biomejs/biome/issues/2028)). Contributed by @Sec-ant

### Formatter

#### Bug fixes

- Fix [#1661](https://github.com/biomejs/biome/issues/1661). Now nested conditionals are aligned with Prettier's logic, and won't contain mixed spaces and tabs. Contributed by @ematipico

### JavaScript APIs

#### Enhancements

- Support applying lint fixes when calling the `lintContent` method of the `Biome` class ([#1956](https://github.com/biomejs/biome/pull/1956)). Contributed by @mnahkies

### Linter

#### New features

- Add [nursery/noDuplicateElseIf](https://biomejs.dev/linter/rules/no-duplicate-else-if/). COntributed by @mdm317

#### Bug fixes

- Rule `noUndeclaredDependencies` now also validates `peerDependencies` and `optionalDependencies` ([#2122](https://github.com/biomejs/biome/issues/2122)). Contributed by @Sec-ant

- Rule `noUndeclaredDependencies` won't check `declare module` statements anymore ([#2123](https://github.com/biomejs/biome/issues/2123)). Contributed by @Sec-ant

- Fix [#1925](https://github.com/biomejs/biome/issues/1925). The fix for `useOptionalChain` would sometimes suggest an incorrect fix that discarded optional chaining operators on the left-hand side of logical expressions. These are now preserved. Contributed by @arendjr

- Rule `noUndeclaredVariables` now also checks for worker globals ([#2121](https://github.com/biomejs/biome/issues/2121)). Contributed by @Sec-ant

### LSP

#### Bug fixes

- Correctly parse `.jsonc` files. Contributed by @Sec-ant

- Correctly resolve external `extends` configs. Contributed by @Sec-ant

## 1.6.1 (2024-03-12)

### CLI

#### Bug fixes

- CLI is now able to automatically search and resolve `biome.jsonc` ([#2008](https://github.com/biomejs/biome/issues/2008)). Contributed by @Sec-ant
- Fix a false positive where some files were counted as "fixed" even though they weren't modified. Contributed by @ematipico

### Configuration

#### Bug fixes

- `json.formatter.trailingCommas` option now works in `overrides` ([#2009](https://github.com/biomejs/biome/issues/2009)). Contributed by @Sec-ant

### Linter

#### New features

- Add rule [noDoneCallback](https://biomejs.dev/linter/rules/no-done-callback), this rule checks the function parameter of hooks & tests
  for use of the done argument, suggesting you return a promise instead. Contributed by @vasucp1207

  ```js
  beforeEach(done => {
    // ...
  });
  ```

#### Bug fixes

- [useJsxKeyInIterable](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable) now recognizes function bodies wrapped in parentheses ([#2011](https://github.com/biomejs/biome/issues/2011)). Contributed by @Sec-ant

- [useShorthandFunctionType](https://biomejs.dev/linter/rules/use-shorthand-function-type) now preserves type parameters of generic interfaces when applying fixes ([#2015](https://github.com/biomejs/biome/issues/2015)). Contributed by @Sec-ant

- Code fixes of [useImportType](https://biomejs.dev/linter/rules/use-import-type) and [useExportType](https://biomejs.dev/linter/rules/use-export-type) now handle multiline statements ([#2041](https://github.com/biomejs/biome/issues/2041)). Contributed by @Conaclos

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare) no longer reports type parameter and parameter with identical names ([#1992](https://github.com/biomejs/biome/issues/1992)).

  The following code is no longer reported:

  ```ts
  function f<a>(a: a) {}
  ```

  Contributed by @Conaclos

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare) now reports duplicate type parameters in a same declaration.

  The following type parameters are now reported as a redeclaration:

  ```ts
  function f<T, T>() {}
  ```

  Contributed by @Conaclos

- [noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies/) now recognizes imports of subpath exports.

  E.g., the following import statements no longer report errors if `@mui/material` and `tailwindcss` are installed as dependencies:

  ```ts
  import Button from "@mui/material/Button";
  import { fontFamily } from "tailwindcss/defaultTheme";
  ```

  Contributed by @Sec-ant

### Parser

#### Bug fixes

- JavaScript lexer is now able to lex regular expression literals with escaped non-ascii chars ([#1941](https://github.com/biomejs/biome/issues/1941)).

  Contributed by @Sec-ant

## 1.6.0 (2024-03-08)

### Analyzer

#### New features

- Add partial for `.astro` files. Biome is able to sort imports inside the frontmatter of the Astro files. Contributed
  by @ematipico

  ```diff
  ---
  - import { getLocale } from "astro:i18n";
  - import { Code } from "astro:components";
  + import { Code } from "astro:components";
  + import { getLocale } from "astro:i18n";
  ---

  <div></div>
  ```
- Add partial for `.vue` files. Biome is able to sort imports inside the script block of Vue files. Contributed by
  @nhedger

  ```diff
  <script setup lang="ts">
  - import Button from "./components/Button.vue";
  - import * as vueUse from "vue-use";
  + import * as vueUse from "vue-use";
  + import Button from "./components/Button.vue";
  </script/>

  <template></template>
  ```

- Add partial for `.svelte` files. Biome is able to sort imports inside the script block of Svelte files. Contributed by
  @ematipico

  ```diff
  <script setup lang="ts">
  - import Button from "./components/Button.svelte";
  - import * as svelteUse from "svelte-use";
  + import * as svelteUse from "svelte-use";
  + import Button from "./components/Button.svelte";
  </script/>

  <div></div>
  ```

- The analyzer now **infers** the correct quote from `javascript.formatter.quoteStyle`, if set. This means that code fixes suggested by the analyzer will use the same quote of the formatter. Contributed by @ematipico

#### Enhancements

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) ignores unused rest spread siblings.

  The following code is now valid:

  ```js
  const { a, ...rest } = { a: 0, b: 1 };
  console.log(rest);
  ```

  Contributed by @ah-yu

- Fix [#1931](https://github.com/biomejs/biome/issues/1931). Built-in React hooks such as
  `useEffect()` can now be validated by the
  [`useExhaustiveDependendies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/), even
  when they're not being imported from the React library. To do so, simply configure them like
  any other user-provided hooks.

  Contributed by @arendjr

- Implemented [#1128](https://github.com/biomejs/biome/issues/1128). User-provided React hooks can
  now be configured to track stable results. For example:

  ```json
  "useExhaustiveDependencies": {
      "level": "error",
      "options": {
          "hooks": [{
              "name": "useMyState",
              "stableResult": [
                  1
              ]
          }]
      }
  }
  ```

  This will allow the following to be validated:

  ```js
  const [myState, setMyState] = useMyState();
  const toggleMyState = useCallback(() => {
    setMyState(!myState);
  }, [myState]); // Only `myState` needs to be specified here.
  ```

  Contributed by @arendjr

#### Bug fixes

- Fix [#1748](https://github.com/biomejs/biome/issues/1748). Now for the following case we won't provide an unsafe fix
  for the `noNonNullAssertion` rule:

  ```ts
  x[y.z!];
  ```

  Contributed by @ah-yu

- Imports that contain the protocol `:` are now sorted after the `npm:` modules, and before the `URL` modules.
  Contributed by @ematipico

  ```diff
  import express from "npm:express";
  - import Component from "./component.js"
  - import { sortBy } from "virtual:utils";
  + import { sortBy } from "virtual:utils";
  + import Component from "./component.js"
  ```

- Fix [#1081](https://github.com/biomejs/biome/issues/1081). The `useAwait` rule does not report `for await...of`.
  Contributed by @unvalley

- Fix [#1827](https://github.com/biomejs/biome/issues/1827) by properly analyzing nested `try-finally` statements. Contributed by @ah-yu

- Fix [#1924](https://github.com/biomejs/biome/issues/1924) Use the correct export name to sort in the import clause. Contributed by @ah-yu
- Fix [#1805](https://github.com/biomejs/biome/issues/1805) fix formatting arrow function which has conditional expression body  Contributed by @mdm317

- Fix [#1781](https://github.com/biomejs/biome/issues/1781) by avoiding the retrieval of the entire static member expression for the reference if the static member expression does not start with the reference. Contributed by @ah-yu

### CLI

#### New features

- Add a new command `biome migrate prettier`. The command will read the file `.prettierrc`/`prettier.json`
  and `.prettierignore` and map its configuration to Biome's one.
  Due to the different nature of `.prettierignore` globs and Biome's globs, it's **highly** advised to make sure that
  those still work under Biome.

- Now the file name printed in the diagnostics is clickable. If you run the CLI from your editor, you can <kbd>
  Ctrl</kbd>/<kbd title="Cmd">⌘</kbd> + Click on the file name, and the editor will open said file. If row and columns
  are specified e.g. `file.js:32:7`, the editor will set the cursor right in that position. Contributed by @ematipico

- Add an option `--linter` to `biome rage`. The option needs to check Biome linter configuration. Contributed by
  @seitarof

- Add an option `--formatter` to `biome rage`. The option needs to check Biome formatter configuration. Contributed by
  @seitarof
- The CLI now consistently reports the number of files tha were changed, out of the total files that were analysed. Contributed by @ematipico
- The CLI now consistently shows the number of errors and warnings emitted. Contributed by @ematipico

#### Bug fixes

- Don't process files under an ignored directory.

  Previously, Biome processed all files in the traversed hierarchy,
  even the files under an ignored directory.
  Now, it completely skips the content of ignored directories.

  For now, directories cannot be ignored using `files.include` in the configuration file.
  This is a known limitation that we want to address in a future release.

  For instance, if you have a project with a folder `src` and a folder `test`,
  the following configuration doesn't completely ignore `test`.

  ```json
  {
    "files": {
      "include": ["src"]
    }
  }
  ```

  Biome will traverse `test`,
  however all files of the directory are correctly ignored.
  This can result in file system errors,
  if Biome encounters dangling symbolic links or files with higher permissions.

  To avoid traversing the `test` directory,
  you should ignore the directory using `ignore`:

  ```json
  {
    "files": {
      "include": ["src"],
      "ignore": ["test"]
    }
  }
  ```

- Fix [#1508](https://github.com/biomejs/biome/issues/1508) by excluding deleted files from being processed. Contributed
  by @ematipico

- Fix [#1173](https://github.com/biomejs/biome/issues/1173). Fix the formatting of a single instruction with commented
  in a control flow body to ensure consistency. Contributed by @mdm317

- Fix overriding of `javascript.globals`. Contributed by @arendjr
- Fix a bug where syntax rules weren't run when pulling the diagnostics. Now Biome will emit more parsing diagnostics,
  e.g.
  ```
  check.js:1:17 parse/noDuplicatePrivateClassMembers ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × Duplicate private class member "#foo"

    > 1 │ class A { #foo; #foo }
        │                 ^^^^

  ```
  Contributed by @ematipico

- Fix [#1774](https://github.com/biomejs/biome/issues/1774) by taking into account the option `--no-errors-on-unmatched` when running the CLI using `--changed`. Contributed by @antogyn

#### Enhancements

- Removed a superfluous diagnostic that was printed during the linting/check phase of a file:

  ```
  test.js check ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × The file contains diagnostics that needs to be addressed.
  ```
  Contributed by @ematipico
- The command `format` now emits parsing diagnostics if there are any, and it will terminate with a non-zero exit code. Contributed by @ematipico

### Configuration

#### New features

- Add the ability to resolve the configuration files defined inside `extends` from the `node_modules/` directory.

  If you want to resolve a configuration file that matches the specifier `@org/configs/biome`, then your `package.json`
  file must look this:

  ```json
  {
    "name": "@org/configs",
    "exports": {
      "./biome": "./biome.json"
    }
  }
  ```

  And the `biome.json` file that "imports" said configuration, will look like this:
  ```json
  {
    "extends": "@org/configs/biome"
  }
  ```
  Read the [documentation](https://biomejs.dev/guides/how-biome-works#the-extends-option) to better understand how it
  works, expectations and restrictions.

### Editors

#### Bug fixes

- Fix a regression where ignored files where formatted in the editor. Contributed by @ematipico
- Fix a bug where syntax rules weren't run when pulling the diagnostics. Now Biome will emit more parsing diagnostics,
  e.g.
  ```
  check.js:1:17 parse/noDuplicatePrivateClassMembers ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × Duplicate private class member "#foo"

    > 1 │ class A { #foo; #foo }
        │                 ^^^^

  ```
  Contributed by @ematipico

### Formatter

#### New features

- Biome now allows to format the `package.json` file. This is now the default behaviour and users can remove their
  workarounds.
  If you rely on other tools to format `package.json`, you'll have to ignore it via configuration. Contributed by
  @pattrickrice
- New formatter option `attributePosition` that have similar behavior as
  Prettier `singleAttributePerLine` [#1706](https://github.com/biomejs/biome/issues/1706). Contributed by @octoshikari
- Add partial for `.astro` files. Biome is able to format the frontmatter of the Astro files. Contributed by @ematipico

  ```diff
  ---
  - statement ( );
  + statement();
  ---

  <div></div>
  ```
- Add partial for `.vue` files. Biome is able to format the script block of Vue files. Contributed by @nhedger

  ```diff
  <script setup lang="ts">
  - statement ( );
  + statement();
  </script/>

  <template></template>
  ```

- Add partial for `.svelte` files. Biome is able to format the script block of Svelte files. Contributed by @ematipico

  ```diff
  <script setup lang="ts">
  - statement ( );
  + statement();
  </script/>

  <div></div>
  ```

#### Enhancements

- `composer.json`, `deno.json`, `jsconfig.json`, `package.json` and `tsconfig.json` are no longer protected files.

  This means that you can now format them.

  If you want to ignore these files, you can use the [files.ignore](https://biomejs.dev/reference/configuration/#filesignore) configuration:

  ```json
  {
    "files": {
      "ignore": [
        "composer.json",
        "jsconfig.json",
        "package.json",
        "tsconfig.json",
        "typescript.json",
        "deno.json",
        "deno.jsonc"
      ]
    }
  }
  ```

  The following files are still protected, and thus ignored:

  - `composer.lock`
  - `npm-shrinkwrap.json`
  - `package-lock.json`
  - `yarn.lock`

   Contributed by @pattrickrice and @Conaclos

#### Bug fixes

- Fix [#1039](https://github.com/biomejs/biome/issues/1039). Check unicode width instead of number of bytes when
  checking if regex expression is a simple argument.

  This no longer breaks.

  ```js
  s(/🚀🚀/).s().s();
  ```

   Contributed by @kalleep

- Fix [#1218](https://github.com/biomejs/biome/issues/1218), by correctly preserving empty lines in member chains.
  Contributed by @ah-yu
- Fix [#1659](https://github.com/biomejs/biome/issues/1659) and [#1662](https://github.com/biomejs/biome/issues/1662), by correctly taking into account the leading comma inside the formatter options. Contributed by @ematipico

- Fix [#1934](https://github.com/biomejs/biome/pull/1934). Fix invalid formatting of long arrow function for AsNeeded arrow parens Contributed by @fireairforce

### JavaScript APIs

### Linter

#### Promoted rules

New rules are incubated in the nursery group.
Once stable, we promote them to a stable group.
The following rules are promoted:

- [complexity/noEmptyTypeParameters](https://biomejs.dev/linter/rules/no-empty-type-parameters)
- [complexity/noUselessLoneBlockStatements](https://biomejs.dev/linter/rules/no-useless-lone-block-statements)
- [correctness/noInvalidUseBeforeDeclaration](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration)
- [correctness/noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports)
- [correctness/noUnusedPrivateClassMembers](https://biomejs.dev/linter/rules/no-unused-private-class-members)
- [security/noGlobalEval](https://biomejs.dev/linter/rules/no-global-eval)
- [style/useConsistentArrayType](https://biomejs.dev/linter/rules/use-consistent-array-type)
- [style/useExportType](https://biomejs.dev/linter/rules/use-export-type)
- [style/useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention)
- [style/useForOf](https://biomejs.dev/linter/rules/use-for-of)
- [style/useImportType](https://biomejs.dev/linter/rules/use-import-type)
- [style/useNodejsImportProtocol](https://biomejs.dev/linter/rules/use-nodejs-import-protocol)
- [style/useNumberNamespace](https://biomejs.dev/linter/rules/use-number-namespace)
- [style/useShorthandFunctionType](https://biomejs.dev/linter/rules/use-shorthand-function-type)
- [suspicious/noEmptyBlockStatements](https://biomejs.dev/linter/rules/no-empty-block-statements)
- [suspicious/noGlobalAssign](https://biomejs.dev/linter/rules/no-global-assign)
- [suspicious/noMisleadingCharacterClass](https://biomejs.dev/linter/rules/no-misleading-character-class)
- [suspicious/noThenProperty](https://biomejs.dev/linter/rules/no-then-property)
- [suspicious/useAwait](https://biomejs.dev/linter/rules/use-await)

Additionally, the following rules are now recommended:

- [suspicious/noApproximativeNumericConstant](https://biomejs.dev/linter/rules/no-approximative-numeric-constant)
- [suspicious/noMisrefactoredShorthandAssign](https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign)

#### Removed rules

- Remove `nursery/useGroupedTypeImport`. The rule [style/useImportType](https://biomejs.dev/linter/rules/use-import-type) covers the behavior of this rule.

  Note that removing a nursery rule is not considered a breaking change according to our [semantic versioning](https://biomejs.dev/internals/versioning).

  Contributed by @Conaclos

#### New features

- Add the rule [noSkippedTests](https://biomejs.dev/linter/rules/no-skipped-tests), to disallow skipped tests:

  ```js
  describe.skip("test", () => {});
  it.skip("test", () => {});
  ```
  Contributed by @ematipico

- Add the rule [noFocusedTests](https://biomejs.dev/linter/rules/no-focused-tests), to disallow skipped tests:

  ```js
  describe.only("test", () => {});
  it.only("test", () => {});
  ```
  Contributed by @ematipico

- Add rule [useSortedClasses](https://biomejs.dev/linter/rules/use-sorted-classes), to sort CSS utility classes:

  ```diff
  - <div class="px-2 foo p-4 bar" />
  + <div class="foo bar p-4 px-2" />
  ```
  Contributed by @DaniGuardiola

- Add rule [noUndeclaredDependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies), to detect the use of
  dependencies that aren't present in the `package.json`.

  The rule ignores imports using a protocol such as `node:`, `bun:`, `jsr:`, `https:`.

  Contributed by @ematipico and @Conaclos

- Add rule [noNamespaceImport](https://biomejs.dev/linter/rules/no-namespace-import), to report namespace imports:

  ```js
  import * as foo from "foo";
  ```
  Contributed by @unvalley
- Add partial support for `.astro` files. Biome is able to lint and fix the frontmatter of the Astro files. Contributed
  by @ematipico

  ```diff
  ---
  - delete a.b
  + a.b = undefined
  ---

  <div></div>
  ```

- Add partial support for `.vue` files. Biome is able to lint and fix the script block of the Vue files.

  ```diff
  <script setup lang="ts">
  - delete a.b
  + a.b = undefined
  <script>

  <template></template>
  ```

  Contributed by @nhedger

- Add rule [useNodeAssertStrict](https://biomejs.dev/linter/rules/use-node-assert-strict), which promotes the use
  of `node:assert/strict` over `node:assert`. Contributed by @ematipico

- Add rule [noExportsInTest](https://biomejs.dev/linter/rules/no-exports-in-test) which disallows `export` or `modules.exports` in files
  containing test. Contributed by @ah-yu

- Add rule [noSemicolonInJsx](https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx/) to detect possible wrong semicolons inside JSX elements.

  ```jsx
  const Component = () => {
    return (
      <div>
        <div />;
      </div>
    );
  }
  ```

  Contributed by @fujiyamaorange
- Add rule [noBarrelFile](https://biomejs.dev/linter/rules/no-barrel-file), to report the usage of barrel file:

  ```js
  export * from "foo";
  ```
  Contributed by @togami2864

- Add rule [noReExportAll](https://biomejs.dev/linter/rules/no-re-export-all/) that report `export * from "mod"`.
  Contributed by @mdm317

- Add rule [noExcessiveNestedTestSuites](https://biomejs.dev/linter/rules/no-excessive-nested-test-suites/).
  Contributed by @vasucp1207

- Add rule [useJsxKeyInIterable](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/).
  Contributed by @vohoanglong0107

#### Enhancements

- [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) now rule not triggered for jsx attributes when
   the fragment child is simple text.

  ```js
  export function SomeComponent() {
    return <div x-some-prop={<>Foo</>} />;
  }
  ```

   Also fixes code action when the fragment child is of type `JsxExpressionChild`.

  ```js
  <>
    <Hello leftIcon={<>{provider?.icon}</>} />
    {<>{provider?.icon}</>}
    <>{provider?.icon}</>
  </>
  ```

  Contributed by @vasucp1207

- [noUselessTernary](https://biomejs.dev/linter/rules/no-useless-ternary) now provides unsafe code fixes. Contributed by
  @vasucp1207

- [noApproximativeNumericConstant](https://biomejs.dev/linter/rules/no-approximative-numeric-constant) now provides
  unsafe code fixes and handle numbers without leading zero and numbers with digit separators.

  The following numbers are now reported as approximated constants.

  ```js
  3.14_15; // PI
  .4342; // LOG10E
  ```

  Contributed by @Conaclos

- [noPrecisionLoss](https://biomejs.dev/linter/rules/no-precision-loss) no longer reports number with extra zeros.

  The following numbers are now valid.

  ```js
  .1230000000000000000000000;
  1230000000000000000000000.0;
  ```

  Contributed by @Conaclos

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention) now
  supports [unicase](https://en.wikipedia.org/wiki/Unicase)
  letters ([#1786](https://github.com/biomejs/biome/issues/1786)).

  [unicase](https://en.wikipedia.org/wiki/Unicase) letters have a single case: they are neither uppercase nor lowercase.
  Previously, Biome reported names in unicase as invalid.
  It now accepts a name in unicase everywhere.

  The following code is now accepted:

  ```js
  const 안녕하세요 = { 안녕하세요: 0 };
  ```

  We still reject a name that mixes unicase characters with lowercase or uppercase characters:
  The following names are rejected:

  ```js
  const A안녕하세요 = { a안녕하세요: 0 };
  ```

  Contributed by @Conaclos

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention)
  and [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) now provides a new option `requireAscii` to require identifiers to
  be in ASCII.

  To avoid any breaking change, this option is turned off by default.
  We intend to turn it on in the next major release of Biome (Biome 2.0).

  Set the `requireAscii` rule option to `true` to require identifiers to be in ASCII.

  ```json
  {
    "linter": {
      "rules": {
        "style": {
          "useNamingConvention": { "options": { "requireAscii": false } }
        },
        "nursery": {
          "useFilenamingConvention": { "options": { "requireAscii": false } }
        }
      }
    }
  }
  ```

  Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) no longer reports unused imports.

  We now have a dedicated rule for reporting unused imports:
  [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports)

  Contributed by @Conaclos

#### Bug fixes

- Fix missing link in [noStaticOnlyClass](https://biomejs.dev/linter/rules/no-static-only-class) documentation.
  Contributed by @yndajas

- [noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type) no longer reports valid use of the void
  type in conditional types ([#1812](https://github.com/biomejs/biome/issues/1812)).

  The rule no longer reports the following code:

  ```ts
  type Conditional<T> = T extends void ? Record<string, never> : T
  ```

  Contributed by @lucasweng

- [noInvalidUseBeforeDeclaration](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration) no longer reports
  valid use of binding patterns ([#1648](https://github.com/biomejs/biome/issues/1648)).

  The rule no longer reports the following code:

  ```js
  const { a = 0, b = a } = {};
  ```

  Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) no longer reports used binding
  patterns ([#1652](https://github.com/biomejs/biome/issues/1652)).

  The rule no longer reports `a` as unused the following code:

  ```js
  const { a = 0, b = a } = {};
  export { b };
  ```

  Contributed by @Conaclos

- Fix [#1651](https://github.com/biomejs/biome/issues/1651). [noVar](https://biomejs.dev/linter/rules/no-var/) now
  ignores TsGlobalDeclaration. Contributed by @vasucp1207

- Fix [#1640](https://github.com/biomejs/biome/issues/1640). [useEnumInitializers](https://biomejs.dev/linter/rules/use-enum-initializers) code action now generates valid code when last member has a comment but no comma. Contributed by @kalleep

- Fix [#1653](https://github.com/biomejs/biome/issues/1653). Handle a shorthand value in `useForOf` to avoid the false-positive case. Contributed by @togami2864

- Fix [#1656](https://github.com/biomejs/biome/issues/1656). [useOptionalChain](https://biomejs.dev/linter/rules/use-optional-chain/) code action now correctly handles logical and chains where methods with the same name are invoked with different arguments:

  ```diff
  - tags && tags.includes('a') && tags.includes('b')
  + tags?.includes('a') && tags.includes('b')
  ```

  Contributed by @lucasweng

- Fix [#1704](https://github.com/biomejs/biome/issues/1704). Convert `/` to escaped slash `\/` to avoid parsing error in
  the result of autofix. Contributed by @togami2864

- Fix[#1697](https://github.com/biomejs/biome/issues/1697). Preserve leading trivia in autofix of suppression rules.
  Contributed by @togami2864

- Fix [#603](https://github.com/biomejs/biome/issues/603). Trim trailing whitespace to avoid double insertion.
  Contributed by @togami2864

- Fix [#1765](https://github.com/biomejs/biome/issues/1765). Now the rule `noDelete` doesn't trigger when deleting a
  dataset:
  ```js
  delete element.dataset.prop;
  ```
  Contributed by @ematipico

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention)
  and [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention) now reject identifiers with consecutive delimiters.

  The following name is now invalid because it includes two underscores:

  ```js
  export const MY__CONSTANT = 0;
  ```

  Note that we still allow consecutive leading and consecutive trailing underscores.

  Contributed by @Conaclos

- Fix [#1932](https://github.com/biomejs/biome/issues/1932) Allow redeclaration of type parameters in different declarations.
  Contributed by @keita-hino

- Fix [#1945](https://github.com/biomejs/biome/issues/1945) Allow constructor with default parameters in `noUselessConstructor`

- Fix [#1982](https://github.com/biomejs/biome/issues/1982) Change to iterate over the module item lists and ignore .d.ts files. Contributed by @togami2864

### Parser

#### Bug fixes

- Fix [#1728](https://github.com/biomejs/biome/issues/1728). Correctly parse the global declaration when the `{` token
  is on the line following the `global` keyword.

  Now the following code is correctly parsed:

  ```ts
  declare global
  { }

  declare module foo {
    global
    { }
  }
  ```

  Contributed by @ah-yu

- Fix [#1730](https://github.com/biomejs/biome/issues/1730). Correctly parse `delete` expressions with operands that are
  not simple member expressions.

  ```js
  delete(a.b);
  delete console.log(1);
  delete(() => {});
  ```

  Contributed by @printfn

### Website

#### Bug fixes

- Fix [#1981](https://github.com/biomejs/biome/issues/1981). Identify TypeScript definition files by their file path within the playground. Contributed by @ah-yu

## 1.5.3 (2024-01-22)

### LSP

#### Bug fixes

- Fix [#1584](https://github.com/biomejs/biome/issues/1584). Ensure the LSP only registers the formatter once.
  Contributed by @nhedger

- Fix [#1589](https://github.com/biomejs/biome/issues/1589). Fix invalid formatting of own line comments when they were
  at the end of an import/export list. Contributed by @spanishpear

### Configuration

#### Bug fixes

- Override correctly the recommended preset ([#1349](https://github.com/biomejs/biome/issues/1349)).

  Previously, if unspecified, Biome turned on the recommended preset in overrides.
  This resulted in reporting diagnostics with a severity level set to `off`.
  This in turn caused Biome to fail.

  Now Biome won't switch on the recommended preset in `overrides` unless told to do so.

  Contributed by @Conaclos

- Don't format **ignored** files that are well-known JSONC files when `files.ignoreUnknown` is
  enabled ([#1607](https://github.com/biomejs/biome/issues/1607)).

  Previously, Biome always formatted files that are known to be JSONC files (e.g. `.eslintrc`)
  when `files.ignoreUnknown` was enabled.

  Contributed by @Conaclos

### Formatter

#### New features

- Add option `json.formatter.trailingCommas`, to provide a better control over the trailing comma in JSON/JSONC files. Its default value is `"none"`.

#### Bug fixes

- Fix [#1178](https://github.com/biomejs/biome/issues/1178), where the line ending option wasn't correctly applied.
  Contributed by @ematipico
- Fix [#1571](https://github.com/biomejs/biome/issues/1571). Fix invalid formatting of nested multiline comments.
  Contributed by @ah-yu

### Linter

#### Bug fixes

-
Fix [#1575](https://github.com/biomejs/biome/issues/1575). [noArrayIndexKey](https://biomejs.dev/linter/rules/no-array-index-key/)
now captures array index value inside template literals and with string concatination. Contributed by @vasucp1207

- Linter rules that inspect regexes now handle multibyte characters
  correctly ([#1522](https://github.com/biomejs/biome/issues/1522)).

  Previously, [noMisleadingCharacterClass](https://biomejs.dev/linter/no-misleading-character-class), [noMultipleSpacesInRegularExpressionLiterals](https://biomejs.dev/linter/no-multiple-spaces-in-regular-expression-literals),
  and [noEmptyCharacterClassInRegex](https://biomejs.dev/linter/no-empty-character-class-in-regex) made Biome errors on
  multi-bytes characters.
  Multibyte characters are now handled correctly.

  The following code no longer raises an internal error:

  ```js
  // Cyrillic characters
  /[\u200E\u2066-\u2069]/gu;
  ```

  Contributed by @Conaclos

- [useExhaustiveDependencies](https://biomejs.dev/linter/use-exhaustive-dependencies) no longer made Biome errors in
  code TypeScript import equal declarations ([#1194](https://github.com/biomejs/biome/issues/1194)). Contributed by
  @Conaclos

- Fix typo in the diagnostic of [noNodejsModules](https://biomejs.dev/linter/rules/no-nodejs-modules). Contributed by
  @huseeiin

### Parser

#### Bug fixes

- Accept the `const` modifier for type parameter in method type
  signature ([#1624](https://github.com/biomejs/biome/issues/1624)).

  The following code is now correctly parsed:

  ```ts
  type Foo = {
    <const T>();
    method<const T>();
  };
  ```

  Contributed by @magic-akari

- Correctly parse type arguments in expression([#1184](https://github.com/biomejs/biome/issues/1184)).

  The following code is now correctly parsed in typescript:

  ```ts
  0 < (0 >= 1);
  ```

  Contributed by @ah-yu

### Website

#### New

- Add a [page that maps the Biome rule to its source](https://biomejs.dev/linter/rules-sources/). Contributed by
  @ematipico

#### Fixes

- Generate Open Graph images based on the linked page. Contributed by @ematipico

- Fix examples of the [git hook page](https://biomejs.dev/recipes/git-hooks/). Contributed by @9renpoto, @lmauromb, and
  @Conaclos

- Fix dead and erroneous hyperlinks. Contributed by @Sec-ant and Conaclos

## 1.5.2 (2024-01-15)

### CLI

### Bug fixes

- Fix [#1512](https://github.com/biomejs/biome/issues/1512) by skipping verbose diagnostics from the count. Contributed
  by @ematipico

- Correctly handle cascading `include` and `ignore`.

  Previously Biome incorrectly included files that were included at tool level and ignored at global level.
  In the following example, `file.js` was formatted when it should have been ignored.
  Now, Biome correctly ignores the directory `./src/sub/`.

  ```shell
  ❯ tree src
    src
    └── sub
        └── file.js

  ❯ cat biome.json
    {
      "files": { "ignore": ["./src/sub/"] },
      "formatter": { "include": ["./src"] }
    }
  ```

  Contributed by @Conaclos

- Don't emit verbose warnings when a protected file is ignored.

  Some files, such as `package.json` and `tsconfig.json`,
  are [protected](https://biomejs.dev/guides/how-biome-works/#protected-files).
  Biome emits a verbose warning when it encounters a protected file.

  Previously, Biome emitted this verbose warning even if the file was ignored by the configuration.
  Now, it doesn't emit verbose warnings for protected files that are ignored.

  Contributed by @Conaclos

- `overrides` no longer affect which files are ignored. Contributed by @Conaclos

- The file `biome.json` can't be ignored anymore. Contributed by @ematipico

- Fix [#1541](https://github.com/biomejs/biome/issues/1541) where the content of protected files wasn't returned
  to `stdout`. Contributed by @ematipico

- Don't handle CSS files, the formatter isn't ready yet. Contributed by @ematipico

### Configuration

#### Bug fixes

- Fix [1440](https://github.com/biomejs/biome/issues/1440), a case where `extends` and `overrides` weren't correctly
  emitting the final configuration. Contributed by @arendjr

- Correctly handle `include` when `ignore` is set (#1468). Contributed by @Conaclos

  Previously, Biome ignored `include` if `ignore` was set.
  Now, Biome check both `include` and `ignore`.
  A file is processed if it is included and not ignored.
  If `include` is not set all files are considered included.

### Formatter

#### Bug fixes

- Fix placement of comments before `*` token in generator methods with
  decorators. [#1537](https://github.com/biomejs/biome/pull/1537) Contributed by @ah-yu

- Fix [#1406](https://github.com/biomejs/biome/issues/1406). Ensure comments before the `async` keyword are placed
  before it. Contributed by @ah-yu

- Fix [#1172](https://github.com/biomejs/biome/issues/1172). Fix placement of line comment after function expression
  parentheses, they are now attached to first statement in body. Contributed by @kalleep

- Fix [#1511](https://github.com/biomejs/biome/issues/1511) that made the JavaScript formatter crash. Contributed
  @Conaclos

### Linter

#### Enhancements

- Add an unsafe code fix for [noConsoleLog](https://biomejs.dev/linter/rules/no-console-log/). Contributed by
  @vasucp1207

- [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function) no longer reports function in `extends`
  clauses or in a `new` expression. Contributed by @Conaclos

  These cases require the presence of a prototype.

- Add dependency variable names on error message when useExhaustiveDependencies rule shows errors. Contributed by
  @mehm8128

#### Bug fixes

- The fix of [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function) now adds parentheses around the
  arrow function in more cases where it is needed ([#1524](https://github.com/biomejs/biome/issues/1524)).

  A function expression doesn't need parentheses in most expressions where it can appear.
  This is not the case with the arrow function.
  We previously added parentheses when the function appears in a call or member expression.
  We now add parentheses in binary-like expressions and other cases where they are needed, hopefully covering all cases.

  Previously:

  ```diff
  - f = f ?? function() {};
  + f = f ?? () => {};
  ```

  Now:

  ```diff
  - f = f ?? function() {};
  + f = f ?? (() => {});
  ```

  Contributed by @Conaclos

- Fix [#1514](https://github.com/biomejs/biome/issues/1514). Fix autofix suggestion to avoid the syntax error
  in `no_useless_fragments`. Contributed by @togami2864

## 1.5.1 (2024-01-10)

### CLI

#### Bug fixes

- The diagnostics `files/missingHandler` are now shown only when the option `--verbose` is passed. Contributed by
  @ematipico
- The diagnostics for protected files are now shown only when the option `--verbose` is passed. Contributed by
  @ematipico
- Fix [#1465](https://github.com/biomejs/biome/issues/1465), by taking in consideration the workspace folder when
  matching a pattern. Contributed by @ematipico
- Fix [#1465](https://github.com/biomejs/biome/issues/1465), by correctly process globs that contain file names.
  Contributed by @ematipico

### Formatter

#### Bug fixes

- Fix [#1170](https://github.com/biomejs/biome/issues/1170). Fix placement of comments inside default switch clause. Now
  all line comments that have a preceding node will keep their position. Contributed by @kalleep

### Linter

#### Bug fixes

-
Fix [#1335](https://github.com/biomejs/biome/issues/1335). [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/)
now ignores code action on component props when the fragment is empty. Contributed by @vasucp1207

- [useConsistentArrayType](https://biomejs.dev/linter/rules/use-consistent-array-type) was accidentally placed in
  the `style` rule group instead of the `nursery` group. It is now correctly placed under `nursery`.

-
Fix [#1483](https://github.com/biomejs/biome/issues/1483). [useConsistentArrayType](https://biomejs.dev/linter/rules/use-consistent-array-type)
now correctly handles its option. Contributed by @Conaclos

-
Fix [#1502](https://github.com/biomejs/biome/issues/1502). [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function)
now correctly handle functions that return a (comma) sequence expression. Contributed by @Conaclos

Previously the rule made an erroneous suggestion:

```diff
- f(function() { return 0, 1; }, "");
+ f(() => 0, 1, "")
```

Now, the rule wraps any comma sequence between parentheses:

```diff
- f(function() { return 0, 1; }, "");
+ f(() => (0, 1), "")
```

-
Fix [#1473](https://github.com/biomejs/biome/issues/1473): [useHookAtTopLevel](https://biomejs.dev/linter/rules/use-hook-at-top-level/)
now correctly handles React components and hooks that are nested inside other functions. Contributed by @arendjr

## 1.5.0 (2024-01-08)

Biome now scores 97% compatibility with Prettier and features more than 180 linter rules.

### Analyzer

### CLI

#### New features

- Biome now shows a diagnostic when it encounters a protected file. Contributed by @ematipico

- The command `biome migrate` now updates the `$schema` if there's an outdated version.

- The CLI now takes in consideration the `.gitignore` in the home directory of the user, if it exists. Contributed by
  @ematipico
- The `biome ci` command is now able to
  print [GitHub Workflow Commands](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions)
  when there are diagnostics in our code. Contributed by @nikeee
  This **might** require setting the proper permissions on your GitHub action:
  ```yaml
  permissions:
    pull-requests: write
  ```
- The commands `format`, `lint`, `check` and `ci` now accept two new arguments: `--changed` and `--since`. Use these
  options with the VCS integration
  is enabled to process only the files that were changed. Contributed by @simonxabris

  ```shell
  biome format --write --changed
  ```

- Introduced a new command called `biome explain`, which has the capability to display documentation for lint rules.
  Contributed by @kalleep
- You can use the command `biome explain` to print the documentation of lint rules. Contributed by @kalleep
  ```shell
  biome explain noDebugger
  biome explain useAltText
  ```
- You can use the command `biome explain` to print the directory where daemon logs are stored. Contributed by @ematipico
  ```shell
  biome explain daemon-logs
  ```
- Removed the hard coded limit of 200 printable diagnostics. Contributed by @ematipico

#### Bug fixes

- Fix [#1247](https://github.com/biomejs/biome/issues/1247), Biome now prints a **warning** diagnostic if it encounters
  files that can't handle. Contributed by @ematipico

  You can ignore unknown file types using
  the [`files.ignoreUnknown`](https://biomejs.dev/reference/configuration/#filesignoreunknown) configuration
  in `biome.json`:

  ```jsonc
  {
    "files": {
      "ignoreUnknown": true
    }
  }
  ```

  Or the `--files-ignore-unknown` CLI option:

  ```shell
  biome format --files-ignore-unknown=true --write .
  ```

- Fix [#709](https://github.com/biomejs/biome/issues/709) and [#805](https://github.com/biomejs/biome/issues/805) by
  correctly parsing `.gitignore` files. Contributed by @ematipico

- Fix [#1117](https://github.com/biomejs/biome/issues/1117) by correctly respecting the matching. Contributed by
  @ematipico

- Fix [#691](https://github.com/biomejs/biome/issues/691) and [#1190](https://github.com/biomejs/biome/issues/1190), by
  correctly apply the configuration when
  computing [`overrides` configuration](https://biomejs.dev/reference/configuration/#overrides). Contributed by
  @ematipico

### Configuration

#### New features

- Users can specify _git ignore patterns_ inside `ignore` and `include` properties, for example it's possible to **allow
  list** globs of files using the `!` character:

  ```jsonc
  {
    "files": {
      "ignore": [
        "node_modules/**",
        "!**/dist/**" // this is now accepted and allow files inside the `dist` folder
      ]
    }
  }
  ```

### Editors

#### New features

- The LSP registers formatting without the need of using dynamic capabilities from the client.

  This brings formatting services to the editors that don't support or have limited support for dynamic capabilities.

### Formatter

#### Bug fixes

- Fix [#1169](https://github.com/biomejs/biome/issues/1169). Account for escaped strings when computing layout for
  assignments. Contributed by @kalleep
- Fix [#851](https://github.com/biomejs/biome/issues/851). Allow regular function expressions to group and break as call
  arguments, just like arrow function expressions. [#1003](https://github.com/biomejs/biome/issues/1003) Contributed by
  @faultyserver
- Fix [#914](https://github.com/biomejs/biome/issues/914). Only parenthesize type-casted function expressions as default
  exports. [#1023](https://github.com/biomejs/biome/issues/1023) Contributed by @faultyserver
- Fix [#1112](https://github.com/biomejs/biome/issues/1112). Break block bodies in case clauses onto their own lines and
  preserve trailing fallthrough comments. [#1035](https://github.com/biomejs/biome/pull/1035) Contributed by
  @faultyserver
- Fix `RemoveSoftLinesBuffer` behavior to also removed conditional expanded content, ensuring no accidental, unused line
  breaks are included [#1032](https://github.com/biomejs/biome/pull/1032) Contributed by @faultyserver
- Fix [#1024](https://github.com/biomejs/biome/issues/1024). Allow JSX expressions to nestle in arrow
  chains [#1033](https://github.com/biomejs/biome/pull/1033) Contributed by @faultyserver
- Fix incorrect breaking on the left side of assignments by always using fluid
  assignment. [#1021](https://github.com/biomejs/biome/pull/1021) Contributed by @faultyserver
- Fix breaking strategy for nested object patterns in function
  parameters [#1054](https://github.com/biomejs/biome/pull/1054) Contributed by @faultyserver
- Fix over-indention of arrow chain expressions by simplifying the way each chain is
  grouped [#1036](https://github.com/biomejs/biome/pull/1036), [#1136](https://github.com/biomejs/biome/pull/1136),
  and [#1162](https://github.com/biomejs/biome/pull/1162) Contributed by @faultyserver.
- Fix "simple" checks for calls and member expressions to correctly handle array accesses, complex arguments to
  single-argument function calls, and multiple-argument function
  calls. [#1057](https://github.com/biomejs/biome/pull/1057) Contributed by @faultyserver
- Fix text wrapping and empty line handling for JSX Text elements to match Prettier's
  behavior. [#1075](https://github.com/biomejs/biome/pull/1075) Contributed by @faultyserver
- Fix leading comments in concisely-printed arrays to prevent unwanted line
  breaks. [#1135](https://github.com/biomejs/biome/pull/1135) Contributed by @faultyserver
- Fix `best_fitting` and interned elements preventing expansion propagation from sibling
  elements. [#1141](https://github.com/biomejs/biome/pull/1141) Contributed by @faultyserver
- Fix heuristic for grouping function parameters when type parameters with constraints are
  present. [#1153](https://github.com/biomejs/biome/pull/1153). Contributed by @faultyserver.
- Fix binary-ish and type annotation handling for grouping call arguments in function expressions and call
  signatures. [#1152](https://github.com/biomejs/biome/pull/1152)
  and [#1160](https://github.com/biomejs/biome/pull/1160) Contributed by @faultyserver
- Fix handling of nestled JSDoc comments to preserve behavior for
  overloads. [#1195](https://github.com/biomejs/biome/pull/1195) Contributed by @faultyserver
- Fix [#1208](https://github.com/biomejs/biome/issues/1208). Fix extraction of inner types when checking for simple type
  annotations in call arguments. [#1195](https://github.com/biomejs/biome/pull/1195) Contributed by @faultyserver

- Fix [#1220](https://github.com/biomejs/biome/issues/1220). Avoid duplicating comments in type unions for mapped, empty
  object, and empty tuple types. [#1240](https://github.com/biomejs/biome/pull/1240) Contributed by @faultyserver

- Fix [#1356](https://github.com/biomejs/biome/issues/1356). Ensure `if_group_fits_on_line` content is always written
  in `RemoveSoftLinesBuffer`s. [#1357](https://github.com/biomejs/biome/pull/1357) Contributed by @faultyserver

- Fix [#1171](https://github.com/biomejs/biome/issues/1171). Correctly format empty statement with comment inside arrow
  body when used as single argument in call expression. Contributed by @kalleep

- Fix [#1106](https://github.com/biomejs/biome/issues/1106). Fix invalid formatting of single bindings when Arrow
  Parentheses is set to "AsNeeded" and the expression breaks over multiple
  lines. [#1449](https://github.com/biomejs/biome/pull/1449) Contributed by @faultyserver

### JavaScript APIs

### Linter

#### Promoted rules

New rules are incubated in the nursery group.
Once stable, we promote them to a stable group.
The following rules are promoted:

- [a11y/noAriaHiddenOnFocusable](https://www.biomejs.dev/linter/rules/no-aria-hidden-on-focusable)
- [a11y/useValidAriaRole](https://www.biomejs.dev/linter/rules/use-valid-aria-role)
- [complexity/useRegexLiterals](https://www.biomejs.dev/linter/rules/use-regex-literals)
- [suspicious/noImplicitAnyLet](https://www.biomejs.dev/linter/rules/no-implicit-any-let)
- [style/noDefaultExport](https://www.biomejs.dev/linter/rules/no-default-export)

#### New features

- Add [useExportType](https://biomejs.dev/linter/rules/use-export-type) that enforces the use of type-only exports for
  types. Contributed by @Conaclos

  ```diff
    interface A {}
    interface B {}
    class C {}

  - export type { A, C }
  + export { type A, C }

  - export { type B }
  + export type { B }
  ```

- Add [useImportType](https://biomejs.dev/linter/rules/use-import-type) that enforces the use of type-only imports for
  types. Contributed by @Conaclos

  ```diff
  - import { A, B } from "./mod.js";
  + import { type A, B } from "mod";
    let a: A;
    const b: B = new B();
  ```

  Also, the rule groups type-only imports:

  ```diff
  - import { type A, type B } from "./mod.js";
  + import type { A, B } from "./mod.js";
  ```

- Add [useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention), that enforces naming
  conventions for JavaScript and TypeScript filenames. Contributed by @Conaclos

  By default, the rule requires that a filename be in `camelCase`, `kebab-case`, `snake_case`, or matches the name of
  an `export` in the file.
  The rule provides options to restrict the allowed cases.

- Add [useNodejsImportProtocol](https://biomejs.dev/linter/rules/use-nodejs-import-protocol) that enforces the use of
  the `node:` protocol when importing _Node.js_ modules. Contributed by @2-NOW, @vasucp1207, and @Conaclos

  ```diff
  - import fs from "fs";
  + import fs from "node:fs";
  ```

- Add [useNumberNamespace](https://biomejs.dev/linter/rules/use-number-namespace) that enforces the use of the `Number`
  properties instead of the global ones.

  ```diff
  - parseInt;
  + Number.parseInt;
  - - Infinity;
  + Number.NEGATIVE_INFINITY;
  ```

- Add [useShorthandFunctionType](https://biomejs.dev/linter/rules/use-shorthand-function-type) that enforces using
  function types instead of object type with call signatures. Contributed by @emab, @ImBIOS, and @seitarof

  ```diff
  - interface Example {
  -   (): string;
  - }
  + type Example = () => string

```

- Add [noNodejsModules](https://biomejs.dev/linter/rules/no-nodejs-modules), that disallows the use of _Node.js_ modules. Contributed by @anonrig, @ematipico, and @Conaclos

- Add [noInvalidUseBeforeDeclaration](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration) that reports variables and function parameters used before their declaration. Contributed by @Conaclos

  ```js
  function f() {
    console.log(c); // Use of `c` before its declaration.
    const c = 0;
  }
  ```

- Add [useConsistentArrayType](https://biomejs.dev/linter/rules/use-consistent-array-type) that enforces the use of a
  consistent syntax for array types. Contributed by @eryue0220

  This rule will replace [useShorthandArrayType](https://biomejs.dev/linter/rules/use-shorthand-array-type).
  It provides an option to choose between the shorthand or the generic syntax.

- Add [noEmptyTypeParameters](https://biomejs.dev/linter/rules/no-empty-type-parameters) that ensures that any type
  parameter list has at least one type parameter. Contributed by @togami2864

  This will report the following empty type parameter lists:

  ```ts
  interface Foo<> {}
  //           ^^
  type Bar<> = {};
  //      ^^
  ```

- Add [noGlobalEval](https://biomejs.dev/linter/rules/no-global-eval) that reports any use of the global `eval`.
  Contributed by @you-5805

- Add [noGlobalAssign](https://biomejs.dev/linter/rules/no-global-assign) that reports assignment to global variables.
  Contributed by @chansuke

  ```js
  Object = {}; // report assignment to `Object`.
  ```

- Add [noMisleadingCharacterClass](https://biomejs.dev/linter/rules/no-misleading-character-class) that disallows
  characters made with multiple code points in character class. Contributed by @togami2864

- Add [noThenProperty](https://biomejs.dev/linter/rules/no-then-property) that disallows the use of `then` as property
  name. Adding a `then` property makes an object _thenable_ that can lead to errors with Promises. Contributed by
  @togami2864

- Add [noUselessTernary](https://biomejs.dev/linter/rules/no-useless-ternary) that disallows conditional expressions (
  ternaries) when simpler alternatives exist.

  ```js
  var a = x ? true : true; // this could be simplified to `x`
  ```

#### Enhancements

- [noEmptyInterface](https://biomejs.dev/linter/rules/no-empty-interface) ignores empty interfaces that extend a type.
  Address [#959](https://github.com/biomejs/biome/issues/959) and [#1157](https://github.com/biomejs/biome/issues/1157).
  Contributed by @Conaclos

  This allows supporting interface augmentation in external modules as demonstrated in the following example:

  ```ts
  interface Extension {
    metadata: unknown;
  }

  declare module "@external/module" {
    // Empty interface that extends a type.
    export interface ExistingInterface extends Extension {}
  }
  ```

- Preserve more comments in the code fix
  of [useExponentiationOperator](https://biomejs.dev/linter/rules/use-exponentiation-operator). Contributed by @Conaclos

  The rule now preserves comments that follow the (optional) trailing comma.

  For example, the rule now suggests the following code fix:

  ```diff
  - Math.pow(
  -    a, // a
  -    2, // 2
  -  );
  +
  +    a ** // a
  +    2 // 2
  +
  ```

- `<svg>` element is now considered as a non-interactive HTML
  element ([#1095](https://github.com/biomejs/biome/issues/1095)). Contributed by @chansuke

  This affects the following rules:
  - [noAriaHiddenOnFocusable](https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable)
  - [noInteractiveElementToNoninteractiveRole](https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role)
  - [noNoninteractiveElementToInteractiveRole](https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role)
  - [noNoninteractiveTabindex](https://biomejs.dev/linter/rules/no-noninteractive-tabindex)
  - [useAriaActivedescendantWithTabindex](https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex)

- [noMultipleSpacesInRegularExpressionLiterals](https://biomejs.dev/linter/rules/no-multiple-spaces-in-regular-expression-literals/)
  has a safe code fix. Contributed by @Conaclos

- [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/) ignores expressions that use `new.target`.
  Contributed by @Conaclos

- [noForEach](https://biomejs.dev/linter/rules/no-for-each) now reports only calls that use a callback with `0` or `1`
  parameter. Address [#547](https://github.com/biomejs/biome/issues/547). Contributed by @Conaclos

#### Bug fixes

-
Fix [#1061](https://github.com/biomejs/biome/issues/1061). [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare)
no longer reports overloads of `export default function`. Contributed by @Conaclos

The following code is no longer reported:

```ts
export default function(a: boolean): boolean;
export default function(a: number): number;
export default function(a: number | boolean): number | boolean {
  return a;
}
```

-
Fix [#651](https://github.com/biomejs/biome/issues/651), [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies)
no longer reports out of scope dependencies. Contributed by @kalleep

The following code is no longer reported:
```ts
let outer = false;

const Component = ({}) => {
  useEffect(() => {
    outer = true;
  }, []);
}
```

-
Fix [#1191](https://github.com/biomejs/biome/issues/1191). [noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
now preserve comments of the `else` clause. Contributed by @Conaclos

For example, the rule suggested the following fix:

```diff
  function f(x) {
    if (x <0) {
      return 0;
    }
-   // Comment
-   else {
      return x;
-   }
  }
```

Now the rule suggests a fix that preserves the comment of the `else` clause:

```diff
  function f(x) {
    if (x <0) {
      return 0;
    }
    // Comment
-   else {
      return x;
-   }
  }
```

-
Fix [#1383](https://github.com/biomejs/biome/issues/1383). [noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type)
now accepts the `void` type in type parameter lists.

The rule no longer reports the following code:

```ts
f<void>();
```

-
Fix [#728](https://github.com/biomejs/biome/issues/728). [useSingleVarDeclarator](https://biomejs.dev/linter/rules/use-single-var-declarator)
no longer outputs invalid code. Contributed by @Conaclos

-
Fix [#1167](https://github.com/biomejs/biome/issues/1167). [useValidAriaProps](https://biomejs.dev/linter/rules/use-valid-aria-props)
no longer reports `aria-atomic` as invalid. Contributed by @unvalley

-
Fix [#1192](https://github.com/biomejs/biome/issues/1192). [useTemplate](https://biomejs.dev/linter/rules/use-template/)
now correctly handles parenthesized expressions and respects type coercions. Contributed by @n-gude

These cases are now properly handled:

```js
"a" + (1 + 2) // `a${1 + 2}`
```

```js
1 + (2 + "a") // `${1}${2}a`
```

-
Fix [#1456](https://github.com/biomejs/biome/issues/1456). [useTemplate](https://biomejs.dev/linter/rules/use-template/)
now reports expressions with an interpolated template literal and non-string expressions. Contributed by @n-gude

The following code is now reported:

```js
`a${1}` + 2;
```

-
Fix [#1436](https://github.com/biomejs/biome/issues/1436). [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/)
now applies a correct fix when a function expression is used in a call expression or a member access. Contributed by
@Conaclos

For example, the rule proposed the following fix:

```diff
- const called = function() {}();
+ const called = () => {}();
```

It now proposes a fix that adds the needed parentheses:

```diff
- const called = function() {}();
+ const called = (() => {})();
```

-
Fix [#696](https://github.com/biomejs/biome/issues/696). [useHookAtTopLevel](https://biomejs.dev/linter/rules/use-hook-at-top-level)
now correctly detects early returns before the calls to the hook.

- The code fix of [noUselessTypeCOnstraint](https://biomejs.dev/linter/rules/no-useless-type-constraint) now adds a
  trailing comma when needed to disambiguate a type parameter list from a JSX element. COntributed by @Conaclos

-
Fix [#578](https://github.com/biomejs/biome/issues/578). [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies)
now correctly recognizes hooks namespaced under the `React` namespace. Contributed by @XiNiHa

-
Fix [#910](https://github.com/biomejs/biome/issues/910). [noSvgWithoutTitle](https://biomejs.dev/linter/rules/no-svg-without-title)
now ignores `<svg>` element with `aria-hidden="true"`. COntributed by @vasucp1207

### Parser

#### BREAKING CHANGES

- The representation of imports has been simplified. Contributed by @Conaclos

  The new representation is closer to the ECMAScript standard.
  It provides a single way of representing a namespace import such as `import * as ns from ""`.
  It rules out some invalid states that was previously representable.
  For example, it is no longer possible to represent a combined import with a `type` qualifier such
  as `import type D, { N } from ""`.

  See [#1163](https://github.com/biomejs/biome/pull/1163) for more details.

#### New features

- Imports and exports with both an _import attribute_ and a `type` qualifier are now reported as parse errors.

  ```ts
  import type A from "mod" with { type: "json" };
  //     ^^^^              ^^^^^^^^^^^^^^^^^^^^^
  //     parse error
  ```

#### Bug fixes

- Fix [#1077](https://github.com/biomejs/biome/issues/1077) where parenthesized identifiers in conditional expression
  were being parsed as arrow expressions. Contributed by @kalleep

  These cases are now properly parsed:

  _JavaScript_:

  ```javascript
    a ? (b) : a => {};
  ```

  _TypeScript_:

  ```ts
    a ? (b) : a => {};
  ```

  _JSX_:

  ```jsx
    bar ? (foo) : (<a>{() => {}}</a>);
  ```

- Allow empty type parameter lists for interfaces and type
  aliases ([#1237](https://github.com/biomejs/biome/issues/1237)). COntributed by @togami2864

  _TypeScript_ allows interface declarations and type aliases to have empty type parameter lists.
  Previously Biome didn't handle this edge case.
  Now, it correctly parses this syntax:

  ```ts
  interface Foo<> {}
  type Bar<> = {};
  ```

### Crates

#### BREAKING CHANGES

- Rename the `biome_js_unicode_table` crate
  to `biome_unicode_table` ([#1302](https://github.com/biomejs/biome/issues/1302)). COntributed by @chansuke

## 1.4.1 (2023-11-30)

### Editors

#### Bug fixes

- Fix [#933](https://github.com/biomejs/biome/issues/933). Some files are properly ignored in the LSP too.
  E.g. `package.json`, `tsconfig.json`, etc.
- Fix [#1394](https://github.com/biomejs/biome/issues/1394), by inferring the language extension from the internal saved
  files. Now newly created files JavaScript correctly show diagnostics.

### Formatter

#### Bug fixes

- Fix some accidental line breaks when printing array expressions within arrow functions and other long
  lines [#917](https://github.com/biomejs/biome/pull/917). Contributed by @faultyserver

- Match Prettier's breaking strategy for `ArrowChain` layouts [#934](https://github.com/biomejs/biome/pull/934).
  Contributed by @faultyserver

- Fix double-printing of leading comments in arrow chain expressions [#951](https://github.com/biomejs/biome/pull/951).
  Contributed by @faultyserver

### Linter

#### Bug fixes

- Fix [#910](https://github.com/biomejs/biome/issues/910), where the rule `noSvgWithoutTitle` should skip elements that
  have `aria-hidden` attributes. Contributed by @vasucp1207

#### New features

- Add [useForOf](https://biomejs.dev/linter/rules/use-for-of) rule.
  The rule recommends a for-of loop when the loop index is only used to read from an array that is being iterated.
  Contributed by @victor-teles

#### Enhancement

- Address [#924](https://github.com/biomejs/biome/issues/924)
  and [#920](https://github.com/biomejs/biome/issues/920). [noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
  now ignores `else` clauses that follow at least one `if` statement that doesn't break early. Contributed by @Conaclos

  For example, the following code is no longer reported by the rule:

  ```js
  function f(x) {
      if (x < 0) {
        // this `if` doesn't break early.
      } else if (x > 0) {
          return x;
      } else {
          // This `else` block was previously reported as useless.
      }
  }
  ```

#### Bug fixes

-
Fix [#918](https://github.com/biomejs/biome/issues/918). [useSimpleNumberKeys](https://biomejs.dev/linter/rules/use-simple-number-keys)
no longer repports false positive on comments. Contributed by @kalleep

- Fix [#953](https://github.com/biomejs/biome/issues/953). [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare)
  no longer reports type parameters with the same name in different mapped types as redeclarations. Contributed by
  @Conaclos

-
Fix [#608](https://github.com/biomejs/biome/issues/608). [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies)
no longer repports missing dependencies for React hooks without dependency array. Contributed by @kalleep

### Parser

## 1.4.0 (2023-11-27)

### CLI

- Remove the CLI options from the `lsp-proxy`, as they were never meant to be passed to that command. Contributed by
  @ematipico

- Add option `--config-path` to `lsp-proxy` and `start` commands. It's now possible to tell the Daemon server to
  load `biome.json` from a custom path. Contributed by @ematipico

- Add option `--diagnostic-level`. It lets users control the level of diagnostics printed by the CLI. Possible values
  are: `"info"`, `"warn"`, and `"hint"`. Contributed by @simonxabris

- Add option `--line-feed` to the `format` command. Contributed by @SuperchupuDev

- Add option `--bracket-same-line` to the `format` command. Contributed by @faultyserver

- Add option `--bracket-spacing` to the `format` command. Contributed by @faultyserver

#### Bug fixes

- Fix the command `format`, now it returns a non-zero exit code when if there pending diffs. Contributed by @ematipico

### Formatter

#### New features

- Add the configuration [`formatter.lineFeed`](https://biomejs.dev/reference/configuration/#formatterlineending). It
  allows changing the type of line endings. Contributed by @SuperchupuDev

- Add the
  configuration [`javascript.formatter.bracketSameLine`](https://biomejs.dev/reference/configuration/#formatterbracketsameline).
  It allows controlling whether ending `>` of a multi-line _JSX_ element should be on the last attribute line or
  not. [#627](https://github.com/biomejs/biome/issues/627). Contributed by @faultyserver

- Add the
  configuration [`javascript.formatter.bracketSpacing`](https://biomejs.dev/reference/configuration/#formatterbracketspacing).
  It allows controlling whether spaces are inserted around the brackets of object
  literals. [#627](https://github.com/biomejs/biome/issues/627). Contributed by @faultyserver

#### Bug fixes

- Fix [#832](https://github.com/biomejs/biome/issues/832), the formatter no longer keeps an unnecessary trailing comma
  in type parameter lists. Contributed by @Conaclos

  ```diff
  - class A<T,> {}
  + class A<T> {}
  ```

- Fix [#301](https://github.com/biomejs/biome/issues/301), the formatter should not break before the `in` keyword.
  Contributed by @ematipico

### Linter

#### Promoted rules

- [a11y/noInteractiveElementToNoninteractiveRole](https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role)
- [complexity/noThisInStatic](https://biomejs.dev/linter/rules/no-this-in-static)
- [complexity/useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function)
- [correctness/noEmptyCharacterClassInRegex](https://biomejs.dev/linter/rules/no-empty-character-class-in-regex)
- [correctness/noInvalidNewBuiltin](https://biomejs.dev/linter/rules/no-invalid-new-builtin)
- [style/noUselessElse](https://biomejs.dev/linter/rules/no-useless-else)
- [style/useAsConstAssertion](https://biomejs.dev/linter/rules/use-as-const-assertion)
- [style/useShorthandAssign](https://biomejs.dev/linter/rules/use-shorthand-assign)
- [suspicious/noApproximativeNumericConstant](https://biomejs.dev/linter/rules/no-approximative-numeric-constant)
- [suspicious/noMisleadingInstantiator](https://biomejs.dev/linter/rules/no-misleading-instantiator)
- [suspicious/noMisrefactoredShorthandAssign](https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign)

The following rules are now recommended:

- [a11y/noAccessKey](https://biomejs.dev/linter/rules/no-access-key)
- [a11y/useHeadingContent](https://biomejs.dev/linter/rules/use-heading-content)
- [complexity/useSimpleNumberKeys](https://biomejs.dev/linter/rules/use-simple-number-keys)

The following rules are now deprecated:

- [correctness/noNewSymbol](https://biomejs.dev/linter/rules/no-new-symbol)
  The rule is replaced by [correctness/noInvalidNewBuiltin](https://biomejs.dev/linter/rules/no-invalid-new-builtin)

#### New features

- Add [noDefaultExport](https://biomejs.dev/linter/rules/no-default-export) which disallows `export default`.
  Contributed by @Conaclos

- Add [noAriaHiddenOnFocusable](https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable) which reports hidden and
  focusable elements. Contributed by @vasucp1207

- Add [noImplicitAnyLet](https://biomejs.dev/linter/rules/no-implicit-any-let) that reports variables declared
  with `let` and without initialization and type annotation. Contributed by @TaKO8Ki and @b4s36t4

- Add [useAwait](https://biomejs.dev/linter/rules/use-await) that reports `async` functions that don't use an `await`
  expression.

- Add [useValidAriaRole](https://biomejs.dev/linter/rules/use-valid-aria-role). Contributed by @vasucp1207

- Add [useRegexLiterals](https://biomejs.dev/linter/rules/use-regex-literals) that suggests turning call to the regex
  constructor into regex literals. COntributed by @Yuiki

#### Enhancements

- Add an unsafe code fix
  for [a11y/useAriaActivedescendantWithTabindex](https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex)

#### Bug fixes

- Fix [#639](https://github.com/biomejs/biome/issues/639) by ignoring unused TypeScript's mapped key. Contributed by
  @Conaclos

- Fix [#565](https://github.com/biomejs/biome/issues/565) by handling several `infer` with the same name in extends
  clauses of TypeScript's conditional types. Contributed by @Conaclos

-
Fix [#653](https://github.com/biomejs/biome/issues/653). [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports)
now correctly removes the entire line where the unused `import` is. Contributed by @Conaclos

- Fix [#607](https://github.com/biomejs/biome/issues/609) `useExhaustiveDependencies`, ignore optional chaining,
  Contributed by @msdlisper

- Fix [#676](https://github.com/biomejs/biome/issues/676), by using the correct node for the `"noreferrer"` when
  applying the code action. Contributed by @ematipico

- Fix [#455](https://github.com/biomejs/biome/issues/455). The CLI can now print complex emojis to the console
  correctly.

-
Fix [#727](https://github.com/biomejs/biome/issues/727). [noInferrableTypes](https://biomejs.dev/linter/rules/no-inferrable-types)
now correctly keeps type annotations when the initialization expression is `null`. Contributed by @Conaclos

-
Fix [#784](https://github.com/biomejs/biome/issues/784), [noSvgWithoutTitle](https://biomejs.dev/linter/rules/no-svg-without-title)
fixes false-positives to `aria-label` and reports svg's role attribute is implicit. Contributed by @unvalley

- Fix [#834](https://github.com/biomejs/biome/issues/834) that
  made [noUselessLoneBlockStatements](https://biomejs.dev/linter/rules/no-useless-lone-block-statements) reports block
  statements of switch clauses. Contributed by @vasucp1207

- Fix [#783](https://github.com/biomejs/biome/issues/834) that
  made [noUselessLoneBlockStatements](https://biomejs.dev/linter/rules/no-useless-lone-block-statements) reports block
  statements of `try-catch` structures. Contributed by @hougesen

- Fix [#69](https://github.com/biomejs/biome/issues/69) that
  made [correctness/noUnnecessaryContinue](https://biomejs.dev/linter/rules/no-unnecessary-continue) incorrectly reports
  a `continue` used to break a switch clause. Contributed by @TaKO8Ki

- Fix [#664](https://github.com/biomejs/biome/issues/664) by improving the diagnostic
  of [style/useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention) when double capital are
  detected in strict camel case mode. Contributed by @vasucp1207

- Fix [#643](https://github.com/biomejs/biome/issues/643) that erroneously parsed the option
  of [complexity/useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-naming-convention). Contributed by
  @arendjr

### Parser

#### Bug fixes

- Fix [#846](https://github.com/biomejs/biome/issues/846) that erroneously parsed `<const T,>() => {}` as a JSX tag
  instead of an arrow function when both TypeScript and JSX are enabled.

### VSCode

## 1.3.3 (2023-10-31)

### Analyzer

#### Bug fixes

- Fix [#604](https://github.com/biomejs/biome/issues/604) which
  made [noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type) report false positives when
  the `void` type is used in a generic type parameter. Contributed by @unvalley

### CLI

#### Bug fixes

- Fix how `overrides` behave. Now `ignore` and `include` apply or not the override pattern, so they override each other.
  Now the options inside `overrides` override the top-level options.
- Bootstrap the logger only when needed. Contributed by @ematipico
- Fix how `overrides` are run. The properties `ignore` and `include` have different semantics and only apply/not apply
  an override. Contributed by @ematipico

### Editors

#### Bug fixes

- Fix [#592](https://github.com/biomejs/biome/issues/592), by changing binary resolution in the IntelliJ plugin.
  Contributed by @Joshuabaker2

### Formatter

#### Bug fixes

- Apply the correct layout when the right hand of an assignment expression is an `await` expression or a yield
  expression. Contributed by @ematipico

- Fix [#303](https://github.com/biomejs/biome/issues/303), where nested arrow functions didn't break. Contributed by
  @victor-teles

### Linter

#### New features

- Add [noUnusedPrivateClassMembers](https://biomejs.dev/linter/rules/no-unused-private-class-members) rule. The rule
  disallow unused private class members. Contributed by @victor-teles

#### Bug fixes

- Fix [#175](https://github.com/biomejs/biome/issues/175) which
  made [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare) report index signatures using the name of a variable
  in the parent scope.

- Fix [#557](https://github.com/biomejs/biome/issues/557) which
  made [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports) report imported types used in `typeof`
  expression. Contributed by @Conaclos

- Fix [#576](https://github.com/biomejs/biome/issues/576) by removing some erroneous logic
  in [noSelfAssign](https://biomejs.dev/linter/rules/no-self-assign/). Contributed by @ematipico

- Fix [#861](https://github.com/biomejs/biome/issues/861) that
  made [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) always reports the parameter of a
  non-parenthesize arrow function as unused.

- Fix [#595](https://github.com/biomejs/biome/issues/595) by updating unsafe-apply logic to avoid unexpected errors
  in [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/). Contributed by @nissy-dev

- Fix [#591](https://github.com/biomejs/biome/issues/591) which
  made [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare) report type parameters with identical names but in
  different method signatures. Contributed by @Conaclos

- Support more a11y roles and fix some methods for a11y lint rules Contributed @nissy-dev

- Fix [#609](https://github.com/biomejs/biome/issues/609) `useExhaustiveDependencies`, by removing `useContext`, `useId`
  and `useSyncExternalStore` from the known hooks. Contributed by @msdlisper

- Fix `useExhaustiveDependencies`, by removing `useContext`, `useId` and `useSyncExternalStore` from the known hooks.
  Contributed by @msdlisper

- Fix [#871](https://github.com/biomejs/biome/issues/871) and [#610](https://github.com/biomejs/biome/issues/610).
  Now `useHookAtTopLevel` correctly handles nested functions. Contributed by @arendjr

- The options of the rule `useHookAtTopLevel` are deprecated and will be removed in Biome 2.0. The rule now determines
  the hooks using the naming convention set by React.

  ```diff
  {
    "linter": {
      "rules": {
        "correctness": {
  +        "useHookAtTopLevel": "error",
  -        "useHookAtTopLevel": {
  -          "level": "error",
  -          "options": {
  -            "hooks": [
  -              {
  -                "name": "useLocation",
  -                "closureIndex": 0,
  -                "dependenciesIndex": 1
  -              },
  -              { "name": "useQuery", "closureIndex": 1, "dependenciesIndex": 0 }
  -            ]
  -          }
  -        }
        }
      }
    }
  }
  ```

### Parser

#### Enhancements

- Support RegExp v flag. Contributed by @nissy-dev
- Improve error messages. Contributed by @ematipico

## 1.3.1 (2023-10-20)

### CLI

#### Bug fixes

- Fix `rage` command, now it doesn't print info about running servers. Contributed by @ematipico

### Editors

#### Bug fixes

- Fix [#552](https://github.com/biomejs/biome/issues/552), where the formatter isn't correctly triggered in Windows
  systems. Contributed by @victor-teles

### Linter

#### New features

- Add [noThisInStatic](https://biomejs.dev/linter/rules/no-this-in-static) rule. Contributed by @ditorodev and @Conaclos

#### Bug fixes

- Fix [#548](https://github.com/biomejs/biome/issues/548) which
  made [noSelfAssign](https://biomejs.dev/linter/rules/no-self-assign) panic.

- Fix [#555](https://github.com/biomejs/biome/issues/555), by correctly map `globals` into the workspace.

## 1.3.0 (2023-10-19)

### Analyzer

#### Enhancements

- Import sorting is safe to apply now, and it will be applied when running `check --apply` instead
  of `check --apply-unsafe`.

- Import sorting now handles Bun imports `bun:<name>`, absolute path imports `/<path>`,
  and [Node's subpath imports `#<name>`](https://nodejs.org/api/packages.html#subpath-imports).
  See [our documentation](https://biomejs.dev/analyzer/) for more details. Contributed by @Conaclos

### CLI

#### Bug fixes

- Fix [#319](https://github.com/biomejs/biome/issues/319). The command `biome lint` now shows the correct options.
  Contributed by @ematipico
- Fix [#312](https://github.com/biomejs/biome/issues/312). Running `biome --version` now exits with status code `0`
  instead of `1`. Contributed by @nhedger
- Fix a bug where the `extends` functionality doesn't carry over `organizeImports.ignore`. Contributed by @ematipico
- The CLI now returns the original content when using `stdin` and the original content doesn't change. Contributed by
  @ematipico

#### New features

- Add support for `BIOME_BINARY` environment variable to override the location of the binary. Contributed by @ematipico
- Add option `--indent-width`, and deprecated the option `--indent-size`. Contributed by @ematipico
- Add option `--javascript-formatter-indent-width`, and deprecated the option `--javascript-formatter-indent-size`.
  Contributed by @ematipico
- Add option `--json-formatter-indent-width`, and deprecated the option `--json-formatter-indent-size`. Contributed by
  @ematipico
- Add option `--daemon-logs` to `biome rage`. The option is required to view Biome daemon server logs. Contributed by
  @unvalley
- Add support for logging. By default, Biome doesn't log anything other than diagnostics. Logging can be enabled with
  the new option `--log-level`:

  ```shell
  biome format --log-level=info ./src
  ```
  There are four different levels of logging, from the most verbose to the least verbose: `debug`, `info`, `warn`
  and `error`. Here's how an `INFO` log will look like:

  ```
  2023-10-05T08:27:01.954727Z  INFO  Analyze file ./website/src/playground/components/Resizable.tsx
    at crates/biome_service/src/file_handlers/javascript.rs:298 on biome::worker_5
    in Pulling diagnostics with categories: RuleCategories(SYNTAX)
    in Processes formatting with path: "./website/src/playground/components/Resizable.tsx"
    in Process check with path: "./website/src/playground/components/Resizable.tsx"
  ```

  You can customize how the log will look like with a new option `--log-kind`. The supported kinds
  are: `pretty`, `compact` and `json`.

  `pretty` is the default logging. Here's how a `compact` log will look like:

  ```
  2023-10-05T08:29:04.864247Z  INFO biome::worker_2 Process check:Processes linting:Pulling diagnostics: crates/biome_service/src/file_handlers/javascript.rs: Analyze file ./website/src/playground/components/Resizable.tsx path="./website/src/playground/components/Resizable.tsx" path="./website/src/playground/components/Resizable.tsx" categories=RuleCategories(LINT)
  2023-10-05T08:29:04.864290Z  INFO biome::worker_7 Process check:Processes formatting: crates/biome_service/src/file_handlers/javascript.rs: Format file ./website/src/playground/components/Tabs.tsx path="./website/src/playground/components/Tabs.tsx" path="./website/src/playground/components/Tabs.tsx"
  2023-10-05T08:29:04.879332Z  INFO biome::worker_2 Process check:Processes formatting:Pulling diagnostics: crates/biome_service/src/file_handlers/javascript.rs: Analyze file ./website/src/playground/components/Resizable.tsx path="./website/src/playground/components/Resizable.tsx" path="./website/src/playground/components/Resizable.tsx" categories=RuleCategories(SYNTAX)
  2023-10-05T08:29:04.879383Z  INFO biome::worker_2 Process check:Processes formatting: crates/biome_service/src/file_handlers/javascript.rs: Format file ./website/src/playground/components/Resizable.tsx path="./website/src/playground/components/Resizable.tsx" path="./website/src/playground/components/Resizable.tsx"
  ```

#### Enhancements

- Deprecated the environment variable `ROME_BINARY`. Use `BIOME_BINARY` instead. Contributed by @ematipico
- Biome doesn't check anymore the presence of the `.git` folder when VCS support is enabled. Contributed by @ematipico
- `biome rage` doesn't print the logs of the daemon, use `biome rage --daemon-logs` to print them. Contributed by
  @unvalley

### Configuration

#### New features

- Add option `formatter.indentWidth`, and deprecated the option `formatter.indentSize`. Contributed by @ematipico
- Add option `javascript.formatter.indentWidth`, and deprecated the option `javascript.formatter.indentSize`.
  Contributed by @ematipico
- Add option `json.formatter.indentWidth`, and deprecated the option `json.formatter.indentSize`. Contributed by
  @ematipico
- Add option `include` to multiple sections of the configuration
  - `files.include`;
  - `formatter.include`;
  - `linter.include`;
  - `organizeImports.include`;
    When `include` and `ignore` are both specified, `ignore` takes **precedence** over `include`
- Add option `overrides`, where users can modify the behaviour of the tools for certain files or paths.

  For example, it's possible to modify the formatter `lineWidth`, and even `quoteStyle` for certain files that are
  included in glob path `generated/**`:

  ```json
  {
    "formatter": {
      "lineWidth": 100
    },
    "overrides": [
      {
        "include": ["generated/**"],
        "formatter": {
          "lineWidth": 160
        },
        "javascript": {
          "formatter": {
            "quoteStyle": "single"
          }
        }
      }
    ]
  }
  ```

  Or, you can disable certain rules for certain path, and disable the linter for other paths:

  ```json
  {
    "linter": {
      "enabled": true,
      "rules": {
        "recommended": true
      }
    },
    "overrides": [
      {
        "include": ["lib/**"],
        "linter": {
          "rules": {
            "suspicious": {
              "noDebugger": "off"
            }
          }
        }
      },
      {
        "include": ["shims/**"],
        "linter": {
          "enabled": false
        }
      }
    ]
  }
  ```

### Bug fixes

- Fix [#343](https://github.com/biomejs/biome/issues/343), `extends` was incorrectly applied to the `biome.json` file.
  Contributed by @ematipico

### Editors

#### Bug fixes

- Fix [#404](https://github.com/biomejs/biome/issues/404). Biome intellij plugin now works on Windows. Contributed by
  @victor-teles

- Fix [#402](https://github.com/biomejs/biome/issues/402). Biome `format` on intellij plugin now recognize biome.json.
  Contributed by @victor-teles

### Formatter

#### Enhancements

- Use `OnceCell` for the Memoized memory because that's what the `RefCell<Option>` implemented. Contributed by
  @denbezrukov

### Linter

#### Promoted rules

- [complexity/noExcessiveCognitiveComplexity](https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity)
- [complexity/noVoid](https://biomejs.dev/linter/rules/no-void)
- [correctness/useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies)
- [correctness/useHookAtTopLevel](https://biomejs.dev/linter/rules/use-hook-at-top-level)
- [performance/noAccumulatingSpread](https://biomejs.dev/linter/rules/no-accumulating-spread)
- [style/useCollapsedElseIf](https://biomejs.dev/linter/rules/use-collapsed-else-if)
- [suspicious/noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type)
- [suspicious/noFallthroughSwitchClause](https://biomejs.dev/linter/rules/no-fallthrough-switch-clause)
- [suspicious/noGlobalIsFinite](https://biomejs.dev/linter/rules/no-global-is-finite)
- [suspicious/noGlobalIsNan](https://biomejs.dev/linter/rules/no-global-is-nan)
- [suspicious/useIsArray](https://biomejs.dev/linter/rules/use-is-array)

The following rules are now recommended:

- [noAccumulatingSpread](https://biomejs.dev/linter/rules/)
- [noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type)
- [noFallthroughSwitchClause](https://biomejs.dev/linter/rules/no-fallthrough-switch-clause)
- [noForEach](https://biomejs.dev/linter/rules/no-for-each)

#### New features

- Add [noEmptyCharacterClassInRegex](https://biomejs.dev/linter/rules/no-empty-character-class-in-regex) rule. The rule
  reports empty character classes and empty negated character classes in regular expression literals. Contributed by
  @Conaclos

- Add [noMisleadingInstantiator](https://biomejs.dev/linter/rules/no-misleading-instantiator) rule. The rule reports the
  misleading use of the `new` and `constructor` methods. Contributed by @unvalley

- Add [noUselessElse](https://biomejs.dev/linter/rules/no-useless-else) rule.
  The rule reports `else` clauses that can be omitted because their `if` branches break.
  Contributed by @Conaclos

- Add [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports) rule.
  The rule reports unused imports and suggests removing them.
  Contributed by @Conaclos

  [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) reports also unused imports, but don't
  suggest their removal.
  Once [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports) stabilized,
  [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) will not report unused imports.

- Add [useShorthandAssign](https://biomejs.dev/linter/rules/use-shorthand-assign) rule.
  The rule enforce use of shorthand operators that combine variable assignment and some simple mathematical operations.
  For example, x = x + 4 can be shortened to x += 4.
  Contributed by @victor-teles

- Add [useAsConstAssertion](https://biomejs.dev/linter/rules/use-as-const-assertion) rule.
  The rule enforce use of `as const` assertion to infer literal types.
  Contributed by @unvalley

- Add [noMisrefactoredShorthandAssign](https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign) rule.
  The rule reports shorthand assigns when variable appears on both sides. For example `x += x + b`
  Contributed by @victor-teles
- Add [noApproximativeNumericConstant](https://biomejs.dev/linter/rules/no-approximative-numeric-constant/) rule.
  Contributed by @nikeee

-
Add [noInteractiveElementToNoninteractiveRole](https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role)
rule. The rule enforces the non-interactive ARIA roles are not assigned to interactive HTML elements. Contributed by
@nissy-dev

- Add [useAriaActivedescendantWithTabindex](https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex)
  rule. The rule enforces that `tabIndex` is assigned to non-interactive HTML elements with `aria-activedescendant`.
  Contributed by @nissy-dev

- Add [noUselessLoneBlockStatements](https://biomejs.dev/linter/rules/no-useless-lone-block-statements) rule.
  The rule reports standalone blocks that don't include any lexical scoped declaration.
  Contributed by @emab

- Add [noInvalidNewBuiltin](https://biomejs.dev/linter/rules/no-invalid-new-builtin) rule.
  The rule reports use of `new` on `Symbol` and `BigInt`. Contributed by @lucasweng

#### Enhancements

- The following rules have now safe code fixes:

  - [noNegationElse](https://biomejs.dev/linter/rules/no-negation-else)
  - [noUselessLabel](https://biomejs.dev/linter/rules/no-useless-label)
  - [noUselessTypeConstraint](https://biomejs.dev/linter/rules/no-useless-type-constraint)
  - [noUnusedLabels](https://biomejs.dev/linter/rules/no-unused-labels)
  - [useConst](https://biomejs.dev/linter/rules/use-const)
  - [useEnumInitializers](https://biomejs.dev/linter/rules/use-enum-initializers)
  - [useWhile](https://biomejs.dev/linter/rules/use-while)

- [noAccumulatingSpread](https://biomejs.dev/linter/rules/no-accumulating-spread) makes more check in order to reduce
  potential false positives. Contributed by @Vivalldi

- [noConstAssign](https://biomejs.dev/linter/rules/no-const-assign) now provides an unsafe code fix that
  replaces `const` with `let`. Contributed by @vasucp1207

- [noExcessiveComplexity](https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity) default complexity
  threshold is now `15`. Contributed by @arendjr

- [noPositiveTabindexValue](https://biomejs.dev/linter/rules/no-positive-tabindex) now provides an unsafe code fix that
  set to `0` the tab index. Contributed by @vasucp1207

- [noUnusedLabels](https://biomejs.dev/linter/rules/no-unused-labels) no longer reports unbreakable labeled statements.
  Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) now reports unused TypeScript's type
  parameters. Contributed by @Conaclos

- [useAnchorContent](https://biomejs.dev/linter/rules/use-anchor-content) now provides an unsafe code fix that removes
  the `aria-hidden`` attribute. Contributed by @vasucp1207

- [useValidAriaProps](https://biomejs.dev/linter/rules/use-valid-aria-props) now provides an unsafe code fix that
  removes invalid properties. Contributed by @vasucp1207

- `noExcessiveComplexity` was renamed to `noExcessiveCognitiveComplexity`

#### Bug fixes

-
Fix [#294](https://github.com/biomejs/biome/issues/294). [noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type/)
no longer reports false positives for return types. Contributed by @b4s36t4

-
Fix [#313](https://github.com/biomejs/biome/issues/313). [noRedundantUseStrict](https://biomejs.dev/linter/rules/no-redundant-use-strict/)
now keeps leading comments.

-
Fix [#383](https://github.com/biomejs/biome/issues/383). [noMultipleSpacesInRegularExpressionLiterals](https://biomejs.dev/linter/rules/no-multiple-spaces-in-regular-expression-literals)
now provides correct code fixes when consecutive spaces are followed by a quantifier. Contributed by @Conaclos

-
Fix [#397](https://github.com/biomejs/biome/issues/397). [useNumericLiterals](https://biomejs.dev/linter/rules/use-numeric-literals)
now provides correct code fixes for signed numbers. Contributed by @Conaclos

- Fix [452](https://github.com/biomejs/biome/pull/452). The linter panicked when it met a malformed regex (a regex not
  ending with a slash).

- Fix [#104](https://github.com/biomejs/biome/issues/104). We now correctly handle types and values with the same name.

- Fix [#243](https://github.com/biomejs/biome/issues/243) a false positive case where the incorrect scope was defined
  for the `infer` type in rule [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/).
  Contributed by @denbezrukov

- Fix [#322](ttps://github.com/biomejs/biome/issues/322),
  now [noSelfAssign](https://biomejs.dev/linter/rules/no-self-assign/) correctly handles literals inside call
  expressions.

- Changed how [noSelfAssign](https://biomejs.dev/linter/rules/no-self-assign/) behaves. The rule is not triggered
  anymore on function calls. Contributed by @ematipico

### Parser

- Enhance diagnostic for infer type handling in the parser. The 'infer' keyword can only be utilized within the '
  extends' clause of a conditional type. Using it outside this context will result in an error. Ensure that any type
  declarations using 'infer' are correctly placed within the conditional type structure to avoid parsing issues.
  Contributed by @denbezrukov
- Add support for parsing trailing commas inside JSON files:

  ```json
  {
    "json": {
      "parser": {
        "allowTrailingCommas": true
      }
    }
  }
  ```

  Contributed by @nissy-dev

### VSCode

## 1.2.2 (2023-09-16)

### CLI

#### Bug fixes

- Fix a condition where import sorting wasn't applied when running `biome check --apply`

## 1.2.1 (2023-09-15)

### Configuration

- Fix an edge case where the formatter language configuration wasn't picked.
- Fix the configuration schema, where `json.formatter` properties weren't transformed in camel case.

## 1.2.0 (2023-09-15)

### CLI

#### New features

- Add new options to customize the behaviour the formatter based on the language of the file
  - `--json-formatter-enabled`
  - `--json-formatter-indent-style`
  - `--json-formatter-indent-size`
  - `--json-formatter-line-width`
  - `--javascript-formatter-enabled`
  - `--javascript-formatter-indent-style`
  - `--javascript-formatter-indent-size`
  - `--javascript-formatter-line-width`

#### Bug fixes

- Fix a bug where `--errors-on-warning` didn't work when running `biome ci` command.

### Configuration

#### New features

- Add new options to customize the behaviour of the formatter based on the language of the file
  - `json.formatter.enabled`
  - `json.formatter.indentStyle`
  - `json.formatter.indentSize`
  - `json.formatter.lineWidth`
  - `javascript.formatter.enabled`
  - `javascript.formatter.indentStyle`
  - `javascript.formatter.indentSize`
  - `javascript.formatter.lineWidth`

### Linter

#### Promoted rules

New rules are incubated in the nursery group.
Once stable, we promote them to a stable group.
The following rules are promoted:

- [a11y/noAriaUnsupportedElements](https://www.biomejs.dev/linter/rules/no-aria-unsupported-elements/)
- [a11y/noNoninteractiveTabindex](https://www.biomejs.dev/linter/rules/no-noninteractive-tabindex/)
- [a11y/noRedundantRoles](https://www.biomejs.dev/linter/rules/no-redundant-roles/)
- [a11y/useValidAriaValues](https://www.biomejs.dev/linter/rules/use-valid-aria-values/)
- [complexity/noBannedTypes](https://www.biomejs.dev/linter/rules/no-banned-types)
- [complexity/noStaticOnlyClass](https://www.biomejs.dev/linter/rules/no-static-only-class)
- [complexity/noUselessEmptyExport](https://www.biomejs.dev/linter/rules/no-useless-empty-export)
- [complexity/noUselessThisAlias](https://www.biomejs.dev/linter/rules/no-useless-this-alias)
- [correctness/noConstantCondition](https://www.biomejs.dev/linter/rules/no-constant-condition)
- [correctness/noNonoctalDecimalEscape](https://www.biomejs.dev/linter/rules/no-nonoctal-decimal-escape)
- [correctness/noSelfAssign](https://www.biomejs.dev/linter/rules/no-self-assign)
- [style/useLiteralEnumMembers](https://www.biomejs.dev/linter/rules/use-literal-enum-members)
- [style/useNamingConvention](https://www.biomejs.dev/linter/rules/use-naming-convention)
- [suspicious/noControlCharactersInRegex](https://www.biomejs.dev/linter/rules/no-control-characters-in-regex)
- [suspicious/noUnsafeDeclarationMerging](https://www.biomejs.dev/linter/rules/no-unsafe-declaration-merging)
- [suspicious/useGetterReturn](https://www.biomejs.dev/linter/rules/use-getter-return)

#### New rules

- Add [noConfusingVoidType](https://biomejs.dev/linter/rules/no-confusing-void-type/) rule. The rule reports the unusual
  use of the `void` type. Contributed by @shulandmimi

#### Removed rules

- Remove `noConfusingArrow`

  Code formatters, such as prettier and Biome, always adds parentheses around the parameter or the body of an arrow
  function.
  This makes the rule useless.

  Contributed by @Conaclos

#### Enhancements

- [noFallthroughSwitchClause](https://biomejs.dev/linter/rules/no-fallthrough-switch-clause/) now relies on control flow
  analysis to report most of the switch clause fallthrough. Contributed by @Conaclos

- [noAssignInExpressions](https://biomejs.dev/linter/rules/no-assign-in-expressions/) no longer suggest code fixes. Most
  of the time the suggestion didn't match users' expectations. Contributed by @Conaclos

- [noUselessConstructor](https://biomejs.dev/linter/rules/no-useless-constructor/) no longer emits safe code fixes.
  Contributed by @Conaclos

  All code fixes are now emitted as unsafe code fixes.
  Removing a constructor can change the behavior of a program.

- [useCollapsedElseIf](https://biomejs.dev/linter/rules/use-collapsed-else-if/) now only provides safe code fixes.
  Contributed by @Conaclos

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) now reports more cases.

  The rule is now able to ignore self-writes.
  For example, the rule reports the following unused variable:

  ```js
  let a = 0;
  a++;
  a += 1;
  ```

  The rule is also capable of detecting an unused declaration that uses itself.
  For example, the rule reports the following unused interface:

  ```ts
  interface I {
    instance(): I
  }
  ```

  Finally, the rule now ignores all _TypeScript_ declaration files,
  including [global declaration files](https://www.typescriptlang.org/docs/handbook/declaration-files/templates/global-d-ts.html).

  Contributed by @Conaclos

#### Bug fixes

- Fix [#182](https://github.com/biomejs/biome/issues/182),
  making [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) retains optional chaining. Contributed by
  @denbezrukov

- Fix [#168](https://github.com/biomejs/biome/issues/168),
  fix [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies) false positive case when
  stable hook is on a new line. Contributed by @denbezrukov

- Fix [#137](https://github.com/biomejs/biome/issues/137),
  fix [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/) false positive case with TypeScript module
  declaration:

  ```typescript
  declare module '*.gif' {
      const src: string;
  }

  declare module '*.bmp' {
      const src: string;
  }
  ```
  Contributed by @denbezrukov

- Fix [#258](https://github.com/biomejs/biome/issues/258),
  fix [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/) the case where the rule removing an
  assignment. Contributed by @denbezrukov
- Fix [#266](https://github.com/biomejs/biome/issues/266), where `complexity/useLiteralKeys` emitted a code action with
  an invalid AST. Contributed by @ematipico


- Fix [#105](https://github.com/biomejs/biome/issues/105), removing false positives reported
  by [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/).

  The rule no longer reports the following used variable:

  ```js
  const a = f(() => a);
  ```

  Contributed by @Conaclos

### VSCode

#### Enhancements

- Improve server binary resolution when using certain package managers, notably pnpm.

  The new strategy is to point to `node_modules/.bin/biome` path,
  which is consistent for all package managers.

## 1.1.2 (2023-09-07)

### Editors

#### Bug fixes

- Fix a case where an empty JSON file would cause the LSP server to crash. Contributed by @ematipico

### Linter

#### Enhancements

- [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) now accepts import namespaces in
  _PascalCase_ and rejects export namespaces in _CONSTANT\_CASE_.

  The following code is now valid:

  ```js
  import * as React from "react";
  ```

  And the following code is now invalid:

  ```js
  export * as MY_NAMESPACE from "./lib.js";
  ```

  Contributed by @Conaclos

- [noUselessConstructor](https://biomejs.dev/linter/rules/no-useless-constructor/) now ignores decorated classes and
  decorated parameters. The rule now gives suggestions instead of safe fixes when parameters are annotated with types.
  Contributed by @Conaclos

## 1.1.1 (2023-09-07)

### Analyzer

#### Bug fixes

- The diagnostic for `// rome-ignore` suppression comment should not be a warning. A warning could block the CI, marking
  a gradual migration difficult. The code action that changes `// rome-ignore` to `// biome-ignore` is disabled as
  consequence. Contributed by @ematipico

## 1.1.0 (2023-09-06)

### Analyzer

#### Enhancements

- Add a code action to replace `rome-ignore` with `biome-ignore`. Use `biome check --apply-unsafe` to update all the
  comments. The action is not bulletproof, and it might generate unwanted code, that's why it's unsafe action.
  Contributed by @ematipico

### CLI

#### Enhancements

- Biome now reports a diagnostics when a `rome.json` file is found.
- `biome migrate --write` creates `biome.json` from `rome.json`, but it won't delete the `rome.json` file. Contributed
  by @ematipico

#### Bug fixes

- Biome uses `biome.json` first, then it attempts to use `rome.json`.
- Fix a case where Biome couldn't compute correctly the ignored files when the VSC integration is enabled. Contributed
  by @ematipico

### Configuration

### Editors

#### Bug fixes

- The LSP now uses its own socket and won't rely on Biome's socket. This fixes some cases where users were seeing
  multiple servers in the `rage` output.

### Formatter

#### Enhancements

- You can use `// biome-ignore` as suppression comment.
- The `// rome-ignore` suppression is deprecated.

### JavaScript APIs

### Linter

#### New features

- Add [useCollapsedElseIf](https://biomejs.dev/linter/rules/use-collapsed-else-if/) rule. This new rule requires merging
  an `else` and an `if`, if the `if` statement is the only statement in the `else` block. Contributed by @n-gude

#### Enhancements

- [useTemplate](https://biomejs.dev/linter/rules/use-template/) now reports all string concatenations.

  Previously, the rule ignored concatenation of a value and a newline or a backquote.
  For example, the following concatenation was not reported:

  ```js
  v + "\n";
  "`" + v + "`";
  ```

  The rule now reports these cases and suggests the following code fixes:

  ```diff
  - v + "\n";
  + `${v}\n`;
  - v + "`";
  + `\`${v}\``;
  ```

  Contributed by @Conaclos

- [useExponentiationOperator](https://biomejs.dev/linter/rules/use-exponentiation-operator/) suggests better code fixes.

  The rule now preserves any comment preceding the exponent,
  and it preserves any parenthesis around the base or the exponent.
  It also adds spaces around the exponentiation operator `**`,
  and always adds parentheses for pre- and post-updates.

  ```diff
  - Math.pow(a++, /**/ (2))
  + (a++) ** /**/ (2)
  ```

  Contributed by @Conaclos

- You can use `// biome-ignore` as suppression comment.

- The `// rome-ignore` suppression is deprecated.

#### Bug fixes

- Fix [#80](https://github.com/biomejs/biome/issues/95),
  making [noDuplicateJsxProps](https://biomejs.dev/linter/rules/no-duplicate-jsx-props/) case-insensitive.

  Some frameworks, such as Material UI, rely on the case-sensitivity of JSX properties.
  For
  example, [TextField has two properties with the same name, but distinct cases](https://mui.com/material-ui/api/text-field/#TextField-prop-inputProps):

  ```jsx
  <TextField inputLabelProps="" InputLabelProps=""></TextField>
  ```

  Contributed by @Conaclos

- Fix [#138](https://github.com/biomejs/biome/issues/138)

  [noCommaOperator](https://biomejs.dev/linter/rules/no-comma-operator/) now correctly ignores all use of comma
  operators inside the update part of a `for` loop.
  The following code is now correctly ignored:

  ```js
  for (
    let i = 0, j = 1, k = 2;
    i < 100;
    i++, j++, k++
  ) {}
  ```

  Contributed by @Conaclos

- Fix [rome#4713](https://github.com/rome/tools/issues/4713).

  Previously, [useTemplate](https://biomejs.dev/linter/rules/use-template/) made the following suggestion:

  ```diff
  - a + b + "px"
  + `${a}${b}px`
  ```

  This breaks code where `a` and `b` are numbers.

  Now, the rule makes the following suggestion:

  ```diff
  - a + b + "px"
  + `${a + b}px`
   ```

  Contributed by @Conaclos

- Fix [rome#4109](https://github.com/rome/tools/issues/4109)

  Previously, [useTemplate](https://biomejs.dev/linter/rules/use-template/) suggested an invalid code fix when a leading
  or trailing single-line comment was present:

  ```diff
    // leading comment
  - 1 /* inner comment */ + "+" + 2 // trailing comment
  + `${// leading comment
  + 1 /* inner comment */}+${2 //trailing comment}` // trailing comment
  ```

  Now, the rule correctly handles this case:

  ```diff
    // leading comment
  - 1 + "+" + 2 // trailing comment
  + `${1}+${2}` // trailing comment
  ```

  As a sideeffect, the rule also suggests the removal of any inner comments.

  Contributed by @Conaclos

- Fix [rome#3850](https://github.com/rome/tools/issues/3850)

  Previously [useExponentiationOperator](https://biomejs.dev/linter/rules/use-exponentiation-operator/) suggested
  invalid code in a specific edge case:

  ```diff
  - 1 +Math.pow(++a, 2)
  + 1 +++a**2
  ```

  Now, the rule properly adds parentheses:

  ```diff
  - 1 +Math.pow(++a, 2)
  + 1 +(++a) ** 2
  ```

  Contributed by @Conaclos

- Fix [#106](https://github.com/biomejs/biome/issues/106)

  [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) now correctly recognizes some
  TypeScript types such as `Uppercase`.

  Contributed by @Conaclos

- Fix [rome#4616](https://github.com/rome/tools/issues/4616)

  Previously [noUnreachableSuper](https://biomejs.dev/linter/rules/no-unreachable-super/) reported valid codes with
  complex nesting of control flow structures.

  Contributed by @Conaclos

## 1.0.0 (2023-08-28)

### Analyzer

#### BREAKING CHANGES

- The organize imports feature now groups import statements by "distance".

  Modules "farther" from the user are put on the top, and modules "closer" to the user are placed on the bottom.
  Check the [documentation](https://biomejs.dev/analyzer/) for more information about it.

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

  Previously, biome would raise warnings and continue the execution by applying its defaults.

  This could have been better for users because this could have created false positives in linting or formatted
  code with a configuration that wasn't the user's.

- The command `biome check` now shows formatter diagnostics when checking the code.

  The diagnostics presence will result in an error code when the command finishes.

  This aligns with semantic and behaviour meant for the command `biome check`.

- `init` command emits a `biome.json` file;

#### Other changes

- Fix [#4670](https://github.com/rome/tools/issues/4670), don't crash at empty default export.

- Fix [#4556](https://github.com/rome/tools/issues/4556), which correctly handles new lines in the
  `.gitignore` file across OS.

- Add a new option to ignore unknown files `--files-ignore-unknown`:

    ```shell
    biome format --files-ignore-unknown ./src
    ```

  Doing so, Biome won't emit diagnostics for files that doesn't know how to handle.

- Add the new option `--no-errors-on-unmatched`:

    ```shell
    biome format --no-errors-on-unmatched ./src
    ```

  Biome doesn't exit with an error code if no files were processed in the given paths.

- Fix the diagnostics emitted when running the `biome format` command.

- Biome no longer warns when discovering (possibly infinite) symbolic links between directories.

  This fixes [#4193](https://github.com/rome/tools/issues/4193) which resulted in incorrect warnings
  when a single file or directory was pointed at by multiple symbolic links. Symbolic links to other
  symbolic links do still trigger warnings if they are too deeply nested.

- Introduced a new command called `biome lint`, which will only run lint rules against the code base.

- Biome recognizes known files as "JSON files with comments allowed":

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
- Add support for `biome.json`;

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

- Add a new `"javascript"` option to support the unsafe/experimental parameter decorators:

    ```json
    {
       "javascript": {
          "parser": {
             "unsafeParameterDecoratorsEnabled": true
          }
       }
    }
    ```

- Add a new `"extends"` option, useful to split the configuration file in multiple files:

  ```json
  {
    "extends": ["../sharedFormatter.json", "linter.json"]
  }
  ```

  The resolution of the files is file system based, Biome doesn't know how to
  resolve dependencies yet.

- The commands `biome check` and `biome lint` now show the remaining diagnostics even when
  `--apply-safe` or `--apply-unsafe` are passed.

- Fix the commands `biome check` and `biome lint`,
  they won't exit with an error code if no error diagnostics are emitted.

- Add a new option `--error-on-warnings`,
  which instructs Biome to exit with an error code when warnings are emitted.

  ```shell
  biome check --error-on-warnings ./src
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

- The Biome LSP no longer applies unsafe quickfixes on-save when `editor.codeActionsOnSave.quickfix.biome` is enabled.

- Fix [#4564](https://github.com/rome/tools/issues/4564); files too large don't emit errors.

- The Biome LSP sends client messages when files are ignored or too big.

### Formatter

- Add a new option called `--jsx-quote-style`.

  This option lets you choose between single and double quotes for JSX attributes.

- Add the option `--arrow-parentheses`.

  This option allows setting the parentheses style for arrow functions.

- The _JSON_ formatter can now format `.json` files with comments.

### Linter

#### Removed rules

- Remove `complexity/noExtraSemicolon` ([#4553](https://github.com/rome/tools/issues/4553))

  The _Biome_ formatter takes care of removing extra semicolons.
  Thus, there is no need for this rule.

- Remove `useCamelCase`

  Use [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) instead.

#### New rules

- Add [noExcessiveComplexity](https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity/)

- Add [useImportRestrictions](https://biomejs.dev/linter/rules/use-import-restrictions/)

- Add [noFallthroughSwitchClause](https://biomejs.dev/linter/rules/no-fallthrough-switch-clause/)

- Add [noGlobalIsFinite](https://biomejs.dev/linter/rules/no-global-is-finite/)

  This rule recommends using `Number.isFinite` instead of the global and unsafe `isFinite` that attempts a type
  coercion.

- Add [noGlobalIsNan](https://biomejs.dev/linter/rules/no-global-is-nan/)

  This rule recommends using `Number.isNaN` instead of the global and unsafe `isNaN` that attempts a type coercion.

- Add [noUnsafeDeclarationMerging](https://biomejs.dev/linter/rules/no-unsafe-declaration-merging/)

  This rule disallows declaration merging between an interface and a class.

- Add [noUselessThisAlias](https://biomejs.dev/linter/rules/no-useless-this-alias/)

  This rule disallows useless aliasing of `this` in arrow functions.

- Add [useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/)

  This rule proposes turning function expressions into arrow functions.
  Function expressions that use `this` are ignored.

- Add [noDuplicateJsonKeys](https://biomejs.dev/linter/rules/no-duplicate-json-keys/)

  This rule disallow duplicate keys in a JSON object.

- Add [noVoid](https://biomejs.dev/linter/rules/no-void/)

  This rule disallows the use of `void`.

- Add [noNonoctalDecimalEscape](https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape/)

  This rule disallows `\8` and `\9` escape sequences in string literals.

- Add [noUselessEmptyExport](https://biomejs.dev/linter/rules/no-useless-empty-export/)

  This rule disallows useless `export {}`.

- Add [useIsArray](https://biomejs.dev/linter/rules/use-is-array/)

  This rule proposes using `Array.isArray()` instead of `instanceof Array`.

- Add [useGetterReturn](https://biomejs.dev/linter/rules/use-getter-return/)

  This rule enforces the presence of non-empty return statements in getters.
  This makes the following code incorrect:

  ```js
  class Person {
      get firstName() {}
  }
  ```

#### Promoted rules

New rules are promoted, please check [#4750](https://github.com/rome/tools/discussions/4750) for more details:

- [a11y/useHeadingContent](https://biomejs.dev/linter/rules/use-heading-content/)
- [complexity/noForEach](https://biomejs.dev/linter/rules/no-for-each/)
- [complexity/useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/)
- [complexity/useSimpleNumberKeys](https://biomejs.dev/linter/rules/use-simple-number-keys/)
- [correctness/useIsNan](https://biomejs.dev/linter/rules/use-is-nan/)
- [suspicious/noConsoleLog](https://biomejs.dev/linter/rules/no-console-log/)
- [suspicious/noDuplicateJsxProps](https://biomejs.dev/linter/rules/no-duplicate-jsx-props/)

The following rules are now recommended:

**- [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/)

- [noRedundantUseStrict](https://biomejs.dev/linter/rules/no-redundant-use-strict/)
- [useExponentiationOperator](https://biomejs.dev/linter/rules/use-exponentiation-operator/)**

#### Other changes

- Add new TypeScript globals (`AsyncDisposable`, `Awaited`, `DecoratorContext`, and
  others) [4643](https://github.com/rome/tools/issues/4643).

- [noRedeclare](https://biomejs.dev/linter/rules/no-redeclare/): allow redeclare of index signatures are in different
  type members [#4478](https://github.com/rome/tools/issues/4478)

-
Improve [noConsoleLog](https://biomejs.dev/linter/rules/no-console-log/), [noGlobalObjectCalls](https://biomejs.dev/linter/rules/no-global-object-calls/), [useIsNan](https://biomejs.dev/linter/rules/use-is-nan/),
and [useNumericLiterals](https://biomejs.dev/linter/rules/use-numeric-literals/) by handling `globalThis` and `window`
namespaces.

For instance, the following code is now reported by `noConsoleLog`:

```js
globalThis.console.log("log")
```

- Improve [noDuplicateParameters](https://biomejs.dev/linter/rules/no-duplicate-parameters/) to manage constructor
  parameters.

- Improve [noInnerDeclarations](https://biomejs.dev/linter/rules/no-inner-declarations/)

  Now, the rule doesn't report false-positives about ambient _TypeScript_ declarations.
  For example, the following code is no longer reported by the rule:

  ```ts
  declare var foo;
  ```

- Improve [useEnumInitializers](https://biomejs.dev/linter/rules/use-enum-initializers/)

  The rule now reports all uninitialized members of an enum in a single diagnostic.

  Moreover, ambient enum declarations are now ignored.
  This avoids reporting ambient enum declarations in _TypeScript_ declaration files.

  ```ts
  declare enum Weather {
    Rainy,
    Sunny,
  }
  ```

- Relax [noBannedTypes](https://biomejs.dev/linter/rules/no-banned-types/) and improve documentation

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

- Improve [noConstantCondition](https://biomejs.dev/linter/rules/no-constant-condition/)

  The rule now allows `while(true)`.
  This recognizes a common pattern in the web community:

  ```js
  while (true) {
    if (cond) {
      break;
    }
  }
  ```

- Improve the diagnostic and the code action
  of [useDefaultParameterLast](https://biomejs.dev/linter/rules/use-default-parameter-last/).

  The diagnostic now reports the last required parameter which should precede optional and default parameters.

  The code action now removes any whitespace between the parameter name and its initialization.

- Relax `noConfusingArrow`

  All arrow functions that enclose its parameter with parenthesis are allowed.
  Thus, the following snippet no longer trigger the rule:

  ```js
  var x = (a) => 1 ? 2 : 3;
  ```

  The following snippet still triggers the rule:

  ```js
  var x = a => 1 ? 2 : 3;
  ```

- Relax [useLiteralEnumMembers](https://biomejs.dev/linter/rules/use-literal-enum-members/)

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

- Improve [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/).

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

- Improve [noNewSymbol](https://biomejs.dev/linter/rules/no-new-symbol/).

  The rule now handles cases where `Symbol` is namespaced with the global `globalThis` or `window`.

- The rules [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/)
  and [useHookAtTopLevel](https://biomejs.dev/linter/rules/use-hook-at-top-level/) accept a different shape of options

  Old configuration:

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

  New configuration:

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

- [noRedundantUseStrict](https://biomejs.dev/linter/rules/no-redundant-use-strict/) check only `'use strict'` directive
  to resolve false positive diagnostics.

  React introduced new directives, "use client" and "use server".
  The rule raises false positive errors about these directives.

- Fix a crash in the [NoParameterAssign](https://biomejs.dev/linter/rules/no-parameter-assign/) rule that occurred when
  there was a bogus binding. [#4323](https://github.com/rome/tools/issues/4323)

- Fix [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) in the following
  cases [#4330](https://github.com/rome/tools/issues/4330):

  - when the first argument of hooks is a named function
  - inside an export default function
  - for `React.use` hooks

- Fix [noInvalidConstructorSuper](https://biomejs.dev/linter/rules/no-invalid-constructor-super/) that erroneously
  reported generic parents [#4624](https://github.com/rome/tools/issues/4624).

- Fix [noDuplicateCase](https://biomejs.dev/linter/rules/no-duplicate-case/) that erroneously reported as equals the
  strings literals `"'"` and `'"'` [#4706](https://github.com/rome/tools/issues/4706).

- Fix [NoUnreachableSuper](https://biomejs.dev/linter/rules/no-unreachable-super/)'s false positive
  diagnostics ([#4483](https://github.com/rome/tools/issues/4483)) caused to nested if statement.

  The rule no longer reports `This constructor calls super() in a loop`
  when using nested if statements in a constructor.

- Fix [useHookAtTopLevel](https://biomejs.dev/linter/rules/use-hook-at-top-level/)'s false positive
  diagnostics ([#4637](https://github.com/rome/tools/issues/4637))

  The rule no longer reports false positive diagnostics when accessing properties directly from a hook and calling a
  hook inside function arguments.

- Fix [noUselessConstructor](https://biomejs.dev/linter/rules/no-useless-constructor/) which erroneously reported
  constructors with default parameters [rome#4781](https://github.com/rome/tools/issues/4781)

- Fix [noUselessFragments](https://biomejs.dev/linter/rules/no-useless-fragments/)'s panics when
  running `biome check --apply-unsafe` ([#4637](https://github.com/rome/tools/issues/4639))

  This rule's code action emits an invalid AST, so I fixed using JsxString instead of JsStringLiteral

- Fix [noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/)'s false positive
  diagnostics ([#4675](https://github.com/rome/tools/issues/4675))

  The semantic analyzer no longer handles `this` reference identifier.

- Fix [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/)'s false positive
  diagnostics ([#4688](https://github.com/rome/tools/issues/4688))

  The semantic analyzer handles ts export declaration clause correctly.

### Parser

- Add support for decorators in class method parameters, example:

    ```js
    class AppController {
       get(@Param() id) {}
       // ^^^^^^^^ new supported syntax
    }
    ```

  This syntax is only supported via configuration, because it's a non-standard syntax.

    ```json
    {
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
