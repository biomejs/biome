use crate::*;
use biome_db::{
    Db, ParsedSnippet, ParsedSource,
    testing::{Events, assert_function_query_was_not_run, assert_function_query_was_run},
};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{JsNumberLiteralExpression, JsReturnStatement};
use biome_languages::JsFileSource;
use biome_rowan::{TextRange, TextSize};
use salsa::Setter;

static_assertions::assert_impl_all!(ControlFlowModel: Send, Sync, Eq);

fn parsed(source: &str) -> biome_js_parser::Parse<AnyJsRoot> {
    parse(source, JsFileSource::ts(), JsParserOptions::default())
}

#[test]
fn equality_includes_source_and_locations() {
    let source = "function f(x) { if (x) return 1; }";
    let model = control_flow_model(&parsed(source).tree());
    assert_eq!(model, control_flow_model(&parsed(source).tree()));
    for changed in [
        "function f(x) { if (x) return 2; }",
        "function f(x) { if (y) return 1; }",
        "\nfunction f(x) { if (x) return 1; }",
        "function f(x) { while (x) return 1; }",
    ] {
        assert_ne!(
            model,
            control_flow_model(&parsed(changed).tree()),
            "{changed}"
        );
    }
}

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
    fn parsed_source_for_path(&self, _: &camino::Utf8Path) -> Option<ParsedSource> {
        None
    }
}

#[salsa::tracked]
fn graph_ranges(db: &dyn Db, source: ParsedSource) -> Vec<TextRange> {
    let mut ranges: Vec<_> = control_flow_model_from_source(db, source)
        .graphs()
        .map(|graph| graph.node.text_trimmed_range())
        .collect();
    ranges.sort();
    ranges
}

#[test]
fn source_query_reuses_equal_models_and_refreshes_locations() {
    let mut db = TestDb::default();
    let source = ParsedSource::new(
        &db,
        "file.ts".into(),
        parsed("function f() { return; }").into(),
        0,
        Vec::new(),
    );
    let original = graph_ranges(&db, source);
    db.take_events();
    assert_eq!(graph_ranges(&db, source), original);
    assert_function_query_was_not_run(
        &db,
        control_flow_model_from_source,
        source,
        &db.take_events(),
    );

    source
        .set_parsed(&mut db)
        .to(parsed("function f() { return; }").into());
    db.take_events();
    assert_eq!(graph_ranges(&db, source), original);
    let events = db.take_events();
    assert_function_query_was_run(&db, control_flow_model_from_source, source, &events);
    assert_function_query_was_not_run(&db, graph_ranges, source, &events);

    source
        .set_parsed(&mut db)
        .to(parsed("\nfunction f() { return; }").into());
    db.take_events();
    let shifted = graph_ranges(&db, source);
    assert_eq!(
        shifted,
        original
            .into_iter()
            .map(|range| range + TextSize::from(1))
            .collect::<Vec<_>>()
    );
    let events = db.take_events();
    assert_function_query_was_run(&db, control_flow_model_from_source, source, &events);
    assert_function_query_was_run(&db, graph_ranges, source, &events);

    source.set_path(&mut db).to("renamed.ts".into());
    db.take_events();
    assert_eq!(graph_ranges(&db, source), shifted);
    assert_function_query_was_not_run(
        &db,
        control_flow_model_from_source,
        source,
        &db.take_events(),
    );
}

#[test]
fn snippet_query_tracks_its_parse_and_preserves_local_coordinates() {
    let mut db = TestDb::default();
    let snippet = ParsedSnippet::new(
        &db,
        parsed("function f() { return 1; }").into(),
        TextRange::default(),
        TextRange::default(),
        100.into(),
        0,
    );
    let original = control_flow_model_from_snippet(&db, snippet).clone();
    db.take_events();
    assert_eq!(*control_flow_model_from_snippet(&db, snippet), original);
    assert_function_query_was_not_run(
        &db,
        control_flow_model_from_snippet,
        snippet,
        &db.take_events(),
    );

    snippet.set_content_offset(&mut db).to(200.into());
    db.take_events();
    assert_eq!(*control_flow_model_from_snippet(&db, snippet), original);
    assert_function_query_was_not_run(
        &db,
        control_flow_model_from_snippet,
        snippet,
        &db.take_events(),
    );

    snippet
        .set_parsed(&mut db)
        .to(parsed("function f() { return 2; }").into());
    db.take_events();
    let changed = control_flow_model_from_snippet(&db, snippet);
    assert_ne!(*changed, original);
    assert!(
        changed
            .graphs()
            .flat_map(|graph| graph.blocks)
            .flat_map(|block| block.instructions)
            .filter_map(|instruction| instruction.node?.into_node())
            .filter_map(JsReturnStatement::cast)
            .filter_map(|statement| statement.argument())
            .filter_map(|argument| JsNumberLiteralExpression::cast(argument.into_syntax()))
            .any(|literal| literal
                .value_token()
                .is_ok_and(|token| token.text_trimmed() == "2"))
    );
    assert_function_query_was_run(
        &db,
        control_flow_model_from_snippet,
        snippet,
        &db.take_events(),
    );
}

#[test]
fn loop_literal_truthiness() {
    for (condition, expected) in [
        ("true", true),
        ("((true))", true),
        ("false", false),
        ("1", true),
        ("0.0", false),
        ("0x0", false),
        ("0b0", false),
        ("0o0", false),
        ("1e-999", false),
        ("1e999", true),
        ("1_000", true),
        ("1n", true),
        ("0x0n", false),
        ("\"yes\"", true),
        ("\"\"", false),
        ("\"\\n\"", true),
        ("\"\\\\\"", true),
        ("\"\\\n\"", false),
        ("\"\\\r\n\"", false),
        ("\"\\\u{2028}\\\u{2029}\"", false),
        ("/pattern/", true),
        ("null", false),
        ("condition", false),
        ("test()", false),
    ] {
        let parse = parsed(&format!("while ({condition}) {{}}"));
        assert!(!parse.has_errors(), "{condition}");
        let statement = parse
            .syntax()
            .descendants()
            .find_map(biome_js_syntax::JsWhileStatement::cast)
            .unwrap();
        assert_eq!(
            nodes::is_truthy_literal(&statement.test().unwrap()),
            expected,
            "{condition}"
        );
    }
}
