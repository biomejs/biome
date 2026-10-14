//! Checks that walking only the node kinds the visitors read gives the same
//! signals as walking every node, over the analyzer and parser fixtures.

use super::*;
use biome_analyze::options::JsxRuntime;
use biome_analyze::{ActionFilter, AnalyzerConfiguration, AnalyzerOptions, Never, RuleFilter};
use biome_db::ParsedSource;
use biome_diagnostics::{Diagnostic, DiagnosticExt, print_diagnostic_to_string};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{TextRange, TextSize};
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_rowan::AstNode;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Storage;

/// Stores the analyzed file, for the services that read it from the database.
#[salsa::db]
#[derive(Default)]
struct TestDb {
    parsed: Option<ParsedSource>,
    source_type: JsFileSource,
    storage: Storage<Self>,
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
        Some(DocumentFileSource::Js(self.source_type))
    }
}

#[salsa::db]
impl biome_db::Db for TestDb {
    fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<ParsedSource> {
        self.parsed
    }
}

#[salsa::db]
impl salsa::Database for TestDb {}

/// Rules whose queries are emitted by visitors other than the syntax visitor,
/// or that run in the semantic phase.
const VISITOR_RULES: &[RuleFilter<'static>] = &[
    RuleFilter::Rule("a11y", "noAutofocus"),
    RuleFilter::Rule("complexity", "noExcessiveNestedTestSuites"),
    RuleFilter::Rule("correctness", "noUnreachableSuper"),
    RuleFilter::Rule("correctness", "noUnusedImports"),
    RuleFilter::Rule("correctness", "noUnusedLabels"),
    RuleFilter::Rule("correctness", "noUnusedVariables"),
    RuleFilter::Rule("correctness", "useHookAtTopLevel"),
    RuleFilter::Rule("correctness", "useYield"),
    RuleFilter::Rule("style", "useFilenamingConvention"),
    RuleFilter::Rule("style", "useReadonlyClassProperties"),
    RuleFilter::Rule("suspicious", "noClassAssign"),
    RuleFilter::Rule("suspicious", "noDuplicateTestHooks"),
    RuleFilter::Rule("suspicious", "noExportsInTest"),
    RuleFilter::Rule("suspicious", "useAwait"),
];

/// `noDocumentCookie` recurses without end on `var a = a;`, which some
/// fixtures contain.
const DISABLED_RULES: &[RuleFilter<'static>] =
    &[RuleFilter::Rule("suspicious", "noDocumentCookie")];

fn collect_sources(dir: &Utf8Path, sources: &mut Vec<Utf8PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| Utf8PathBuf::try_from(entry.unwrap().path()).unwrap())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_sources(&path, sources);
        } else if matches!(
            path.extension(),
            Some("js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" | "mts" | "cts")
        ) {
            sources.push(path);
        }
    }
}

/// Returns what the analyzer emits for `filter`, in emission order.
fn signals(
    root: &AnyJsRoot,
    source_type: JsFileSource,
    path: &Utf8Path,
    filter: AnalysisFilter,
    walk_every_node: bool,
) -> Vec<String> {
    let mut options = AnalyzerOptions::default()
        .with_file_path(path)
        .with_configuration(
            AnalyzerConfiguration::default().with_jsx_runtime(JsxRuntime::ReactClassic),
        );
    if walk_every_node {
        options = options.with_walk_every_node();
    }
    let mut db = TestDb {
        source_type,
        ..TestDb::default()
    };
    let parsed = ParsedSource::new(
        &db,
        path.to_path_buf(),
        root.syntax().as_send().unwrap().into(),
        0,
        vec![],
    );
    db.parsed = Some(parsed);
    let services = JsAnalyzerServices::from(root)
        .with_source_type(source_type)
        .with_language_db(Rc::new(db))
        .with_parsed_source(parsed.into());
    let mut signals = Vec::new();
    let (_, errors) = analyze(root, filter, &options, &[], services, |signal| {
        if let Some(diagnostic) = signal.diagnostic() {
            let span = diagnostic.location().span;
            let printed = print_diagnostic_to_string(&diagnostic.with_file_path(path.as_str()));
            signals.push(format!("{span:?} {printed}"));
        }
        for action in signal.actions(ActionFilter::all()) {
            if action.is_suppression() {
                continue;
            }
            let edit = action.mutation.to_text_range_and_edit();
            signals.push(format!(
                "action {:?} {:?} {edit:?}",
                action.rule_name, action.category
            ));
        }
        ControlFlow::<Never>::Continue(())
    });
    assert!(errors.is_empty(), "{path}: {errors:?}");
    signals
}

