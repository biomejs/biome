use biome_analyze::RuleCategoriesBuilder;
use biome_configuration::{Configuration, analyzer::AnalyzerSelector};
use biome_diagnostics::Severity;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_service::{
    Workspace,
    settings::ModuleGraphResolutionKind,
    workspace::{
        FileContent, OpenFileParams, OpenProjectParams, PullDiagnosticsParams,
        PullDiagnosticsResult, ScanKind, ScanProjectParams, UpdateSettingsParams, server,
    },
};
use camino::Utf8PathBuf;
use divan::{Bencher, black_box};
use std::{fmt, str::FromStr, sync::Arc};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

const PROJECT_ROOT: &str = "/project";
const TARGET_PATH: &str = "/project/index.ts";
const IMPORTED_PROMISE_CHAIN_LENGTH: usize = 64;

type BenchmarkFiles = Vec<(Utf8PathBuf, Vec<u8>)>;

struct RuleBenchmarkCase {
    name: &'static str,
    files: fn() -> BenchmarkFiles,
}

impl fmt::Display for RuleBenchmarkCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name)
    }
}

const NO_UNUSED_VARIABLES_FILES: &[(&str, &str)] = &[(
    TARGET_PATH,
    r#"
const unusedOne = 1;
const unusedTwo = 2;
const unusedThree = 3;
const unusedFour = 4;
const unusedFive = 5;
const unusedSix = 6;
const unusedSeven = 7;
const unusedEight = 8;
"#,
)];

const NO_UNRESOLVED_IMPORTS_FILES: &[(&str, &str)] = &[
    (
        TARGET_PATH,
        r#"
import defaultExport, {
    existing,
    missingOne,
    missingTwo,
    missingThree,
} from "./dependency.ts";
import { missingFour, missingFive, missingSix } from "./dependency.ts";
import "./missing.ts";

console.log(
    defaultExport,
    existing,
    missingOne,
    missingTwo,
    missingThree,
    missingFour,
    missingFive,
    missingSix,
);
"#,
    ),
    (
        "/project/dependency.ts",
        r#"
export const existing = 1;
"#,
    ),
];

const NO_FLOATING_PROMISES_CASES: &[RuleBenchmarkCase] = &[
    RuleBenchmarkCase {
        name: "local_calls",
        files: no_floating_promises_local_files,
    },
    RuleBenchmarkCase {
        name: "array_results",
        files: no_floating_promises_array_files,
    },
    RuleBenchmarkCase {
        name: "imported_chain",
        files: no_floating_promises_imported_chain_files,
    },
];

const NO_MISUSED_PROMISES_CASES: &[RuleBenchmarkCase] = &[
    RuleBenchmarkCase {
        name: "conditionals_and_spreads",
        files: no_misused_promises_condition_files,
    },
    RuleBenchmarkCase {
        name: "callbacks",
        files: no_misused_promises_callback_files,
    },
    RuleBenchmarkCase {
        name: "imported_chain",
        files: no_misused_promises_imported_chain_files,
    },
];

fn main() {
    divan::main();
}

#[divan::bench]
fn e2e_no_unused_variables(bencher: Bencher) {
    bench_pull_diagnostics(
        bencher,
        || benchmark_files(NO_UNUSED_VARIABLES_FILES),
        "lint/correctness/noUnusedVariables",
        ScanKind::NoScanner,
    );
}

#[divan::bench]
fn e2e_no_unresolved_imports(bencher: Bencher) {
    bench_pull_diagnostics(
        bencher,
        || benchmark_files(NO_UNRESOLVED_IMPORTS_FILES),
        "lint/correctness/noUnresolvedImports",
        ScanKind::Project,
    );
}

#[divan::bench(args = NO_FLOATING_PROMISES_CASES)]
fn e2e_no_floating_promises(bencher: Bencher, case: &RuleBenchmarkCase) {
    bench_pull_diagnostics(
        bencher,
        case.files,
        "lint/nursery/noFloatingPromises",
        ScanKind::TypeAware,
    );
}

#[divan::bench(args = NO_MISUSED_PROMISES_CASES)]
fn e2e_no_misused_promises(bencher: Bencher, case: &RuleBenchmarkCase) {
    bench_pull_diagnostics(
        bencher,
        case.files,
        "lint/nursery/noMisusedPromises",
        ScanKind::TypeAware,
    );
}

struct PullDiagnosticsBenchmark {
    workspace: Box<dyn Workspace>,
    params: PullDiagnosticsParams,
}

impl PullDiagnosticsBenchmark {
    fn pull_diagnostics(&self) -> PullDiagnosticsResult {
        self.workspace
            .pull_diagnostics(self.params.clone())
            .expect("diagnostics should be pulled")
    }
}

