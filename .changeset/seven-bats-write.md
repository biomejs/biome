---
"@biomejs/biome": patch
---

Added the new nursery rule `noDuplicateDependencies`, which detects if there isn't a dependency listed twice.
In the dependencies, devDependencies, optionalDependencies & peerDependencies.

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
