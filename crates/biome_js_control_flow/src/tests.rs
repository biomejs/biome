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

fn assert_selected_graphs_match(parse: biome_js_parser::Parse<AnyJsRoot>) {
    let tree = parse.tree();
    let model = control_flow_model(&tree);
    for root in tree
        .syntax()
        .descendants()
        .filter_map(AnyJsControlFlowRoot::cast)
    {
        let selected = control_flow_graph(&root);
        let complete = model.graph(&root);
        assert_eq!(selected.is_some(), complete.is_some(), "{root:?}");
        if let (Some(selected), Some(complete)) = (selected, complete) {
            assert_eq!(selected.node, complete.node);
            assert_eq!(selected.blocks.len(), complete.blocks.len());
            for (selected, complete) in selected.blocks.iter().zip(&complete.blocks) {
                assert_eq!(selected.exception_handlers, complete.exception_handlers);
                assert_eq!(selected.cleanup_handlers, complete.cleanup_handlers);
                assert_eq!(selected.instructions.len(), complete.instructions.len());
                for (selected, complete) in selected.instructions.iter().zip(&complete.instructions)
                {
                    assert_eq!(selected.kind, complete.kind);
                    assert_eq!(selected.node, complete.node);
                }
            }
        }
    }
}

#[test]
fn selected_roots_match_complete_graphs_without_traversing_nested_roots() {
    let source = r#"
        function outer(flag, callback = () => true) {
            while (flag) {
                try {
                    const nested = () => { for (;;) { break; } };
                    function inner() { return 1; }
                    if (callback()) break;
                } finally { cleanup(); }
            }
            return flag;
        }
        const expression = function(flag) { if (flag) return 1; return 0; };
        const arrow = flag => flag && flag;
        const object = {
            get value() { return 1; },
            set value(next) { consume(next); },
            method() { return () => 1; }
        };
        class Value {
            constructor(flag) { if (flag) consume(flag); }
            get value() { return 1; }
            set value(next) { consume(next); }
            method() { return () => 1; }
            static { if (true) consume(1); }
        }
        namespace Namespace { export function inner() { return 1; } }
        interface Shape { value: string; }
        export default function () { return 1; }
    "#;
    let parsed_source = parsed(source);
    assert!(!parsed_source.has_errors());
    assert_selected_graphs_match(parsed_source);
    assert_selected_graphs_match(parse(
        "function f() { return 1; } f();",
        JsFileSource::js_script(),
        JsParserOptions::default(),
    ));
}

#[test]
fn selected_roots_preserve_malformed_statement_isolation() {
    for source in [
        "function outer() { if () work(); const inner = () => 1; }",
        "function outer(flag) { if (flag) { function inner() { if () work(); } return 1; } }",
    ] {
        let parse = parsed(source);
        assert!(parse.has_errors());
        assert_selected_graphs_match(parse);
    }
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
    let original = graph_ranges(&db, source).clone();
    db.take_events();
    assert_eq!(graph_ranges(&db, source), &original);
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
    assert_eq!(graph_ranges(&db, source), &original);
    let events = db.take_events();
    assert_function_query_was_run(&db, control_flow_model_from_source, source, &events);
    assert_function_query_was_not_run(&db, graph_ranges, source, &events);

    source
        .set_parsed(&mut db)
        .to(parsed("\nfunction f() { return; }").into());
    db.take_events();
    let shifted = graph_ranges(&db, source).clone();
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
    assert_eq!(graph_ranges(&db, source), &shifted);
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
