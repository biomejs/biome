---
title: Biome v1.7
description: Migrate from Prettier and ESLint with one commad!
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

## Migrate from ESLint with a single command

This release introduces a new command called `biome migrate eslint`.
This command will read your ESLint configurations and attempt to port their settings to Biome.

The command is able to handle both the legacy and the flat ESLint configurations.
It supports the `extends` field of the legacy configuration and loads both shared and plugin configurations!
The command also attempts to migrate `.eslintignore`.

Given the following ESLint configuration:

```json
{
  "extends": ["plugin:unicorn/recommended"],
  "plugins": ["unicorn"],
  "ignore_patterns": ["dist//**"],
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

And the following Biome configuration (obtained by running `biome init`):

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
This results in the following Biome configuration.
Note that this overrides your initial Biome configuration.

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
For now, `biome migrate eslint` doesn't support configuration written in YAML.


## Migrate from Prettier with a single command

[Biome v1.6 introduced the command `biome migrate prettier`](https://biomejs.dev/blog/biome-v1-6/#easier-migration-from-prettier).

This new version adds the support of the `overrides` field and attempts to convert `.prettierignore` glob patterns to globs supported by Biome.


## Linter

### New rules

This release comes with several new rules.
New rules are incubated in the nursery group.
Nursery rules are subject to breaking changes.

- [nursery/noDoneCallback](https://biomejs.dev/linter/rules/no-done-callback/)
- [nursery/noDuplicateElseIf](https://biomejs.dev/linter/rules/no-duplicate-else-if/)
- [nursery/noEvolvingAny](https://biomejs.dev/linter/rules/no-evolving-any/)
- [nursery/noMisplacedAssertion](https://biomejs.dev/linter/rules/no-misplaced-assertion/)


## Miscellaneous

- Biome is able to apply specific settings to [well-known files](https://biomejs.dev/guides/how-biome-works/#well-known-files).
  It now recognized more files and is able to differentiate JSON files that allows only comments and JSON files that allows both comments and trailing commas.
- Biome now displays the location of a parsing error for its configuration file.
- Biome extension is now able to parse the JSX syntax in files that associated with the javaScript language identifier.
  In React ecosystem, `.js` files are allowed to include JSX syntax.
- You can now ignore `React` imports in the rule [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/#options).
- [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now supports _Preact_.

