---
"@biomejs/biome": minor
---

Added support for multiple reporters, and the ability to save reporters on arbitrary files.

#### Combine two reporters in CI

If you run Biome on GitHub, take advantage of the reporter and still see the errors in console, you can now use both reporters:

```shell
biome ci --reporter=default --reporter=github
```

#### Save reporter output to a file

With the new `--reporter-file` CLI option, it's now possible to save the output of all reporters to a file. The file is a path,
so you can pass a relative or an absolute path:

```shell
biome ci --reporter=rdjson --reporter-file=/etc/tmp/report.json
biome ci --reporter=summary --reporter-file=./reports/file.txt
```

You can combine these two features. For example, have the `default` reporter written on terminal, and the `rdjson` reporter written on file:

```shell
biome ci --reporter=default --reporter=rdjson --reporter-file=/etc/tmp/report.json
```

**The `--reporter` and `--reporter-file` flags must appear next to each other, otherwise an error is thrown.**
