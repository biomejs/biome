---
"@biomejs/biome": minor
---

Added a option to the `lint` command called `--suppress`. The new option suppresses a violation instead of applying a rule fix. The option accepts a string that is used as *reason* of the suppression comment.

When running the following command, it will add the suppression comment:

```shell
biome lint --write --suppress="Migration to Biome"
```

```js
debugger;
foo == bar;
```

```diff
+ // biome-ignore lint/suspicious/noDebugger: Migration to Biome
debugger;
+ // biome-ignore lint/suspicious/noDoubleEquals: Migration to Biome
foo == bar;
```

