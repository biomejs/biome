---
"@biomejs/biome": minor
---

Added the new CLI option `--profile-type-inference` to the commands `lint` and `check`.

The profiler is designed to track the execution time of internal queries and operations, group them by file, and sort them from the most expensive to the least expensive.

```shell
biome lint --profile-type-inference
```

```text
profiler ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  i Type inference profile (Biome <version>)

  src/index.ts

    Requests (top 5 of 12 groups; 28 source records; 46 executions)
      1. Promise classification <- nursery/noFloatingPromises
      ...

    Queries (top 8 of 25 groups; 61 source records; 94 executions)
      1. Promises / infer_expression_is_promise
      ...

    Whole-module inference (top 1 of 1 group; 1 source record; 1 execution)
      1. Import depth limit
      ...

  68 files omitted.

  ℹ To show all the information, use the --verbose option. The output might be very verbose, so it's advised to analyze a single file.

  $ biome lint --profile-type-inference --verbose ./path/to/file.ts
```

Use `--verbose` to include detailed timings, attribution, breadth, and implementation references for the top source records in each file.
