# Prettier Test Suite

These test snapshots were extracted from the
[prettier/prettier](https://github.com/prettier/prettier) repository

# Usage

These tests are run as part of the `biome_css_formatter` test suite but can be
explicitly called with `cargo test -p biome_css_formatter --test prettier_tests`

Setting the `REPORT_PRETTIER=1` environment variable when running these tests
will emit a `report.md` file containing an exhaustive difference between the
output of `biome_css_formatter` and Prettier's own snapshots.
Setting the `REPORT_TYPE=json | markdown` environment variable will dictate the format of the report.
Setting the environment variable `INCOMPATIBLE_ONLY=1` will emit an `incompatible_report.md`
file containing only incompatible test cases.


# Updating

Prettier is using Jest to run snapshot tests, and a node.js script is needed to
extract these snapshots into plain files for use in the Rust tests. To update
the snapshots:

1. Clone the Prettier git repository locally
2. Remove all the directories inside
   `crates/biome_css_formatter/tests/specs/prettier` to ensure all obsolete tests are
   removed
3. Go to `crates/biome_formatter_test/src/prettier` directory
4. Install prettier ``pnpm install``
5. Go to `crates/biome_css_formatter/tests/specs/prettier` directory
6. Run `node prepare_tests.js <prettier root directory>`
