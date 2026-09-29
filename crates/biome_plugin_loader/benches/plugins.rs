//! Compares GritQL plugins against JS plugins that implement the same rule.
//!
//! Every scenario ships a `.grit` and a `.js` plugin under `benches/fixtures`.
//! Before any measurement, the two plugins are checked to report the same
//! ranges on every fixture, so the numbers always compare equivalent work.
//!
//! Run with `cargo bench -p biome_plugin_loader --features js_plugin --bench plugins`.

use biome_analyze::{AnalysisFilter, AnalyzerOptions, AnalyzerPlugin, ControlFlow, Never};
use biome_diagnostics::{Diagnostic, PrintDescription};
use biome_fs::MemoryFileSystem;
use biome_js_analyze::JsAnalyzerServices;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_plugin_loader::{AnalyzerGritPlugin, AnalyzerJsPlugin};
use biome_resolver::FsWithResolverProxy;
use biome_text_size::TextRange;
use camino::Utf8Path;
use divan::{Bencher, black_box};
use std::fmt::{self, Write};
use std::sync::{Arc, Once};

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

const FILE_PATH: &str = "/project/index.js";

type Plugins = Vec<Arc<Box<dyn AnalyzerPlugin>>>;

/// A rule implemented once as a GritQL plugin and once as a JS plugin.
struct Scenario {
    name: &'static str,
    grit: &'static str,
    js: &'static str,
}

const CONSOLE_LOG: Scenario = Scenario {
    name: "console_log",
    grit: include_str!("fixtures/grit/console_log.grit"),
    js: include_str!("fixtures/js/console_log.js"),
};

const RESTRICTED_CALLEE: Scenario = Scenario {
    name: "restricted_callee",
    grit: include_str!("fixtures/grit/restricted_callee.grit"),
    js: include_str!("fixtures/js/restricted_callee.js"),
};

const UNDEFINED_REFERENCE: Scenario = Scenario {
    name: "undefined_reference",
    grit: include_str!("fixtures/grit/undefined_reference.grit"),
    js: include_str!("fixtures/js/undefined_reference.js"),
};

/// Walks the ancestors of every `await` up to the closest loop or function.
const AWAIT_IN_LOOP: Scenario = Scenario {
    name: "await_in_loop",
    grit: include_str!("fixtures/grit/await_in_loop.grit"),
    js: include_str!("fixtures/js/await_in_loop.js"),
};

const SCENARIOS: &[&Scenario] = &[
    &CONSOLE_LOG,
    &RESTRICTED_CALLEE,
    &UNDEFINED_REFERENCE,
    &AWAIT_IN_LOOP,
];

#[derive(Clone, Copy)]
enum Engine {
    Grit,
    Js,
}

const ENGINES: &[Engine] = &[Engine::Grit, Engine::Js];

impl Engine {
    fn plugin_path(self) -> &'static str {
        match self {
            Self::Grit => "/plugin.grit",
            Self::Js => "/plugin.js",
        }
    }

    fn load(self, scenario: &Scenario) -> Box<dyn AnalyzerPlugin> {
        let path = Utf8Path::new(self.plugin_path());
        let fs = MemoryFileSystem::default();
        match self {
            Self::Grit => {
                fs.insert(path.into(), scenario.grit);
                Box::new(AnalyzerGritPlugin::load(&fs, path, None).expect("valid Grit plugin"))
            }
            Self::Js => {
                fs.insert(path.into(), scenario.js);
                let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
                Box::new(AnalyzerJsPlugin::load(fs, path, None).expect("valid JS plugin"))
            }
        }
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Grit => "grit",
            Self::Js => "js",
        })
    }
}

/// Synthetic input, generated so the benchmark doesn't depend on the network.
struct Fixture {
    name: &'static str,
    source: fn() -> String,
}

/// Every block contains a match for each scenario.
const DENSE: Fixture = Fixture {
    name: "dense",
    source: dense_source,
};

/// Mostly unrelated code, with a match for each scenario every few blocks.
const SPARSE: Fixture = Fixture {
    name: "sparse",
    source: sparse_source,
};

impl fmt::Display for Fixture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

const FIXTURES: &[&Fixture] = &[&DENSE, &SPARSE];

const BLOCKS: usize = 200;

fn dense_source() -> String {
    let mut source = String::new();
    for index in 0..BLOCKS {
        writeln!(
            source,
            r#"function handler{index}(event, options) {{
    const value = Object.assign({{}}, options, {{ id: {index} }});
    if (event.type === "click") {{
        console.log("clicked", value);
    }} else {{
        console.error("unexpected", event);
    }}
    setTimeout(() => process(value, undefined), 10);
    return compute(value.id * 2, event.detail ?? undefined);
}}

async function load{index}(ids) {{
    for (const id of ids) {{
        if (id > 0) {{
            await fetch(id);
        }}
    }}
    for (let i = 0; i < ids.length; i++) {{
        ids.forEach(async (id) => await save(id));
    }}
    return await Promise.all(ids);
}}
"#
        )
        .unwrap();
    }
    source
}