#[test]
fn kind_filtered_walk_emits_the_signals_of_a_full_walk() {
    let manifest = Utf8Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut analyzer_sources = Vec::new();
    collect_sources(&manifest.join("tests/specs"), &mut analyzer_sources);
    let mut parser_sources = Vec::new();
    collect_sources(
        &manifest.join("../biome_js_parser/tests/js_test_suite"),
        &mut parser_sources,
    );
    assert!(analyzer_sources.len() > 1000 && parser_sources.len() > 500);

    let fixtures: Vec<_> = analyzer_sources
        .iter()
        .map(|path| (path.as_path(), false))
        .chain(parser_sources.iter().map(|path| (path.as_path(), true)))
        .collect();
    let threads = std::thread::available_parallelism().map_or(1, |count| count.get().min(8));
    let compared: usize = std::thread::scope(|scope| {
        let workers: Vec<_> = fixtures
            .chunks(fixtures.len().div_ceil(threads))
            .map(|chunk| {
                // Debug builds of the parser and some rules recurse deeper than
                // the default thread stack allows on the largest fixtures.
                std::thread::Builder::new()
                    .stack_size(64 * 1024 * 1024)
                    .spawn_scoped(scope, move || {
                        chunk
                            .iter()
                            .map(|&(path, is_parser_fixture)| {
                                compare_walks(path, is_parser_fixture)
                            })
                            .sum::<usize>()
                    })
                    .unwrap()
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .sum()
    });
    assert!(compared > 1000);
}

/// Compares the walks on the file at `path` for several rule sets, and returns
/// the number of signals compared.
fn compare_walks(path: &Utf8Path, is_parser_fixture: bool) -> usize {
    let Ok(source_type) = JsFileSource::try_from(path) else {
        return 0;
    };
    let text = std::fs::read_to_string(path).unwrap();
    let parsed = parse(&text, source_type, JsParserOptions::default());
    let root = parsed.tree();
    let length = u32::from(root.syntax().text_range_with_trivia().len());
    let middle = TextRange::new(TextSize::from(length / 3), TextSize::from(length * 2 / 3));

    let only_debugger = [RuleFilter::Rule("suspicious", "noDebugger")];
    let correctness = [RuleFilter::Group("correctness")];
    let mut runs: Vec<(Option<&[RuleFilter]>, Option<TextRange>)> = vec![
        (Some(&only_debugger), None),
        (Some(VISITOR_RULES), None),
        (Some(VISITOR_RULES), Some(middle)),
        (Some(&correctness), None),
    ];
    // Alone, so that no other visitor of the phase reads the kinds it needs.
    runs.extend(
        VISITOR_RULES
            .iter()
            .map(|rule| (Some(std::slice::from_ref(rule)), None)),
    );
    // Every rule on every parser fixture adds little over the analyzer
    // fixtures and dominates the run time.
    if !is_parser_fixture {
        runs.push((None, None));
    }

    let mut compared = 0;
    for (enabled_rules, range) in runs {
        let filter = AnalysisFilter {
            enabled_rules,
            disabled_rules: DISABLED_RULES,
            range,
            ..AnalysisFilter::default()
        };
        let filtered = signals(&root, source_type, path, filter, false);
        let full = signals(&root, source_type, path, filter, true);
        assert_eq!(
            filtered, full,
            "{path} with rules {enabled_rules:?} and range {range:?}"
        );
        compared += full.len();
    }
    compared
}
