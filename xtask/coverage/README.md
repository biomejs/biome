# coverage


## `cargo coverage`
Runs the parser against the various test suites to validate its conformance.

```
Run coverage command.
USAGE:
    cargo coverage <SUBCOMMAND> [option]
SUBCOMMANDS:
    compare             Compares output between two --json outputs
OPTIONS
    --markdown          Emits supported output into markdown format. Supported by `compare` subcommand.
    --json              Prints the test results in JSON. This mode will send all other test output and user messages to stderr.
    --detailed=[debug]  Prints a detailed summary at the end for all failing tests. Includes in depth details if set to `debug`.
    --suites=<IDS>      Runs the specified tests suites. Use comma as separator.
                        Valid values are:
                            *: will run all suites
                            js: will run all javascript suites; Same as "js/262";
                            ts: will run all typescript suites; Same as "ts/microsoft,ts/babel";
                            jsx: will run all jsx suites; Same as "jsx/babel";
                            yaml: will run all YAML suites; Same as "yaml/yaml-test-suite";
                            js/262: will run https://github.com/tc39/test262/tree/main/test;
                            ts/microsoft: will run https://github.com/microsoft/Typescript/tree/main/tests/cases
                            ts/babel: will run https://github.com/babel/babel/tree/main/packages/babel-parser/test/fixtures/typescript
                            jsx/babel: will run https://github.com/babel/babel/tree/main/packages/babel-parser/test/fixtures/jsx/basic
                            yaml/yaml-test-suite: will run YAML Test Suite parser acceptance tests (https://github.com/yaml/yaml-test-suite)
                        Default is "*".
    --filter=<file>     Filters out tests that don't match the query.
    --help              Prints this help.
```

## `cargo coverage compare`

Useful to compare the coverage, for example between your feature branch and `main`.

```bash
# (commit your code on pr branch, run)
git checkout main
cargo coverage --json > base_results.json
git checkout <your branch>
cargo coverage --json > new_results.json
cargo coverage compare ./base_results.json ./new_results.json --markdown
```