fn bench_pull_diagnostics(
    bencher: Bencher,
    files: fn() -> BenchmarkFiles,
    rule: &'static str,
    scan_kind: ScanKind,
) {
    bencher
        .with_inputs(move || {
            setup_benchmark(files(), rule, scan_kind.clone_without_targeting_info())
        })
        .bench_local_refs(|benchmark| {
            black_box(benchmark.pull_diagnostics());
        });
}

fn setup_benchmark(
    files: BenchmarkFiles,
    rule: &str,
    scan_kind: ScanKind,
) -> PullDiagnosticsBenchmark {
    let fs = MemoryFileSystem::default();
    for (path, content) in files {
        fs.insert(path, content);
    }

    let workspace = server(Arc::new(fs), None);
    let project_key = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new(PROJECT_ROOT),
            open_uninitialized: true,
        })
        .expect("project should be opened")
        .project_key;

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration::default(),
            workspace_directory: Some(BiomePath::new(PROJECT_ROOT)),
            extended_configurations: Vec::new(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::from(&scan_kind),
        })
        .expect("settings should be updated");

    if !scan_kind.is_none() {
        workspace
            .scan_project(ScanProjectParams {
                project_key,
                watch: false,
                force: false,
                scan_kind,
                verbose: false,
            })
            .expect("project should be scanned");
    }

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(TARGET_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .expect("target file should be opened");

    let selector = AnalyzerSelector::from_str(rule).expect("rule selector should be valid");
    PullDiagnosticsBenchmark {
        workspace,
        params: PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(TARGET_PATH),
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            only: vec![selector],
            skip: Vec::new(),
            enabled_rules: Vec::new(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        },
    }
}

fn benchmark_files(files: &[(&str, &str)]) -> BenchmarkFiles {
    files
        .iter()
        .map(|(path, content)| (Utf8PathBuf::from(*path), content.as_bytes().to_vec()))
        .collect()
}

fn no_floating_promises_local_files() -> BenchmarkFiles {
    benchmark_files(&[(
        TARGET_PATH,
        include_str!("fixtures/no_floating_promises/local_calls.ts"),
    )])
}

fn no_floating_promises_array_files() -> BenchmarkFiles {
    benchmark_files(&[(
        TARGET_PATH,
        include_str!("fixtures/no_floating_promises/array_results.ts"),
    )])
}

fn no_floating_promises_imported_chain_files() -> BenchmarkFiles {
    imported_promise_chain_files(false)
}

fn no_misused_promises_condition_files() -> BenchmarkFiles {
    benchmark_files(&[(
        TARGET_PATH,
        include_str!("fixtures/no_misused_promises/conditionals_and_spreads.ts"),
    )])
}

fn no_misused_promises_callback_files() -> BenchmarkFiles {
    benchmark_files(&[(
        TARGET_PATH,
        include_str!("fixtures/no_misused_promises/callbacks.ts"),
    )])
}

fn no_misused_promises_imported_chain_files() -> BenchmarkFiles {
    imported_promise_chain_files(true)
}

fn imported_promise_chain_files(as_callbacks: bool) -> BenchmarkFiles {
    let mut files = Vec::with_capacity(IMPORTED_PROMISE_CHAIN_LENGTH + 1);
    files.push((
        Utf8PathBuf::from("/project/load-000.ts"),
        b"export async function load0(value: number) { return value; }".to_vec(),
    ));

    for index in 1..IMPORTED_PROMISE_CHAIN_LENGTH {
        let previous = index - 1;
        files.push((
            Utf8PathBuf::from(format!("/project/load-{index:03}.ts")),
            format!(
                "import {{ load{previous} }} from \"./load-{previous:03}.ts\";\n\
                 export function load{index}(value: number) {{ return load{previous}(value); }}"
            )
            .into_bytes(),
        ));
    }

    let mut target = String::new();
    for index in 0..IMPORTED_PROMISE_CHAIN_LENGTH {
        target.push_str(&format!(
            "import {{ load{index} }} from \"./load-{index:03}.ts\";\n"
        ));
    }
    if as_callbacks {
        target.push_str("const values: number[] = [1, 2, 3];\n");
        for index in 0..IMPORTED_PROMISE_CHAIN_LENGTH {
            target.push_str(&format!("values.forEach(value => load{index}(value));\n"));
        }
    } else {
        for index in 0..IMPORTED_PROMISE_CHAIN_LENGTH {
            target.push_str(&format!("load{index}({index});\n"));
        }
    }
    files.push((Utf8PathBuf::from(TARGET_PATH), target.into_bytes()));
    files
}
