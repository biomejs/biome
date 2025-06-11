---
"@biomejs/biome": minor
---

Introduced more advanced logging capabilities:

Every Biome CLI command can now be passed a `--log-file=<path>` argument, which
will write all log messages for that invocation to the given path instead of
`stdout`.

In addition, the `--log-level` parameter now also accepts a `tracing` value.
When `--log-level=tracing` is used, Biome also prints timing information from
tracing spans to the log.

Combined with Biome's ability to print logs in JSON format, and the `jq` command
line utility, this allows you to perform advanced analysis on Biome's internal
performance.

For example, if you want to figure out which paths take the longest when
building the module graph, you can use the following commands:

```sh
biome lint --log-level=tracing --log-kind=json --log-file=tracing.json
cat tracing.json | jq '. | select(.span.name == "update_module_graph") | { path: .span.path, time_busy: .["time.busy"], time_idle: .["time.idle"] }' > filtered.json
```

Now you will have a file called `filtered.json` with all the relevant timings,
together with the paths used during the invocations.
