use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_db::{
    Db, ParsedSnippet, ParsedSource,
    testing::{Events, assert_function_query_was_not_run, assert_function_query_was_run},
};
use biome_js_analyze::JsAnalyzerServices;
use biome_js_control_flow::{control_flow_model_from_snippet, control_flow_model_from_source};
use biome_js_parser::{JsParserOptions, Parse, parse};
use biome_js_semantic::{semantic_model_from_snippet, semantic_model_from_source};
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use salsa::Setter;
use std::rc::Rc;

#[salsa::db]
struct TestDb {
    storage: salsa::Storage<Self>,
    events: Events,
    source: Option<ParsedSource>,
}

impl Default for TestDb {
    fn default() -> Self {
        let events = Events::default();
        Self {
            storage: salsa::Storage::new(Some(Box::new({
                let events = events.clone();
                move |event| events.0.lock().unwrap().push(event)
            }))),
            events,
            source: None,
        }
    }
}

impl TestDb {
    fn take_events(&self) -> Vec<salsa::Event> {
        std::mem::take(&mut *self.events.0.lock().unwrap())
    }
}

#[salsa::db]
impl salsa::Database for TestDb {}

#[salsa::db]
impl Db for TestDb {
    fn parsed_source_for_path(&self, _: &Utf8Path) -> Option<ParsedSource> {
        self.source
    }
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, _: usize) -> Option<DocumentFileSource> {
        Some(JsFileSource::js_module().into())
    }
}

fn parsed(source: &str) -> Parse<AnyJsRoot> {
    parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    )
}

fn analyze(db: &Rc<TestDb>, root: &AnyJsRoot, rule: &str) -> Vec<TextRange> {
    let mut ranges = Vec::new();
    let (_, errors) = biome_js_analyze::analyze(
        root,
        AnalysisFilter {
            enabled_rules: Some(&[RuleFilter::Rule("correctness", rule)]),
            ..AnalysisFilter::default()
        },
        &AnalyzerOptions::default().with_file_path("/file.js"),
        &[],
        JsAnalyzerServices::from(root).with_language_db(db.clone()),
        |signal| {
            if let Some(diagnostic) = signal.diagnostic() {
                ranges.push(diagnostic.get_span().unwrap());
            }
            ControlFlow::<Never>::Continue(())
        },
    );
    assert!(errors.is_empty(), "{errors:?}");
    ranges
}

#[test]
fn cfg_queries_are_lazy_cached_and_match_the_analyzed_tree() {
    let original = parsed("function f() { return; unreachable(); }");
    let original_root = original.tree();
    let mut db = TestDb::default();
    let source = ParsedSource::new(&db, "/file.js".into(), original.into(), 0, vec![]);
    db.source = Some(source);
    let mut db = Rc::new(db);

    let (_, errors) = biome_js_analyze::analyze(
        &original_root,
        AnalysisFilter {
            enabled_rules: Some(&[RuleFilter::Rule("suspicious", "noDebugger")]),
            ..AnalysisFilter::default()
        },
        &AnalyzerOptions::default().with_file_path("/file.js"),
        &[],
        JsAnalyzerServices::default().with_language_db(db.clone()),
        |_| ControlFlow::<Never>::Continue(()),
    );
    assert!(errors.is_empty());
    assert_function_query_was_not_run(
        db.as_ref(),
        semantic_model_from_source,
        source,
        &db.take_events(),
    );

    let unused_ranges = analyze(&db, &original_root, "noUnusedVariables");
    assert_eq!(unused_ranges.len(), 1);
    let events = db.take_events();
    assert_function_query_was_run(db.as_ref(), semantic_model_from_source, source, &events);
    assert_function_query_was_not_run(db.as_ref(), control_flow_model_from_source, source, &events);

    let original_ranges = analyze(&db, &original_root, "noUnreachable");
    assert_eq!(original_ranges.len(), 1);
    let events = db.take_events();
    assert_function_query_was_not_run(db.as_ref(), semantic_model_from_source, source, &events);
    assert_function_query_was_run(db.as_ref(), control_flow_model_from_source, source, &events);
    assert_eq!(
        analyze(&db, &original_root, "noUnreachable"),
        original_ranges
    );
    assert_function_query_was_not_run(
        db.as_ref(),
        control_flow_model_from_source,
        source,
        &db.take_events(),
    );

    let changed = parsed("\nfunction f() { return; unreachable(); }");
    let changed_root = changed.tree();
    let shifted_ranges: Vec<_> = original_ranges
        .iter()
        .map(|range| *range + TextSize::from(1))
        .collect();
    assert_eq!(analyze(&db, &changed_root, "noUnreachable"), shifted_ranges);
    assert_function_query_was_not_run(
        db.as_ref(),
        control_flow_model_from_source,
        source,
        &db.take_events(),
    );

    source
        .set_parsed(Rc::get_mut(&mut db).unwrap())
        .to(changed.into());
    db.take_events();
    assert_eq!(analyze(&db, &changed_root, "noUnreachable"), shifted_ranges);
    assert_function_query_was_run(
        db.as_ref(),
        control_flow_model_from_source,
        source,
        &db.take_events(),
    );
    assert_eq!(
        analyze(&db, &changed_root, "noUnusedVariables"),
        unused_ranges
            .into_iter()
            .map(|range| range + TextSize::from(1))
            .collect::<Vec<_>>()
    );
    assert_function_query_was_not_run(
        db.as_ref(),
        semantic_model_from_source,
        source,
        &db.take_events(),
    );
}

#[test]
fn cfg_queries_select_the_matching_snippet() {
    let mut db = TestDb::default();
    let first_parse = parsed("function first() { return; first(); }");
    let second_parse = parsed("function second() { return; second(); }");
    let second_root = second_parse.tree();
    let first = ParsedSnippet::new(
        &db,
        first_parse.into(),
        TextRange::default(),
        TextRange::default(),
        10.into(),
        0,
    );
    let second = ParsedSnippet::new(
        &db,
        second_parse.into(),
        TextRange::default(),
        TextRange::default(),
        100.into(),
        0,
    );
    let source = ParsedSource::new(
        &db,
        "/file.js".into(),
        parsed("host();").into(),
        0,
        vec![first, second],
    );
    db.source = Some(source);
    let db = Rc::new(db);

    let ranges = analyze(&db, &second_root, "noUnreachable");
    assert_eq!(ranges.len(), 1);
    let events = db.take_events();
    assert_function_query_was_run(db.as_ref(), semantic_model_from_snippet, second, &events);
    assert_function_query_was_not_run(db.as_ref(), semantic_model_from_snippet, first, &events);
    assert_function_query_was_run(
        db.as_ref(),
        control_flow_model_from_snippet,
        second,
        &events,
    );
    assert_function_query_was_not_run(db.as_ref(), control_flow_model_from_snippet, first, &events);
    assert_function_query_was_not_run(db.as_ref(), control_flow_model_from_source, source, &events);

    assert_eq!(analyze(&db, &second_root, "noUnreachable"), ranges);
    assert_function_query_was_not_run(
        db.as_ref(),
        control_flow_model_from_snippet,
        second,
        &db.take_events(),
    );
}
