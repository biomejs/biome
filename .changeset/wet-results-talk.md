---
"@biomejs/biome": minor
---

Added new capabilities to the CLI arguments `--skip` and `--only`, available to the `biome lint` command.

`--skip` and `--only` can now accept domain names; when provided, Biome will run or skip all the rules that belong to a certain domain.

For example, the following command will only run the rules that belong to the [next](https://biomejs.dev/linter/domains/#next) domain:

```shell
biome lint --only=next
```

Another example, the following command will skip the rules that belong to the [project](https://biomejs.dev/linter/domains/#project) domain:

```shell
biome lint --skip=project
```
