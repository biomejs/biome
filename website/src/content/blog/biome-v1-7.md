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

Today we’re excited to announce the release of Biome v1.7!

This new version provides an easy path to migrate from ESLint and Prettier.
It also introduces experimental machine-readable reports for the formatter and the linter, new linter rules, and many fixes.

Update Biome using the following commands:

```
npm install --save-dev --save-exact @biomejs/biome@latest
npx @biomejs/biome migrate
```

## Migrate from ESLint with a single command

This release introduces a new subcommand `biome migrate eslint`.
This command will read your ESLint configuration and attempt to port their settings to Biome.

The subcommand is able to handle both the legacy and the flat configuration files.
It supports the `extends` field of the legacy configuration and loads both shared and plugin configurations!
The subcommand also migrates `.eslintignore`.

Given the following ESLint configuration:

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

And the following Biome configuration:

```json title="biome.json"
{
	"linter": {
		"enabled": true,
		"rules": {
			"recommended": true
		}
	}
}
```

Run `biome migrate eslint --write` to migrate your ESLint configuration to Biome.
The command overwrites your initial Biome configuration.
For example, it disables `recommended`.
This results in the following Biome configuration:

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

The subcommand needs Node.js to load and resolve all the plugins and `extends` configured in the ESLint configuration file.
For now, `biome migrate eslint` doesn't support configuration written in YAML.

We have a [dedicated page](/linter/rules-sources/) that lists the equivalent Biome rule of a given ESLint rule.
We handle some ESLint plugins such as [TypeScript ESLint](https://typescript-eslint.io/), [ESLint JSX A11y](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y), [ESLint React](https://github.com/jsx-eslint/eslint-plugin-react), and [ESLint Unicorn](https://github.com/sindresorhus/eslint-plugin-unicorn).
Some rules are equivalent to their ESLint counterparts, while others are inspired.
By default, Biome doesn't migrate inspired rules.
You can use the CLI flag `--include-inspired` to migrate them.

## Migrate from Prettier with a single command

[Biome v1.6 introduced the subcommand `biome migrate prettier`](/blog/biome-v1-6/#easier-migration-from-prettier).

In Biome v1.7, we add support of [Prettier's `overrides`](https://prettier.io/docs/en/configuration.html#configuration-overrides) and attempts to convert `.prettierignore` glob patterns to globs supported by Biome.

During the migration, Prettier's `overrides` is translated to [Biome's `overrides`](https://biomejs.dev/reference/configuration/#overrides).
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

Run `biome migrate prettier --write` to migrate your Prettier configuration to Biome.
This results in the following Biome configuration:

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

The subcommand needs Node.js to load JavaScript configurations such as `.prettierrc.js`.
`biome migrate prettier` doesn't support configuration written in JSON5, TOML, or YAML.


## Emit machine-readable reports

Biome is now able to output JSON reports detailing the diagnostics emitted by a command.

For instance, you can emit a report when you lint a codebase:

```shell
biome lint --reporter=json-pretty .
```

For now, we support two report formats: `json` and `json-pretty`.

Note that the report format is **experimental**, and it might change in the future.
Please try this feature and let us know if any information needs to be added to the reports.


## Check `git` staged files

Biome v1.5 added the `--changed` to format and lint `git` tracked files that have been changed.

Today we are introducing a new option `--staged` which allows you to check only files added to the _Git index_ (_staged files_).
This is useful for checking that the files you want to commit are formatted and linted:

```shell
biome check --staged .
```

This is handy for writing your own [pre-commit script](/recipes/git-hooks/#shell-script).
Note that unstaged changes on a staged file are **not** ignored.
Thus, we still recommend using a [dedicated pre-commit tool](/recipes/git-hooks/).

Thanks to [@castarco](https://github.com/castarco) for implementing this feature!


## Linter

### New nursery rules

Since _Biome v1.6_, we added several new rules.
New rules are incubated in the nursery group.
Nursery rules are exempt from semantic versioning.

The new rules are:

- [nursery/noConstantMathMinMaxClamp](/linter/rules/no-constant-math-min-max-clamp/)
- [nursery/noDoneCallback](/linter/rules/no-done-callback/)
- [nursery/noDuplicateElseIf](/linter/rules/no-duplicate-else-if/)
- [nursery/noEvolvingAny](/linter/rules/no-evolving-any/)
- [nursery/noFlatMapIdentity](/linter/rules/no-flat-map-identity/)
- [nursery/noMisplacedAssertion](/linter/rules/no-misplaced-assertion/)

### Promoted rules

Once stable, a nursery rule is promoted to a stable group.
The following rules are promoted:

- [complexity/noExcessiveNestedTestSuites](https://biomejs.dev/linter/rules/no-excessive-nested-test-suites)
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

- By default, Biome searches a configuration file in the working directory and parent directories if it doesn't exist.
  Biome provides a CLI option `--config-path` and an environment variable `BIOME_CONFIG_PATH` that allows which can be used to override this behavior.
  Previously, they required a directory containing a Biome configuration file.
  For example, the following command uses the Biome configuration file in `./config/`.

  ```shell
  biome format --config-path=./config/ ./src
  ```

  This wasn't very clear for many users who are used to specifying the configuration file path directly.
  They now accept a file, so the following command is valid:

  ```shell
  biome format --config-path=./config/biome.json ./src
  ```

- You can now ignore `React` imports in the rules [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/#options) and [useImportType](https://biomejs.dev/linter/rules/use-import-type/#options) by setting [`javascript.jsxRuntime`](https://biomejs.dev/reference/configuration/#javascriptjsxruntime) to `reactClassic`.

- Biome applies specific settings to [well-known files](https://biomejs.dev/guides/how-biome-works/#well-known-files).
  It now recognizes more files and distinguishes between JSON files that only allow comments and JSON files that allow both comments and trailing commas.

- In the React ecosystem, files ending in `.js` are allowed to contain JSX syntax.
  The Biome extension is now able to parse JSX syntax in files that are associated with the JavaScript language identifier.

- [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now supports Preact.

See the [changelog](/internals/changelog/#170-2024-04-15) for more details.


## What’s Next?

We have started work on the CSS formatter and linter.
Early implementation towards a [plugin system](https://github.com/biomejs/biome/discussions/2286) is also underway.
Some of our contributors have started preliminary work for [_GraphQL_](https://github.com/biomejs/biome/issues/1927) and [YAML](https://github.com/biomejs/biome/issues/2365).
Any help is welcome!

If Biome is valuable to you or your company, consider donating monthly to our [Open Collective](https://opencollective.com/biome).
You can also [sponsor us on GitHub](https://github.com/sponsors/biomejs).
This is important for the sustainability of the project.

Follow us on [our Twitter](https://twitter.com/home) and join [our Discord community](https://discord.gg/BypW39g6Yc).
