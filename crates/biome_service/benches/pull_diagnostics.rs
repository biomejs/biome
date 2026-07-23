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
use std::{str::FromStr, sync::Arc};

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

const NO_FLOATING_PROMISES_FILES: &[(&str, &str)] = &[(
    TARGET_PATH,
    r#"
async function returnsPromise(): Promise<void> {}

returnsPromise();
returnsPromise();
returnsPromise();
returnsPromise();
returnsPromise();
returnsPromise();
returnsPromise();
returnsPromise();
"#,
)];

const NO_MISUSED_PROMISES_FILES: &[(&str, &str)] = &[(
    TARGET_PATH,
    r#"
const promise = Promise.resolve(true);

if (promise) {}
if (promise) {}
const first = promise ? 1 : 0;
const second = promise ? 2 : 0;
while (promise) {
    break;
}
while (promise) {
    break;
}
do {} while (promise);
do {} while (promise);
"#,
)];

fn main() {
    divan::main();
}

#[divan::bench]
fn e2e_no_unused_variables(bencher: Bencher) {
    bench_pull_diagnostics(
        bencher,
        NO_UNUSED_VARIABLES_FILES,
        "lint/correctness/noUnusedVariables",
        ScanKind::NoScanner,
    );
}

#[divan::bench]
fn e2e_no_unresolved_imports(bencher: Bencher) {
    bench_pull_diagnostics(
        bencher,
        NO_UNRESOLVED_IMPORTS_FILES,
        "lint/correctness/noUnresolvedImports",
        ScanKind::Project,
    );
}

#[divan::bench]
fn e2e_no_floating_promises(bencher: Bencher) {
    bench_pull_diagnostics(
        bencher,
        NO_FLOATING_PROMISES_FILES,
        "lint/nursery/noFloatingPromises",
        ScanKind::TypeAware,
    );
}

#[divan::bench]
fn e2e_no_misused_promises(bencher: Bencher) {
    bench_pull_diagnostics(
        bencher,
        NO_MISUSED_PROMISES_FILES,
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
    files: &'static [(&'static str, &'static str)],
    rule: &'static str,
    scan_kind: ScanKind,
) {
    bencher
        .with_inputs(move || setup_benchmark(files, rule, scan_kind.clone_without_targeting_info()))
        .bench_local_refs(|benchmark| {
            black_box(benchmark.pull_diagnostics());
        });
}

fn setup_benchmark(
    files: &[(&str, &str)],
    rule: &str,
    scan_kind: ScanKind,
) -> PullDiagnosticsBenchmark {
    let fs = MemoryFileSystem::default();
    for (path, content) in files {
        fs.insert(Utf8PathBuf::from(path), content.as_bytes());
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
