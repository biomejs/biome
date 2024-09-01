# Benchmarks

## Running in Docker

Run in the root directory, not `./benchmark`

1. `docker build --rm -f Dockerfile.benchmark . --progress plain`

## Running locally

1. Install hyperfine: `cargo install hyperfine`
2. Install node modules: `pnpm i`
3. Run one of benchmark suites:
    - the benchmarks of the formatters: `node bench.js formatter`
    - the benchmarks of the linter: `node bench.js linter`


## Results

Setup: MacBook Pro (13-inch, M1, 2020)

### Formatting

* Biome's ~25 times faster than Prettier
* Biome's ~20 times faster than parallel-prettier
* Biome's ~20 times faster than `xargs -P`[^1]
* Biome's 1.5 to 2 times faster than `dprint`
* Biome single-threaded is ~7 times faster than Prettier.

The speed-ups for the multithreaded benchmarks can vary significantly depending on the setup.
For example, Biome is 100 times faster than Prettier on an M1 Max with 10 cores.

[^1]: Run `time find lib/ examples declarations benchmark -name '*.js' -print0 | xargs -P8 -0 -n 200 npx prettier --write --loglevel=error` in the `target/webpack` directory. I manually tinkered with the `-n` parameter to get the fastest run.

### Linting

* Biome's ~15x times faster than ESLint (without any plugins)
* Biome single-threaded is ~4 times faster than ESLint.

* Biome's linter is fast, but there is room for improvements
* Biome's linter spends significant time building the semantic model, the control flow graph, and matching queries. I'm convinced there's room for improvement ([3565](https://github.com/rome/tools/pull/3565), [3569](https://github.com/rome/tools/pull/3569)).
* Computing the diff for code fixes is expensive. Biome can spend up to 3s computing diffs (not measured by these benchmarks, see explanations below)


## Notes

We've been careful to create fair benchmarks. This section explains some of the decisions behind the benchmark setup and how these decisions affect the results. Please [let us know](https://github.com/rome/tools/issues) if you have ideas on how to make the benchmarks fairer or if there's a mistake with our setup.

### Formatting

* Compares the wall time of Biome, Prettier, and dprint to format all files in a project where all files are correctly formatted. To ensure that files are correctly formatted we run twice a toll (warm-up) before benchmarking it.
* dprint and Prettier support incremental formatting to only format changed files, whereas Biome does not. This benchmark does not measure incremental formatting as it measures cold formatting time. You may see significant speedups on subsequent formatting runs when enabling incremental formatting.
* The benchmark limits Prettier to only format JavaScript and TypeScript files because Biome doesn't support other file types.
* Biome only prints a summary with the number of formatted files. The prettier benchmark uses `--log-level=error` for a fairer benchmark so that Prettier doesn't print every filename.

### Linting

* Compares the wall time of Biome and ESLint to check all files in a project.
* Only enables rules that both Biome and ESLint support.
* The comparison includes ESLint without plugin and ESLint with the TypeScript ESLint plugin.
  To have a fair comparison we don't enable any rules that require type information because this is known to be very slow and Biome has not such capabilities yet.
* Biome prints rich diffs for each lint diagnostic, whereas ESLint only shows the name and description of the lint rule. That's why the benchmark uses `--max-diagnostics=0` when running Biome because Biome then only counts diagnostics without generating the diffs. Overall, this results in a more accurate comparison but has the downside that Biome only prints the diagnostic count, whereas ESLint prints a line for each lint error.
