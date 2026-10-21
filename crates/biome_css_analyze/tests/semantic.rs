use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_css_analyze::CssAnalyzerServices;
use biome_css_parser::{CssParserOptions, parse_css, parse_css_with_offset};
use biome_css_syntax::AnyCssRoot;
use biome_db::{
    Db, Source,
    testing::{Events, function_query_will_execute_count_by_name},
};
use biome_languages::{CssFileSource, DocumentFileSource, LanguageDb};
use biome_parser::db::{AnyParsedSource, ParseDb, ParsedSourceExt, SnippetParse, SourceParse};
use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use salsa::Setter;
use std::rc::Rc;

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

#[salsa::db]
impl salsa::Database for TestDb {}

#[salsa::db]
impl Db for TestDb {
    fn parsed_source_for_path(&self, _: &Utf8Path) -> Option<Source> {
        None
    }
}

/// Marks the start of the snippet that a test document embeds: the text after
/// the marker.
const SNIPPET_MARKER: &str = "<style>";

#[salsa::db]
impl ParseDb for TestDb {
    /// Parses the text before the [SNIPPET_MARKER] as the document, and the
    /// text after it as a snippet of the document.
    fn parse_source(&self, source: Source) -> SourceParse {
        let text = source.text(self);
        let Some((document, snippet)) = text.split_once(SNIPPET_MARKER) else {
            return SourceParse::new(
                parse_css(text, CssFileSource::css(), CssParserOptions::default()).into(),
                0,
            );
        };
        let content_offset = TextSize::of(document) + TextSize::of(SNIPPET_MARKER);
        let snippet = SnippetParse::new(
            parse_css_with_offset(
                snippet,
                CssFileSource::css(),
                content_offset,
                CssParserOptions::default(),
            )
            .into(),
            TextRange::default(),
            TextRange::default(),
            content_offset,
            0,
        );
        SourceParse::new(
            parse_css(document, CssFileSource::css(), CssParserOptions::default()).into(),
            0,
        )
        .with_snippets(vec![snippet])
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

fn analyze(
    db: &Rc<TestDb>,
    root: &AnyCssRoot,
    source: Option<AnyParsedSource>,
    rule: RuleFilter,
) -> Vec<TextRange> {
    let mut ranges = Vec::new();
    let services = CssAnalyzerServices::default().with_language_db(db.clone());
    let services = match source {
        Some(source) => services.with_parsed_source(source),
        None => services,
    };
    let (_, errors) = biome_css_analyze::analyze(
        root,
        AnalysisFilter {
            enabled_rules: Some(&[rule]),
            ..AnalysisFilter::default()
        },
        &AnalyzerOptions::default().with_file_path("/file.css"),
        services,
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
    let db = TestDb::default();
    let source = Source::new(
        &db,
        "/file.css".into(),
        "a { color: red; color: red; }".into(),
        0,
    );
    let root: AnyCssRoot = source.parsed(&db).tree();
    let mut db = Rc::new(db);
    let rule = RuleFilter::Rule("suspicious", "noDuplicateProperties");

    analyze(
        &db,
        &root,
        Some(source.into()),
        RuleFilter::Rule("correctness", "noUnknownProperty"),
    );
    assert_eq!(db.query_count("css_model_from_parsed_source"), 0);
    let ranges = analyze(&db, &root, Some(source.into()), rule);
    assert_eq!(ranges.len(), 1);
    assert_eq!(db.query_count("css_model_from_parsed_source"), 1);
    assert_eq!(analyze(&db, &root, Some(source.into()), rule), ranges);
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
    assert_eq!(analyze(&db, &changed_root, None, rule), shifted);
    assert_eq!(db.query_count("css_model_from_parsed_source"), 0);
    source
        .set_text(Rc::get_mut(&mut db).unwrap())
        .to("\na { color: red; color: red; }".into());
    let changed_root: AnyCssRoot = source.parsed(db.as_ref()).tree();
    assert_eq!(
        analyze(&db, &changed_root, Some(source.into()), rule),
        shifted
    );
    assert_eq!(db.query_count("css_model_from_parsed_source"), 1);
}

#[test]
fn semantic_queries_cache_offset_snippets() {
    let db = TestDb::default();
    let source = Source::new(
        &db,
        "/file.css".into(),
        ".host {}<style>a { color: red; color: red; }".into(),
        0,
    );
    let db = Rc::new(db);
    let rule = RuleFilter::Rule("suspicious", "noDuplicateProperties");
    let snippet = source.snippets(db.as_ref())[0].to_ref(db.as_ref());
    let root: AnyCssRoot = snippet.parsed().tree();

    let ranges = analyze(&db, &root, Some(snippet.clone().into()), rule);
    assert_eq!(ranges.len(), 1);
    assert_eq!(db.query_count("css_model_from_parsed_snippet"), 1);
    assert_eq!(analyze(&db, &root, Some(snippet.into()), rule), ranges);
    assert_eq!(db.query_count("css_model_from_parsed_snippet"), 0);
}
