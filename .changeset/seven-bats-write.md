---
"@biomejs/biome": patch
---

Added the new nursery rule [`noDuplicateDependencies`](https://next.biomejs.dev/linter/rules/no-duplicate-dependencies/), which verifies that no dependencies are duplicated between the `bundledDependencies`, `bundleDependencies`, `dependencies`, `devDependencies`, `overrides`, `optionalDependencies`, and `peerDependencies` sections.

For example, the following snippets will trigger the rule:

```json
{
  "dependencies": {
    "foo": ""
  },
  "devDependencies": {
    "foo": ""
  }
}
```

```json
{
  "dependencies": {
    "foo": ""
  },
  "optionalDependencies": {
    "foo": ""
  }
}
```

```json
{
  "dependencies": {
    "foo": ""
  },
  "peerDependencies": {
    "foo": ""
  }
}
```
