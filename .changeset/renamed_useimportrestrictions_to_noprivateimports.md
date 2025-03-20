---
"@biomejs/biome": major
---

The rule `useImportRestrictions` has been renamed to [`noPrivateImports`](https://biomejs.dev/linter/rules/no-private-imports), and its
functionality has been significantly upgraded.

Previously, the rule would assume that any direct imports from modules inside
other directories should be forbidden due to their _package private_ visibility.

The updated rule allows configuring the default visibility of exports, and
recognises JSDoc comments to override this visibility. The default visibility
is now `**public**`, but can be set to `**package**`, or even `**private**`.
Refer to the [documentation of the rule](https://biomejs.dev/linter/rules/no-private-imports) to understand how to leverage the JSDoc comments.

`noPrivateImports` is now recommended by default.
