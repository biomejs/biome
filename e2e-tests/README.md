# End-to-end tests

`e2e-tests` directory allows testing the Biome CLI directly.
Each directory in `e2e-tests` is a test that represents a project using Biome.
Every directory must include a shell script named `test.sh`.
The exit status of the script determines if the test passed or failed.

For example, the following script executes `biome lint`:

```sh
# fail if any command fail or if some variables are undefined
set -eu

cargo run --bin biome -- lint src

```

If the command reports lint error, then the script fails and makes the test fails.
The working directory is always set to the directory that holds `test.sh`.
