---
"@biomejs/biome": patch
---

The documentation & rule sources for [`lint/complexity/noBannedTypes`](https://biomejs.dev/linter/rules/no-banned-types) have been updated.

Among other things, the rule now recommends `Record<keyof any, never>` instead of `Record<string, never>` (which incorrectly allows symbol-keyed properties), as well as mentioning an alternate method involving `unique symbol`-based guards used by some existing packages.

The rule's listed sources have been updated as well to reflect the original source rule (`ban-types`) having been [split into 3 separate rules](https://github.com/typescript-eslint/typescript-eslint/pull/8977) circa April 2024.