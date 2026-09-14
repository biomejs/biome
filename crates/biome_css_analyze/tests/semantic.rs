use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_css_analyze::CssAnalyzerServices;
use biome_css_parser::{CssParserOptions, parse_css, parse_css_with_offset};
use biome_css_syntax::AnyCssRoot;
use biome_db::{
    Db, ParsedSnippet, ParsedSource,
    testing::{Events, function_query_will_execute_count_by_name},
};
use biome_languages::{CssFileSource, DocumentFileSource, LanguageDb};
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
        Some(CssFileSource::css().into())
    }
}

impl TestDb {
    fn query_count(&self, name: &str) -> usize {
        let events = std::mem::take(&mut *self.events.0.lock().unwrap());
        function_query_will_execute_count_by_name(self, name, &events)
    }
}

fn analyze(db: &Rc<TestDb>, root: &AnyCssRoot, rule: RuleFilter) -> Vec<TextRange> {
    let mut ranges = Vec::new();
    let (_, errors) = biome_css_analyze::analyze(
        root,
        AnalysisFilter {
            enabled_rules: Some(&[rule]),
            ..AnalysisFilter::default()
        },
        &AnalyzerOptions::default().with_file_path("/file.css"),
        CssAnalyzerServices::default().with_language_db(db.clone()),
        &[],
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
fn semantic_queries_are_lazy_cached_and_refresh_locations() {
    let mut db = TestDb::default();
    let parse = parse_css(
        "a { color: red; color: red; }",
        CssFileSource::css(),
        CssParserOptions::default(),
    );
    let root = parse.tree();
    let source = ParsedSource::new(&db, "/file.css".into(), parse.into(), 0, vec![]);
    db.source = Some(source);
    let mut db = Rc::new(db);
    let rule = RuleFilter::Rule("suspicious", "noDuplicateProperties");

    analyze(
        &db,
        &root,
        RuleFilter::Rule("correctness", "noUnknownProperty"),
    );
    assert_eq!(db.query_count("css_model_from_parsed_source"), 0);
    let ranges = analyze(&db, &root, rule);
    assert_eq!(ranges.len(), 1);
    assert_eq!(db.query_count("css_model_from_parsed_source"), 1);
    assert_eq!(analyze(&db, &root, rule), ranges);
    assert_eq!(db.query_count("css_model_from_parsed_source"), 0);

    let changed = parse_css(
        "\na { color: red; color: red; }",
        CssFileSource::css(),
        CssParserOptions::default(),
    );
    let changed_root = changed.tree();
    let shifted: Vec<_> = ranges
        .into_iter()
        .map(|range| range + TextSize::from(1))
        .collect();
    assert_eq!(analyze(&db, &changed_root, rule), shifted);
    assert_eq!(db.query_count("css_model_from_parsed_source"), 0);
    source
        .set_parsed(Rc::get_mut(&mut db).unwrap())
        .to(changed.into());
    assert_eq!(analyze(&db, &changed_root, rule), shifted);
    assert_eq!(db.query_count("css_model_from_parsed_source"), 1);
}

#[test]
fn semantic_queries_cache_offset_snippets() {
    let mut db = TestDb::default();
    let parse = parse_css_with_offset(
        "a { color: red; color: red; }",
        CssFileSource::css(),
        100.into(),
        CssParserOptions::default(),
    );
    let root = parse.tree();
    let snippet = ParsedSnippet::new(
        &db,
        parse.into(),
        TextRange::default(),
        TextRange::default(),
        100.into(),
        0,
    );
    let source = ParsedSource::new(
        &db,
        "/file.css".into(),
        parse_css(
            ".host {}",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .into(),
        0,
        vec![snippet],
    );
    db.source = Some(source);
    let db = Rc::new(db);
    let rule = RuleFilter::Rule("suspicious", "noDuplicateProperties");

    let ranges = analyze(&db, &root, rule);
    assert_eq!(ranges.len(), 1);
    assert_eq!(db.query_count("css_model_from_parsed_snippet"), 1);
    assert_eq!(analyze(&db, &root, rule), ranges);
    assert_eq!(db.query_count("css_model_from_parsed_snippet"), 0);
}
