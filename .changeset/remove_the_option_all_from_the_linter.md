---
"@biomejs/biome": major
---

Remove the option `all` from the linter.

The options `linter.rules.all` and `linter.rules.<group>.all` has been removed.

The number of rules in Biome have increased in scope and use cases, and sometimes some of them can conflict with each other.

The option was useful at the beginning, but now it's deemed harmful, because it can unexpected behaviours in users projects.

To automatically remove it, run the following command:
```shell
biome migrate --write
```