fn sparse_source() -> String {
    let mut source = String::new();
    for index in 0..BLOCKS {
        writeln!(
            source,
            r#"export class Store{index} {{
    #items = new Map();
    get size() {{
        return this.#items.size;
    }}
    add(key, item) {{
        const previous = this.#items.get(key);
        this.#items.set(key, {{ ...previous, ...item, updatedAt: Date.now() }});
        return this.#items.get(key).value ?? null;
    }}
    remove(key) {{
        return [...this.#items.keys()].filter((other) => other !== key).length > 0;
    }}
}}
"#
        )
        .unwrap();
        if index % 10 == 0 {
            writeln!(
                source,
                r#"console.log("store", {index});
setInterval(() => flush({index}), 1000);
export const empty{index} = undefined;
export async function drain{index}(queue) {{
    while (queue.length > 0) {{
        await queue.shift()();
    }}
}}
"#
            )
            .unwrap();
        }
    }
    source
}

struct Case {
    scenario: &'static Scenario,
    engine: Engine,
    fixture: &'static Fixture,
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}",
            self.scenario.name, self.engine, self.fixture.name
        )
    }
}

/// The cartesian product of scenarios, engines and fixtures.
fn cases() -> impl Iterator<Item = Case> {
    SCENARIOS.iter().flat_map(|scenario| {
        FIXTURES.iter().flat_map(move |fixture| {
            ENGINES.iter().map(move |engine| Case {
                scenario,
                engine: *engine,
                fixture,
            })
        })
    })
}

struct LoadCase(&'static Scenario, Engine);

fn load_cases() -> impl Iterator<Item = LoadCase> {
    SCENARIOS.iter().flat_map(|scenario| {
        ENGINES
            .iter()
            .map(move |engine| LoadCase(scenario, *engine))
    })
}

impl fmt::Display for LoadCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0.name, self.1)
    }
}

fn parse_fixture(fixture: &Fixture) -> AnyJsRoot {
    let parsed = parse(
        &(fixture.source)(),
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    assert!(!parsed.has_errors(), "fixture {} has errors", fixture.name);
    parsed.tree()
}

/// Runs the analyzer with no built-in rules enabled, so only the plugins
/// contribute to the measured work. Returns the range of every diagnostic.
fn analyze(root: &AnyJsRoot, plugins: &Plugins) -> Vec<TextRange> {
    let filter = AnalysisFilter {
        enabled_rules: Some(&[]),
        ..AnalysisFilter::default()
    };
    let options = AnalyzerOptions::default().with_file_path(FILE_PATH);
    let services = JsAnalyzerServices::default().with_source_type(JsFileSource::js_module());

    let mut ranges = Vec::new();
    biome_js_analyze::analyze(root, filter, &options, plugins, services, |signal| {
        if let Some(diagnostic) = signal.diagnostic() {
            // Plugin failures are reported as diagnostics without a range.
            let range = diagnostic
                .location()
                .span
                .unwrap_or_else(|| panic!("the plugin failed: {}", PrintDescription(&diagnostic)));
            ranges.push(range);
        }
        ControlFlow::<Never>::Continue(())
    });
    ranges.sort_unstable_by_key(|range| (range.start(), range.end()));
    ranges
}

fn plugins(plugin: Box<dyn AnalyzerPlugin>) -> Plugins {
    vec![Arc::new(plugin)]
}

/// Asserts that both implementations of every scenario report the same ranges,
/// and that they report something at all.
fn assert_parity() {
    static PARITY: Once = Once::new();
    PARITY.call_once(|| {
        for scenario in SCENARIOS {
            for fixture in FIXTURES {
                let root = parse_fixture(fixture);
                let grit = analyze(&root, &plugins(Engine::Grit.load(scenario)));
                let js = analyze(&root, &plugins(Engine::Js.load(scenario)));
                assert!(
                    !grit.is_empty(),
                    "{}/{}: the plugins don't report anything",
                    scenario.name,
                    fixture.name
                );
                assert_eq!(
                    grit, js,
                    "{}/{}: the Grit and JS plugins report different ranges",
                    scenario.name, fixture.name
                );
            }
        }
    });
}

fn main() {
    assert_parity();
    divan::main();
}

/// Cost of turning the plugin source into something the analyzer can run:
/// compiling the GritQL pattern, or evaluating the JS module in a new context.
#[divan::bench(args = load_cases())]
fn load(bencher: Bencher, case: &LoadCase) {
    bencher.bench_local(|| black_box(case.1.load(case.0)));
}

/// Analysis without plugins. Subtract it from the other groups to isolate the
/// cost of the plugins from parsing-independent analyzer overhead.
#[divan::bench(args = FIXTURES)]
fn baseline(bencher: Bencher, fixture: &Fixture) {
    let root = parse_fixture(fixture);
    let plugins = Plugins::new();
    bencher.bench_local(|| black_box(analyze(&root, &plugins)));
}

/// First analysis with a freshly loaded plugin. JS plugins create their
/// context lazily in every thread, so this is what each worker thread pays
/// once per run. Dropping the plugin, which tears down the JS context, is
/// measured too. The plugin loading itself isn't measured.
#[divan::bench(args = cases())]
fn cold_eval(bencher: Bencher, case: &Case) {
    let root = parse_fixture(case.fixture);
    bencher
        .with_inputs(|| plugins(case.engine.load(case.scenario)))
        .bench_local_values(|plugins| black_box(analyze(&root, &plugins)));
}

/// Repeated analysis with a plugin that was already evaluated in this thread.
#[divan::bench(args = cases())]
fn steady_eval(bencher: Bencher, case: &Case) {
    let root = parse_fixture(case.fixture);
    let plugins = plugins(case.engine.load(case.scenario));
    // Warm up the thread-local state of JS plugins.
    analyze(&root, &plugins);
    bencher.bench_local(|| black_box(analyze(&root, &plugins)));
}
