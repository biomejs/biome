---
"@biomejs/biome": minor
---

Added the new CLI option called `--threads` to the `ci` command. It allows to control the numbers of threads that can be used when using the Biome CLI.

It's possible to use the environment variable `BIOME_THREADS` as an alternatives.

This feature is useful when running the CLI in environments that have limited resources, for example CI/CD.

```shell
biome ci --threads=1
BIOME_THREADS=1 biome ci
```
