use pico_args::Arguments;
use xtask::{project_root, pushd, Result};
use xtask_bench::{run, FeatureToBenchmark, RunArgs};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;

#[cfg(all(target_os = "windows", not(feature = "dhat-heap")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
    not(feature = "dhat-heap")
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn main() -> Result<(), pico_args::Error> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let _d = pushd(project_root());
    let mut args = Arguments::from_env();

    if args.contains("--help") {
        eprintln!(
            "\
cargo bench
Benchmark parser and formatter.
USAGE:
    cargo bench [option]
OPTIONS
    --save-baseline     Allows different runs to be compared.
    --feature           Possible values: parser, formatter
    --criterion=[true/false]  Run a series of statistical test to assess with the this run is faster or slower than previous runs.
    --suites=<IDS>      Runs the specified benchmarks. Use comma as separator.
                        Valid values are:
                            *: will run all benchmarks;
                            js: will benchmark all javascript libraries;
                            ts: will benchmark all typescript libraries;
                        Default is \"*\".
    --filter=<file>     Filters out tests that don't match the query.
    --help              Prints this help.
			"
        );
        return Ok(());
    }

    // on pr branch, run
    // git checkout main
    // cargo benchmark --save-baseline main
    // git checkout -
    // cargo benchmark --save-baseline pr
    // critcmp main pr # (cargo install critcmp)
    let filter: String = args
        .opt_value_from_str("--filter")
        .unwrap()
        .unwrap_or_else(|| ".*".to_string());
    let criterion: bool = args
        .opt_value_from_str("--criterion")
        .unwrap()
        .unwrap_or(true);
    let suites = args
        .opt_value_from_str("--suites")
        .unwrap()
        .unwrap_or_else(|| "*".to_string());

    let baseline: Option<String> = args.opt_value_from_str("--save-baseline").unwrap();
    // "feature" is a mandatory option and will throw an error if it's missing or incorrect
    let feature: FeatureToBenchmark = args.value_from_str("--feature")?;

    run(RunArgs {
        filter,
        criterion,
        baseline,
        feature,
        suites,
    });

    Ok(())
}
