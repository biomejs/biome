---
title: Biome v1.7
description: Migrate from Prettier and ESLint with one command!
summary: |
  This new version provides an easy path to migrate from ESLint and Prettier.
  As usual, it also comes with new linter rules.
authors:
  - conaclos
  - team
pubDate: 2024-04-15
coverImage:
  lightSrc: "@/assets/blog/roadmap-2024/banner-light.png"
  darkSrc: "@/assets/blog/roadmap-2024/banner-dark.png"
  alt: The brand of the project. It says "Biome, toolchain of the web"
socialImage: "@/assets/social-logo.png"
---

This new version provides an easy path to migrate from ESLint and Prettier.
As usual, it also comes with new linter rules.

Update Biome using the following commands:

```
npm install --save-dev --save-exact @biomejs/biome@latest
npx @biomejs/biome migrate
```

## Migrate from _ESLint_ with a single command

This release introduces a new command called `biome migrate eslint`.
This command will read your _ESLint_ configurations and attempt to port their settings to Biome.

The command is able to handle both the legacy and the flat _ESLint_ configurations.
It supports the `extends` field of the legacy configuration and loads both shared and plugin configurations!
The command also attempts to migrate `.eslintignore`.

Given the following _ESLint_ configuration:

```json
{
  "extends": ["plugin:unicorn/recommended"],
  "plugins": ["unicorn"],
  "ignore_patterns": ["dist/**"],
  "globals": {
    "Global1": "readonly"
  },
  "rules": {
    "eqeqeq": "error"
  },
  "overrides": [
    {
      "files": ["tests/**"],
      "rules": {
        "eqeqeq": "off"
      }
    }
  ]
}
```

And the following _Biome_ configuration (obtained by running `biome init`):

```json
{
	"organizeImports": {
		"enabled": true
	},
	"linter": {
		"enabled": true,
		"rules": {
			"recommended": true
		}
	}
}
```

Run `biome migrate eslint --write` to migrate your ESLint configuration to Biome.
This results in the following _Biome_ configuration.
Note that this overrides your initial _Biome_ configuration.

```json
{
	"organizeImports": { "enabled": true },
	"linter": {
		"enabled": true,
		"rules": {
			"recommended": false,
			"complexity": {
				"noForEach": "error",
				"noStaticOnlyClass": "error",
				"noUselessSwitchCase": "error",
				"useFlatMap": "error"
			},
			"style": {
				"noNegationElse": "off",
				"useForOf": "error",
				"useNodejsImportProtocol": "error",
				"useNumberNamespace": "error"
			},
			"suspicious": {
				"noDoubleEquals": "error",
				"noThenProperty": "error",
				"useIsArray": "error"
			}
		}
	},
	"javascript": { "globals": ["Global1"] },
	"overrides": [
		{
			"include": ["tests/**"],
			"linter": { "rules": { "suspicious": { "noDoubleEquals": "off" } } }
		}
	]
}
```

The command requires _Node.js_ to be installed to load _JavaScript_ configurations such as `eslint.config.js` and to resolve the `extends` field.
For now, `biome migrate eslint` doesn't support configuration written in _YAML_.


## Migrate from _Prettier_ with a single command

[_Biome v1.6_ introduced the command `biome migrate prettier`](/blog/biome-v1-6/#easier-migration-from-prettier).

This new version adds the support of the `overrides` field and attempts to convert `.prettierignore` glob patterns to globs supported by _Biome_.


## Emit formatting and linting reports

_Biome_ is now able to output _JSON_ reports detailing the diagnostics obtained from a run.

For instance, you can emit a report when you lint a codebase:

```shell
biome lint --reporter=json-pretty .
```

For now, we support two report formats: `json` and `json-pretty`.

Note that the report format is subject to breaking changes.
So you should not rely on it yet.
Please try this feature and let us know if any information is missing from the reports.


## Check _Git_ staged files

_Biome v1.5_ added the `--changed` flag on its main commands `biome format`, `biome lint`, and `biome check ` to format and lint _Git_ tracked files that have been changed.

Today we are introducing a new flag `--staged` which allows you to check only files that have been added to the Git index (_staged files_).
This is useful for checking that the files you want to commit are formatted and linted.

This is handy for writing your own [pre-commit script](/recipes/git-hooks/#shell-script).
Note that, unstaged changes on a staged file are **not** ignored.
Thus, we still recommend using a [dedicated pre-commit tool](/recipes/git-hooks/).

`--changed` and `--staged` are not available on the command `biome ci` because they don't make sense in a CI environment.


## Linter

### Promoted rules

Once stable, a nursery rule is promoted to a stable group
The following rules are promoted:

- [complecity/noExcessiveNestedTestSuites](https://biomejs.dev/linter/rules/no-excessive-nested-test-suites)
- [complexity/noUselessTernary](https://biomejs.dev/linter/rules/no-useless-ternary/)
- [correctness/useJsxKeyInIterable](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/)
- [performance/noBarrelFile](https://biomejs.dev/linter/rules/no-barrel-file/)
- [performance/noReExportAll](https://biomejs.dev/linter/rules/no-re-export-all/)
- [style/noNamespaceImport](https://biomejs.dev/linter/rules/no-namespace-import/)
- [style/useNodeAssertStrict](https://biomejs.dev/linter/rules/use-node-assert-strict/)
- [suspicious/noDuplicateTestHooks](https://biomejs.dev/linter/rules/no-duplicate-test-hooks/)
- [suspicious/noExportsInTest](https://biomejs.dev/linter/rules/no-exports-in-test/)
- [suspicious/noFocusedTests](https://biomejs.dev/linter/rules/no-focused-tests/)
- [suspicious/noSkippedTests](https://biomejs.dev/linter/rules/no-skipped-tests/)
- [suspicious/noSuspiciousSemicolonInJsx](https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx)

### New nursery rules

Since _Biome 1.6_, we added several new rules.
New rules are incubated in the nursery group.
Nursery rules are subject to breaking changes.
The new rules are:

- [nursery/noDoneCallback](https://biomejs.dev/linter/rules/no-done-callback/)
- [nursery/noDuplicateElseIf](https://biomejs.dev/linter/rules/no-duplicate-else-if/)
- [nursery/noEvolvingAny](https://biomejs.dev/linter/rules/no-evolving-any/)
- [nursery/noMisplacedAssertion](https://biomejs.dev/linter/rules/no-misplaced-assertion/)


## Miscellaneous

- Biome is able to apply specific settings to [well-known files](https://biomejs.dev/guides/how-biome-works/#well-known-files).
  It now recognized more files and is able to differentiate JSON files that allows only comments and JSON files that allows both comments and trailing commas.
- Biome now displays the location of a parsing error for its configuration file.
- Biome extension is now able to parse the JSX syntax in files that associated with the _JavaScript_ language identifier.
  In React ecosystem, `.js` files are allowed to include JSX syntax.
- You can now ignore `React` imports in the rules [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/#options) and [useImportType](https://biomejs.dev/linter/rules/use-import-type/#options) by setting [`javascript.jsxRuntime`](https://biomejs.dev/reference/configuration/#javascriptjsxruntime) to `reactClassic`.
- [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now supports _Preact_.

See the [changelog](/internals/changelog/#170-2024-04-15) for more details.
