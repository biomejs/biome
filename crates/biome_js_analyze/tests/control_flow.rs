use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, AnalyzerPlugin, ControlFlow, Never, PluginEvalResult,
    PluginTargetLanguage, RuleFilter, ServiceBag,
};
use biome_db::{
    AnyParsedSource, Db, ParsedSnippet, ParsedSource,
    testing::{Events, assert_function_query_was_not_run, assert_function_query_was_run},
};
use biome_js_analyze::JsAnalyzerServices;
use biome_js_control_flow::{control_flow_model_from_snippet, control_flow_model_from_source};
use biome_js_parser::{JsParserOptions, Parse, parse};
use biome_js_semantic::{SemanticModel, semantic_model_from_snippet, semantic_model_from_source};
use biome_js_syntax::{AnyJsRoot, JsSyntaxKind};
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_rowan::{AnySyntaxNode, RawSyntaxKind, SyntaxKind, TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Setter;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[salsa::db]
struct TestDb {
    storage: salsa::Storage<Self>,
    events: Events,
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
        None
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

fn analyze(
    db: &Rc<TestDb>,
    root: &AnyJsRoot,
    source: Option<AnyParsedSource>,
    rule: &str,
) -> Vec<TextRange> {
    let mut ranges = Vec::new();
    let services = JsAnalyzerServices::from(root).with_language_db(db.clone());
    let services = match source {
        Some(source) => services.with_parsed_source(source),
        None => services,
    };
    let (_, errors) = biome_js_analyze::analyze(
        root,
        AnalysisFilter {
            enabled_rules: Some(&[RuleFilter::Rule("correctness", rule)]),
            ..AnalysisFilter::default()
        },
        &AnalyzerOptions::default().with_file_path("/file.js"),
        &[],
        services,
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
fn cfg_only_analysis_reports_unreachable_code() {
    let parse = parsed("function example() { return; neverCalled(); }");
    let root = parse.tree();
    let db = TestDb::default();
    let source = ParsedSource::new(&db, "/file.js".into(), parse.into(), 0, vec![]);
    let db = Rc::new(db);

    for services in [
        JsAnalyzerServices::default(),
        JsAnalyzerServices::default()
            .with_language_db(db.clone())
            .with_parsed_source(source.into()),
    ] {
        let mut ranges = Vec::new();
        let (_, errors) = biome_js_analyze::analyze(
            &root,
            AnalysisFilter {
                enabled_rules: Some(&[RuleFilter::Rule("correctness", "noUnreachable")]),
                ..AnalysisFilter::default()
            },
            &AnalyzerOptions::default().with_file_path("/file.js"),
            &[],
            services,
            |signal| {
                if let Some(diagnostic) = signal.diagnostic() {
                    ranges.push(diagnostic.get_span().unwrap());
                }
                ControlFlow::<Never>::Continue(())
            },
        );
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(ranges, [TextRange::new(29.into(), 43.into())]);
        assert_function_query_was_not_run(
            db.as_ref(),
            semantic_model_from_source,
            source,
            &db.take_events(),
        );
    }
}

#[test]
fn cfg_queries_are_lazy_cached_and_match_the_analyzed_tree() {
    let original = parsed("function f() { return; unreachable(); }");
    let original_root = original.tree();
    let db = TestDb::default();
    let source = ParsedSource::new(&db, "/file.js".into(), original.into(), 0, vec![]);
    let mut db = Rc::new(db);

    let (_, errors) = biome_js_analyze::analyze(
        &original_root,
        AnalysisFilter {
            enabled_rules: Some(&[RuleFilter::Rule("suspicious", "noDebugger")]),
            ..AnalysisFilter::default()
        },
        &AnalyzerOptions::default().with_file_path("/file.js"),
        &[],
        JsAnalyzerServices::default()
            .with_language_db(db.clone())
            .with_parsed_source(source.into()),
        |_| ControlFlow::<Never>::Continue(()),
    );
    assert!(errors.is_empty());
    assert_function_query_was_not_run(
        db.as_ref(),
        semantic_model_from_source,
        source,
        &db.take_events(),
    );

    let unused_ranges = analyze(
        &db,
        &original_root,
        Some(source.into()),
        "noUnusedVariables",
    );
    assert_eq!(unused_ranges.len(), 1);
    let events = db.take_events();
    assert_function_query_was_run(db.as_ref(), semantic_model_from_source, source, &events);
    assert_function_query_was_not_run(db.as_ref(), control_flow_model_from_source, source, &events);

    let original_ranges = analyze(&db, &original_root, Some(source.into()), "noUnreachable");
    assert_eq!(original_ranges.len(), 1);
    let events = db.take_events();
    assert_function_query_was_not_run(db.as_ref(), semantic_model_from_source, source, &events);
    assert_function_query_was_run(db.as_ref(), control_flow_model_from_source, source, &events);
    assert_eq!(
        analyze(&db, &original_root, Some(source.into()), "noUnreachable"),
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
    assert_eq!(
        analyze(&db, &changed_root, None, "noUnreachable"),
        shifted_ranges
    );
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
    assert_eq!(
        analyze(&db, &changed_root, Some(source.into()), "noUnreachable"),
        shifted_ranges
    );
    assert_function_query_was_run(
        db.as_ref(),
        control_flow_model_from_source,
        source,
        &db.take_events(),
    );
    assert_eq!(
        analyze(&db, &changed_root, Some(source.into()), "noUnusedVariables"),
        unused_ranges
            .into_iter()
            .map(|range| range + TextSize::from(1))
            .collect::<Vec<_>>()
    );
    assert_function_query_was_run(
        db.as_ref(),
        semantic_model_from_source,
        source,
        &db.take_events(),
    );
}

#[test]
fn cfg_queries_use_the_supplied_snippet_with_identical_content() {
    let db = TestDb::default();
    let first_parse = parsed("function second() { return; second(); }");
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
    let db = Rc::new(db);

    let ranges = analyze(&db, &second_root, Some(second.into()), "noUnreachable");
    assert_eq!(ranges.len(), 1);
    let events = db.take_events();
    assert_function_query_was_not_run(db.as_ref(), semantic_model_from_snippet, second, &events);
    assert_function_query_was_not_run(db.as_ref(), semantic_model_from_snippet, first, &events);
    assert_function_query_was_run(
        db.as_ref(),
        control_flow_model_from_snippet,
        second,
        &events,
    );
    assert_function_query_was_not_run(db.as_ref(), control_flow_model_from_snippet, first, &events);
    assert_function_query_was_not_run(db.as_ref(), control_flow_model_from_source, source, &events);

    assert_eq!(
        analyze(&db, &second_root, Some(second.into()), "noUnreachable"),
        ranges
    );
    assert_function_query_was_not_run(
        db.as_ref(),
        control_flow_model_from_snippet,
        second,
        &db.take_events(),
    );
}

#[derive(Debug)]
struct ModelConsumerPlugin {
    requires_model: bool,
    evaluations: Arc<AtomicUsize>,
}

impl AnalyzerPlugin for ModelConsumerPlugin {
    fn name(&self) -> &str {
        "model-consumer"
    }

    fn language(&self) -> PluginTargetLanguage {
        PluginTargetLanguage::JavaScript
    }

    fn query(&self) -> Vec<RawSyntaxKind> {
        vec![JsSyntaxKind::JS_MODULE.to_raw()]
    }

    fn requires_semantic_model(&self) -> bool {
        self.requires_model
    }

    fn evaluate(
        &self,
        _: AnySyntaxNode,
        _: Utf8PathBuf,
        services: &ServiceBag,
    ) -> PluginEvalResult {
        assert_eq!(
            services.get_service::<SemanticModel>().is_some(),
            self.requires_model
        );
        self.evaluations.fetch_add(1, Ordering::Relaxed);
        PluginEvalResult::default()
    }
}

#[test]
fn plugins_request_semantic_models_before_evaluation() {
    for (requires_model, disabled) in [(false, false), (true, false), (true, true)] {
        let db = TestDb::default();
        let parse = parsed("let value = 1; value;");
        let root = parse.tree();
        let source = ParsedSource::new(&db, "/file.js".into(), parse.into(), 0, vec![]);
        let db = Rc::new(db);
        let evaluations = Arc::new(AtomicUsize::new(0));
        let plugins: Vec<Arc<Box<dyn AnalyzerPlugin>>> =
            vec![Arc::new(Box::new(ModelConsumerPlugin {
                requires_model,
                evaluations: evaluations.clone(),
            }))];
        let (_, errors) = biome_js_analyze::analyze(
            &root,
            AnalysisFilter {
                enabled_rules: Some(&[]),
                disabled_rules: if disabled {
                    &[RuleFilter::Group("plugin")]
                } else {
                    &[]
                },
                ..AnalysisFilter::default()
            },
            &AnalyzerOptions::default().with_file_path("/file.js"),
            &plugins,
            JsAnalyzerServices::default()
                .with_language_db(db.clone())
                .with_parsed_source(source.into()),
            |_| ControlFlow::<Never>::Continue(()),
        );
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(evaluations.load(Ordering::Relaxed), usize::from(!disabled));
        let events = db.take_events();
        if requires_model && !disabled {
            assert_function_query_was_run(db.as_ref(), semantic_model_from_source, source, &events);
        } else {
            assert_function_query_was_not_run(
                db.as_ref(),
                semantic_model_from_source,
                source,
                &events,
            );
        }
        assert_function_query_was_not_run(
            db.as_ref(),
            control_flow_model_from_source,
            source,
            &events,
        );
    }
}
