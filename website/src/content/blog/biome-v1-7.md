---
title: Biome v1.7
description: Migrate from Prettier and ESLint with one command!
summary: |
  This new version provides an easy path to migrate from ESLint and Prettier.
  It also introduces machine-readable reports for the formatter and the linter, new linter rules, and many fixes.
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

Today we’re excited to announce the release of _Biome v1.7_!

This new version provides an easy path to migrate from ESLint and Prettier.
It also introduces experimental machine-readable reports for the formatter and the linter, new linter rules, and many fixes.

If you’re not familiar with _Biome_ yet, it is a fast formatter and linter for JavaScript, TypeScript, JSX, and JSON that scores [97% compatibility with Prettier](https://console.algora.io/challenges/prettier) and provides [more than 200 linter rules](/linter/rules/).

Update _Biome_ using the following commands:

```
npm install --save-dev --save-exact @biomejs/biome@latest
npx @biomejs/biome migrate
```

## Migrate from _ESLint_ with a single command

This release introduces a new command `biome migrate eslint`.
This command will read your _ESLint_ configurations and attempt to port their settings to Biome.

The command handles both the legacy and the flat _ESLint_ configurations.
It supports the `extends` field of the legacy configuration and loads both shared and plugin configurations!
The command also migrates `.eslintignore`.

Given the following _ESLint_ configuration:

```json title=".eslintrc.json"
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

```json title="biome.json"
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

Run `biome migrate eslint --write` to migrate your _ESLint_ configuration to _Biome_.
This results in the following _Biome_ configuration:

```json title="biome.json"
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

Note that this overrides your initial _Biome_ configuration.
The command requires _Node.js_ to be installed to load _JavaScript_ configurations such as `eslint.config.js` and to resolve the `extends` field.
For now, `biome migrate eslint` doesn't support configuration written in _YAML_.

We have a [dedicated page](/linter/rules-sources/) that lists the equivalent _Biome_ rule of a given _ESLint_ rule.
We handle some _ESLint_ plugins such as _TypeScript ESLint_, _ESLint JSX A11y_, _ESLint React_, and _ESLint Unicorn_.
Some rules are equivalent to their _ESLint_ counterparts, while others are inspired.
By default, _Biome_ doesn't migrate inspired rules.
You can use the _CLI_ flag `--include-inspired` to migrate them.

## Migrate from _Prettier_ with a single command

[_Biome v1.6_ introduced the command `biome migrate prettier`](/blog/biome-v1-6/#easier-migration-from-prettier).

This new version adds the support of the `overrides` field and attempts to convert `.prettierignore` glob patterns to globs supported by _Biome_.

Given the following `.prettierrc.json`

```json title=".prettierrc.json"
{
	"useTabs": false,
	"singleQuote": true,
	"overrides": [
		{
      		"files": ["*.json"],
      		"options": { "tabWidth": 2 }
    	}
	]
}
```

Run `biome migrate prettier --write` to migrate your _Prettier_ configuration to _Biome_.
This results in the following _Biome_ configuration:

```json title="biome.json"
{
	"formatter": {
		"enabled": true,
		"formatWithErrors": false,
		"indentStyle": "space",
		"indentWidth": 2,
		"lineEnding": "lf",
		"lineWidth": 80,
		"attributePosition": "auto"
	},
	"organizeImports": { "enabled": true },
	"linter": { "enabled": true, "rules": { "recommended": true } },
	"javascript": {
		"formatter": {
			"jsxQuoteStyle": "double",
			"quoteProperties": "asNeeded",
			"trailingComma": "all",
			"semicolons": "asNeeded",
			"arrowParentheses": "always",
			"bracketSpacing": true,
			"bracketSameLine": false,
			"quoteStyle": "single",
			"attributePosition": "auto"
		}
	},
	"overrides": [
		{
			"include": ["*.json"],
			"formatter": {
				"indentWidth": 2
			}
		}
	]
}
```

Note that this overrides your initial _Biome_ configuration.
The command requires _Node.js_ to be installed to load _JavaScript_ configurations such as `.prettierrc.js`.
For now, `biome migrate eslint` doesn't support configuration written in _JSON5_, _TOML_, or _YAML_.


## Emit formatting and linting reports

_Biome_ is now able to output _JSON_ reports detailing the diagnostics obtained from a run.

For instance, you can emit a report when you lint a codebase:

```shell
biome lint --reporter=json-pretty .
```

For now, we support two report formats: `json` and `json-pretty`.

Note that the report format is subject to breaking changes, so you **should not** rely on it yet.
Please try this feature and let us know if any information is missing from the reports.


## Check _Git_ staged files

_Biome v1.5_ added the `--changed` flag on its main commands `biome format`, `biome lint`, and `biome check` to format and lint _Git_ tracked files that have been changed.

Today we are introducing a new flag `--staged` which allows you to check only files that have been added to the _Git index_ (_staged files_).
This is useful for checking that the files you want to commit are formatted and linted:

```shell
biome check --staged .
```

This is handy for writing your own [pre-commit script](/recipes/git-hooks/#shell-script).
Note that, unstaged changes on a staged file are **not** ignored.
Thus, we still recommend using a [dedicated pre-commit tool](/recipes/git-hooks/).

`--changed` and `--staged` are not available on the command `biome ci` because they don't make sense in a CI environment.


## Linter

### New nursery rules

Since _Biome v1.6_, we added several new rules.
New rules are incubated in the nursery group.
Nursery rules are subject to breaking changes.
The new rules are:

- [nursery/noDoneCallback](/linter/rules/no-done-callback/)
- [nursery/noDuplicateElseIf](/linter/rules/no-duplicate-else-if/)
- [nursery/noEvolvingAny](/linter/rules/no-evolving-any/)
- [nursery/noMisplacedAssertion](/linter/rules/no-misplaced-assertion/)

### Promoted rules

Once stable, a nursery rule is promoted to a stable group
The following rules are promoted:

- [complecity/noExcessiveNestedTestSuites](https://biomejs.dev/linter/rules/no-excessive-nested-test-suites)
- [complexity/noUselessTernary](/linter/rules/no-useless-ternary/)
- [correctness/useJsxKeyInIterable](/linter/rules/use-jsx-key-in-iterable/)
- [performance/noBarrelFile](/linter/rules/no-barrel-file/)
- [performance/noReExportAll](/linter/rules/no-re-export-all/)
- [style/noNamespaceImport](/linter/rules/no-namespace-import/)
- [style/useNodeAssertStrict](/linter/rules/use-node-assert-strict/)
- [suspicious/noDuplicateTestHooks](/linter/rules/no-duplicate-test-hooks/)
- [suspicious/noExportsInTest](/linter/rules/no-exports-in-test/)
- [suspicious/noFocusedTests](/linter/rules/no-focused-tests/)
- [suspicious/noSkippedTests](/linter/rules/no-skipped-tests/)
- [suspicious/noSuspiciousSemicolonInJsx](/linter/rules/no-suspicious-semicolon-in-jsx)


## Miscellaneous

- By default, _Biome_ searches a configuration file in the working directory and in parent directories if it doesn't exist.
  _Biome_ provides a _CLI_ option `--config-path` and an environment variable `BIOME_CONFIG_PATH` that allows which can be used to override this behavior.
  Previously, the option and the environment variable required a directory containing a _Biome_ configuration file.
  For example, the following command uses the _Biome_ configuration file located in `./config/`.

  ```shell
  biome format --config-path=./config/ ./src
  ```

  This was confusing for many users who are used to specifying the configuration file path directly.
  The option and environment variable now accept a file, so the following command is valid:

  ```shell
  biome format --config-path=./config/biome.json ./src
  ```

- You can now ignore `React` imports in the rules [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/#options) and [useImportType](https://biomejs.dev/linter/rules/use-import-type/#options) by setting [`javascript.jsxRuntime`](https://biomejs.dev/reference/configuration/#javascriptjsxruntime) to `reactClassic`.

- Biome applies specific settings to [well-known files](https://biomejs.dev/guides/how-biome-works/#well-known-files).
  It now recognizes more files and distinguishes between _JSON_ files that only allow comments and _JSON_ files that allow both comments and trailing commas.

- In the _React_ ecosystem, files ending in `.js` are allowed to contain _JSX_ syntax.
  The _Biome_ extension is now able to parse _JSX_ syntax in files that are associated with the _JavaScript_ language identifier.

- [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now supports _Preact_.

See the [changelog](/internals/changelog/#170-2024-04-15) for more details.


## What’s Next?

We have started work on the _CSS_ formatter and linter.
Some of our contributors have also started preliminary work on support for [_GraphQL_](https://github.com/biomejs/biome/issues/1927) and [_YAML_](https://github.com/biomejs/biome/issues/2365).
Any help is welcome!

If _Biome_ is valuable to you or your company, consider making a monthly donation to our [Open Collective](https://opencollective.com/biome).
You can also [sponsor us on GitHub](https://github.com/sponsors/biomejs).
This is important for the sustainability of the project.

Follow us on [our Twitter](https://twitter.com/home) and join [our Discord community](https://discord.gg/BypW39g6Yc).
