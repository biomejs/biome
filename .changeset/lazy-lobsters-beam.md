---
"@biomejs/biome": patch
---

The default value of the `ignoreRestSiblings` option for [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables)'
has been reverted to its prior value of `true` after [an internal refactor](https://github.com/biomejs/biome/pull/7941) accidentally changed it.

The diagnostic message has also been tweaked for readability.
