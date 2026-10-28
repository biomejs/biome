use biome_configuration::{Configuration, JsConfiguration};
use biome_formatter::rowan::Printed;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_service::{
    Workspace,
    settings::ModuleGraphResolutionKind,
    workspace::{
        FileContent, FormatFileParams, OpenFileParams, OpenProjectParams, ProjectKey,
        UpdateSettingsParams, server,
    },
};
use divan::{Bencher, black_box};
use std::{fmt, sync::Arc};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(all(target_env = "musl", target_arch = "aarch64")),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

const PROJECT_ROOT: &str = "/project";
const TARGET_PATH: &str = "/project/index.js";
const FIXTURE_REPETITIONS: usize = 32;

struct EmbeddedBenchmarkCase {
    name: &'static str,
    source: &'static str,
}

impl fmt::Display for EmbeddedBenchmarkCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name)
    }
}

/// The styled templates of both cases have similar rules, but the ones of
/// `interpolations` replace values, selectors, and rules with interpolations,
/// including a nested template.
const CSS_IN_JS_CASES: &[EmbeddedBenchmarkCase] = &[
    EmbeddedBenchmarkCase {
        name: "plain",
        source: include_str!("fixtures/css_in_js/plain.js"),
    },
    EmbeddedBenchmarkCase {
        name: "interpolations",
        source: include_str!("fixtures/css_in_js/interpolations.js"),
    },
];

fn main() {
    // Without the setting or with broken detection, the templates are kept as
    // is and the benchmarks would silently measure only the JavaScript work.
    for case in CSS_IN_JS_CASES {
        let (workspace, project_key) = setup_workspace(case.source);
        open_file(workspace.as_ref(), project_key);
        let formatted = format_file(workspace.as_ref(), project_key);
        assert!(
            formatted.as_code().contains("display: inline-flex;"),
            "the templates of {case} should be formatted as CSS"
        );
    }

    divan::main();
}

/// Measures parsing a JavaScript file and the CSS-in-JS templates inside it.
#[divan::bench(args = CSS_IN_JS_CASES)]
fn e2e_open_css_in_js(bencher: Bencher, case: &EmbeddedBenchmarkCase) {
    bencher
        .with_inputs(|| setup_workspace(case.source))
        .bench_local_values(|(workspace, project_key)| {
            open_file(workspace.as_ref(), project_key);
            black_box(workspace)
        });
}

/// Measures formatting a JavaScript file and the CSS-in-JS templates inside it.
#[divan::bench(args = CSS_IN_JS_CASES)]
fn e2e_format_css_in_js(bencher: Bencher, case: &EmbeddedBenchmarkCase) {
    bencher
        .with_inputs(|| {
            let (workspace, project_key) = setup_workspace(case.source);
            open_file(workspace.as_ref(), project_key);
            (workspace, project_key)
        })
        .bench_local_refs(|(workspace, project_key)| {
            black_box(format_file(workspace.as_ref(), *project_key))
        });
}

fn setup_workspace(source: &str) -> (Box<dyn Workspace>, ProjectKey) {
    let fs = MemoryFileSystem::default();
    fs.insert(TARGET_PATH.into(), source.repeat(FIXTURE_REPETITIONS));

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
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            workspace_directory: Some(BiomePath::new(PROJECT_ROOT)),
            extended_configurations: Vec::new(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .expect("settings should be updated");

    (workspace, project_key)
}

fn open_file(workspace: &dyn Workspace, project_key: ProjectKey) {
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
}

fn format_file(workspace: &dyn Workspace, project_key: ProjectKey) -> Printed {
    workspace
        .format_file(FormatFileParams {
            project_key,
            path: BiomePath::new(TARGET_PATH),
            inline_config: None,
        })
        .expect("file should be formatted")
}
