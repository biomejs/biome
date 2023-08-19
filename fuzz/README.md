# rome-fuzz

Fuzzers and associated utilities for automatic testing of Biome.

## Usage

To use the fuzzers provided in this directory, start by invoking:

```bash
./fuzz/init-fuzzers.sh
```

This will install [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) and optionally download
datasets which improve the efficacy of the testing.
**This step is necessary for initialising the corpus directory, as all fuzzers share a common
corpus.**
The dataset may take several hours to download and clean, so if you're just looking to try out the
fuzzers, skip the dataset download, though be warned that some features simply cannot be tested
without it (very unlikely for the fuzzer to generate valid python code from "thin air").

Once you have initialised the fuzzers, you can then execute any fuzzer with:

```bash
cargo fuzz run --strip-dead-code -s none name_of_fuzzer -- -timeout=1
```

**Users using Apple M1 devices must use a nightly compiler and omit the `-s none` portion of this
command, as this architecture does not support fuzzing without a sanitizer.**
You can view the names of the available fuzzers with `cargo fuzz list`.
For specific details about how each fuzzer works, please read this document in its entirety.

**IMPORTANT: You should run `./reinit-fuzzer.sh` after adding more file-based testcases.** This will
allow the testing of new features that you've added unit tests for.

### Debugging a crash

Once you've found a crash, you'll need to debug it.
The easiest first step in this process is to minimise the input such that the crash is still
triggered with a smaller input.
`cargo-fuzz` supports this out of the box with:

```bash
cargo fuzz tmin --strip-dead-code -s none name_of_fuzzer artifacts/name_of_fuzzer/crash-...
```

From here, you will need to analyse the input and potentially the behaviour of the program.
The debugging process from here is unfortunately less well-defined, so you will need to apply some
expertise here.
Happy hunting!

## A brief introduction to fuzzers

Fuzzing, or fuzz testing, is the process of providing generated data to a program under test.
The most common variety of fuzzers are mutational fuzzers; given a set of existing inputs (a
"corpus"), it will attempt to slightly change (or "mutate") these inputs into new inputs that cover
parts of the code that haven't yet been observed.
Using this strategy, we can quite efficiently generate testcases which cover significant portions of
the program, both with expected and unexpected data.
[This is really quite effective for finding bugs.](https://github.com/rust-fuzz/trophy-case)

The fuzzers here use [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), a utility which allows
Rust to integrate with [libFuzzer](https://llvm.org/docs/LibFuzzer.html), the fuzzer library built
into LLVM.
Each source file present in [`fuzz_targets`](fuzz_targets) is a harness, which is, in effect, a unit
test which can handle different inputs.
When an input is provided to a harness, the harness processes this data and libFuzzer observes the
code coverage and any special values used in comparisons over the course of the run.
Special values are preserved for future mutations and inputs which cover new regions of code are
added to the corpus.

## Each fuzzer harness in detail

Each fuzzer harness is designed to test different aspects of Biome.
Since Biome's primary function is parsing, formatting, and linting, we can use fuzzing not only to
detect crashes or panics, but also to detect violations of guarantees of the crate.
This concept is used extensively throughout the fuzzers.

### `rome_parse_*`

Each of the `rome_parse_*` fuzz harnesses utilise the [round-trip
property](https://blog.ssanj.net/posts/2016-06-26-property-based-testing-patterns.html) of parsing
and unparsing; that is, given a particular input, if we parse some code successfully, we expect the
unparsed code to have the content as the original code.
If they do not match, then some details of the original input were not captured on the first parse.
The corpus for the JS-like parsers is based on unit tests and [a JS dataset for machine learning
training](https://www.sri.inf.ethz.ch/js150).

Errata for specific fuzzers can be seen below.

#### `rome_parse_json`

Since JSON formats are distinct from JS source code and are a relatively simple format, it is not
strictly necessary to use the shared corpus.
[Fuzzbench](https://google.github.io/fuzzbench/) results consistently show that JSON parsers tend to
max out their coverage with minimal or no corpora.

At time of writing (June 11, 2023), JSONC does not seem to be supported, so it is not fuzzed.

#### `rome_parse_all`

This fuzz harness merely merges all the JS parsers together to create a shared corpus.
It can be used in place of the parsers for d_ts, jsx, module, script, tsx, and typescript in
continuous integration.

### `rome_format_*`

These fuzzers use the same corpora as the fuzzers previously mentioned, but check the correctness of
the formatters as well.
We assume the following qualities of formatters:
 - Formatters will not introduce syntax errors into the program
 - Formatting code twice will have the same result as formatting code once

In this way, we verify the [idempotency](https://en.wikipedia.org/wiki/Idempotence) and syntax
preservation property of formatting.

Of particular note: these fuzzers may have false negative results if e.g. two tokens are turned into
one token and the reformatting result is the same.
Unfortunately, we can't necessarily control for this because the formatter may reorganise the
sequence of tokens.

## Errata

Unfortunately, `--strip-dead-code` is necessary to build the target with a suitable amount of
memory.
This seems to be caused by some issue in LLVM, but I haven't been able to spend the time to
investigate this fully yet.
