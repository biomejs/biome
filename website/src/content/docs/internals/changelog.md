---
title: Changelog
description: The changelog of Biome
tableOfContents:
    maxHeadingLevel: 2
---
# Biome changelog

This project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Due to the nature of Biome as a toolchain,
it can be unclear what changes are considered major, minor, or patch.
Read our [guidelines to categorize a change](https://biomejs.dev/internals/versioning).

New entries must be placed in a section entitled `Unreleased`.
Read
our [guidelines for writing a good changelog entry](https://github.com/biomejs/biome/blob/main/CONTRIBUTING.md#changelog).

## Unreleased

### Analyzer

#### Bug fixes

- Now Biome can detect the script language in Svelte and Vue script blocks more reliably ([#2245](https://github.com/biomejs/biome/issues/2245)). Contributed by @Sec-ant

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

#### Bug fixes

- Biome now tags the diagnostics emitted by `organizeImports` and `formatter` with correct severity levels, so they will be properly filtered by the flag `--diagnositic-level` ([#2288](https://github.com/biomejs/biome/issues/2288)). Contributed by @Sec-ant

- Biome now correctly filters out files that are not present in the current directory when using the `--changed` flag [#1996](https://github.com/biomejs/biome/issues/1996). Contributed by @castarco

### Configuration

#### Bug fixes

- Now setting group level `all` to `false` can disable recommended rules from that group when top level `recommended` is `true` or unset. Contributed by @Sec-ant

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

### Formatter

### JavaScript APIs

### Linter

#### New features

- Add a new option `ignoreReact` to [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports).

  When `ignoreReact` is enabled, Biome ignores imports of `React` from the `react` package.
  The option is disabled by default.

  Contributed by @Conaclos

- Implement [#2043](https://github.com/biomejs/biome/issues/2043): The React rule [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) is now also compatible with Preact hooks imported from `preact/hooks` or `preact/compat`. Contributed by @arendjr

#### Enhancements

- [style/useFilenamingConvention](https://biomejs.dev/linter/rules/use-filenaming-convention/) now allows prefixing a filename with `+` ([#2341](https://github.com/biomejs/biome/issues/2341)).

  This is a convention used by [Sveltekit](https://kit.svelte.dev/docs/routing#page) and [Vike](https://vike.dev/route).

  Contributed by @Conaclos

#### Bug fixes

- Lint rules `useNodejsImportProtocol`, `useNodeAssertStrict`, `noRestrictedImports`, `noNodejsModules` will no longer check `declare module` statements anymore. Contributed by @Sec-ant

### Parser


## 1.6.4 (2022-04-03)

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

- Add rule [noEvolvingAny](https://biomejs.dev/linter/rules/no-evolving-any) to disallow variables from evolving into `any` type through reassignments. Contributed by @fujiyamaorange

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

  The following type parameters are now reported as a redeclaraion:

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

- Add lint rule useJsxKeyInIterable from Eslint rule [`react/jsx-key`](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-key.md). Contributed by @vohoanglong0107
- The analyzer now **infers** the correct quote from `javascript.formatter.quoteStyle`, if set. This means that code fixes suggested by the analyzer will use the same quote of the formatter. Contributed by @ematipico

#### Enhancements

- [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables) ignores unused rest spread silbings.

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

- Add rule [noUndeclaredependencies](https://biomejs.dev/linter/rules/no-undeclared-dependencies), to detect the use of
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

- Add rule [noSemicolonInJsx](https://biomejs.dev/linter/rules/no-semicolon-in-jsx/) to detect possible wrong semicolons inside JSX elements.

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

- Add option `--bracket-same-line` to the `format` command. Contributed by @faultyserve

- Add option `--bracket-spacing` to the `format` command. Contributed by @faultyserve

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
  biome check --error-on-wanrings ./src
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
