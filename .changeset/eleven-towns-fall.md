---
"@biomejs/biome": minor
---

Added the new command `biome inspect config`, a convenient command to understand the resolved value of each option.

This is particularly useful when you extend the configuration from multiple sources, and you want to know the effective
value of some options that are applied to your project/library.

Running the command without

```shell
biome inspect config
```

The command accepts a positional argument, which represents the specific key that you want to inspect. If you want to know the `lineWidth`
applied to your JavaScript files, you would write the following command:

```shell
biome inspect config javascript.formatter.lineWidth
```

The command accepts also a `--path` argument, useful if you have multiple overrides, and you want to know which override applies to
your use case.

```shell
biome inspect config avascript.formatter.lineWidth --path ./src/components/Footer.tsx
```
