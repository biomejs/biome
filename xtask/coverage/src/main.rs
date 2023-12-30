use pico_args::Arguments;
use xtask::{project_root, pushd, Result};

use xtask_coverage::{compare::coverage_compare, run, SummaryDetailLevel};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let sub_command = args.subcommand()?;

    if sub_command.as_deref() == Some("compare") {
        // on pr branch, run
        // git checkout main
        // cargo coverage js --json > base_results.json
        // git checkout -
        // cargo coverage js --json > new_results.json
        // cargo coverage compare ./base_results.json ./new_results.json --markdown
        let markdown = args.contains("--markdown");
        let free = args.finish();
        let base_result_path = free.first().and_then(|arg| arg.to_str());
        let new_result_path = free.get(1).and_then(|arg| arg.to_str());
        coverage_compare(base_result_path, new_result_path, markdown);
        return Ok(());
    }

    if args.contains("--help") {
        eprintln!(
            "\
cargo coverage
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
                            js: will run all javascript suites; Same as \"js/262\";
                            ts: will run all typescript suites; Same as \"ts/microsoft,ts/babel\";
                            jsx: will run all jsx suites; Same as \"jsx/babel\";
                            js/262: will run https://github.com/tc39/test262/tree/main/test;
                            ts/microsoft: will run https://github.com/microsoft/Typescript/tree/main/tests/cases
                            ts/babel: will run https://github.com/babel/babel/tree/main/packages/babel-parser/test/fixtures/typescript
                            jsx/babel: will run https://github.com/babel/babel/tree/main/packages/babel-parser/test/fixtures/jsx/basic
                        Default is \"*\".
    --filter=<file>     Filters out tests that don't match the query.
    --help              Prints this help.
			"
        );
        return Ok(());
    }

    let json = args.contains("--json");
    let suites: Option<String> = args.opt_value_from_str("--suites").unwrap();
    let filter: Option<String> = args.opt_value_from_str("--filter").unwrap();

    let detail_level: Option<SummaryDetailLevel> =
        args.opt_value_from_str("--detailed").unwrap_or_else(|_| {
            if args.contains("--detailed") {
                Some(SummaryDetailLevel::Failing)
            } else {
                Some(SummaryDetailLevel::Coverage)
            }
        });

    args.finish();

    run(
        suites.as_deref(),
        filter.as_deref(),
        json,
        detail_level.unwrap_or(SummaryDetailLevel::Coverage),
    );

    Ok(())
}
