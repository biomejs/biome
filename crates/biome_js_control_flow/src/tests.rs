use crate::*;
use biome_db::{
    Db, Source,
    testing::{Events, assert_function_query_was_not_run, assert_function_query_was_run},
};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    JsExpressionStatement, JsFunctionDeclaration, JsNumberLiteralExpression, JsReturnStatement,
    JsSyntaxKind,
};
use biome_languages::JsFileSource;
use biome_parser::AnyParse;
use biome_parser::db::{ParseDb, ParsedSourceExt, SharedParse, SnippetParse, SourceParse};
use biome_rowan::{RawSyntaxKind, SyntaxKind, TextRange, TextSize};
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::Setter;
use std::sync::Mutex;

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

#[test]
fn visited_kinds_are_the_kinds_the_visitor_acts_on() {
    for raw in 0..JsSyntaxKind::__LAST as u16 {
        let kind = JsSyntaxKind::from_raw(RawSyntaxKind(raw));
        assert_eq!(
            visitor::ControlFlowVisitor::VISITED_KINDS.matches(kind),
            visitor::ControlFlowVisitor::acts_on(kind),
            "{kind:?}"
        );
    }
}

#[test]
fn model_equals_a_visit_of_every_node() {
    for source in [
        "",
        "label: for (const a of b) { if (a) continue label; else break; }",
        "function f(x) { switch (x) { case 1: return; default: throw x; } }",
        "const f = () => { try { g(() => { debugger; }); } catch { ; } finally { h(); } };",
        "for (let i = 0; i < 1; i++) { for (const k in o) { while (k) { do { i++; } while (i); } } }",
        "class A extends B { constructor() { super(); } static { init(); } get a() { return 1; } set a(v) {} m() { return class { n() {} }; } }",
        "const o = { get a() { return 1; }, set a(v) {}, m() { return function* () { yield 1; }; } };",
        "namespace N { export function f() { return 1; } } declare module 'm' { const a: number; }",
        "interface I { a: { b: () => void }; [key: string]: unknown } type T = { c: number };",
        "a(b(c(d(async function () { await e; return () => { if (f) { return g; } }; }))));",
        "function f() { if (x) { return } } }",
        "function f() { for (;;) { break } while (",
        "class { m() { if } }",
    ] {
        let root = parsed(source).tree();
        let mut visitor = visitor::ControlFlowVisitor::new();
        let mut graphs = Vec::new();
        for event in root.syntax().preorder() {
            visitor.visit(event, &mut graphs);
        }
        assert_eq!(
            control_flow_model(&root),
            ControlFlowModel::new(&root, graphs),
            "{source}"
        );
    }
}

/// Marks the start of the snippet that a test document embeds: the text after
/// the marker.
const SNIPPET_MARKER: &str = "<script>";

#[salsa::db]
struct TestDb {
    storage: salsa::Storage<Self>,
    /// The parses of the snippet texts, which are kept while the text of a
    /// snippet doesn't change, like the parse of an unchanged snippet.
    snippet_parses: Mutex<FxHashMap<String, SharedParse>>,
    events: Events,
}

impl Default for TestDb {
    fn default() -> Self {
        let events = Events::default();
        Self {
            snippet_parses: Mutex::default(),
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
    fn parsed_source_for_path(&self, _: &camino::Utf8Path) -> Option<Source> {
        None
    }
}

#[salsa::db]
impl ParseDb for TestDb {
    /// Parses the text before the [SNIPPET_MARKER] as the document, and the
    /// text after it as a snippet of the document.
    fn parse_source(&self, source: Source) -> SourceParse {
        let text = source.text(self);
        let Some((document, snippet)) = text.split_once(SNIPPET_MARKER) else {
            return SourceParse::new(parsed(text).into(), 0);
        };
        let parse = self
            .snippet_parses
            .lock()
            .unwrap()
            .entry(snippet.to_string())
            .or_insert_with(|| AnyParse::from(parsed(snippet)).into())
            .clone();
        let content_offset = TextSize::of(document) + TextSize::of(SNIPPET_MARKER);
        SourceParse::new(parsed(document).into(), 0).with_snippets(vec![SnippetParse {
            parse,
            element_range: TextRange::default(),
            content_range: TextRange::default(),
            content_offset,
            document_source_index: 0,
        }])
    }
}

#[salsa::tracked]
fn graph_ranges(db: &dyn ParseDb, source: Source) -> Vec<TextRange> {
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
    let source = Source::new(&db, "file.ts".into(), "function f() { return; }".into(), 0);
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
        .set_text(&mut db)
        .to("function f() { return; }".into());
    db.take_events();
    assert_eq!(graph_ranges(&db, source), &original);
    let events = db.take_events();
    assert_function_query_was_run(&db, control_flow_model_from_source, source, &events);
    assert_function_query_was_not_run(&db, graph_ranges, source, &events);

    source
        .set_text(&mut db)
        .to("\nfunction f() { return; }".into());
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
    let source = Source::new(
        &db,
        "file.html".into(),
        "<script>function f() { return 1; }".into(),
        0,
    );
    let snippet = source.snippets(&db)[0];
    let original = control_flow_model_from_snippet(&db, snippet).clone();
    db.take_events();
    assert_eq!(*control_flow_model_from_snippet(&db, snippet), original);
    assert_function_query_was_not_run(
        &db,
        control_flow_model_from_snippet,
        snippet,
        &db.take_events(),
    );

    // The snippet keeps its parse, so only its offset changes.
    source
        .set_text(&mut db)
        .to(" <script>function f() { return 1; }".into());
    db.take_events();
    let snippet = source.snippets(&db)[0];
    assert_eq!(*control_flow_model_from_snippet(&db, snippet), original);
    assert_function_query_was_not_run(
        &db,
        control_flow_model_from_snippet,
        snippet,
        &db.take_events(),
    );

    source
        .set_text(&mut db)
        .to(" <script>function f() { return 2; }".into());
    db.take_events();
    let snippet = source.snippets(&db)[0];
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

/// Builds the graph of the first function in `source` and returns the text of
/// every expression statement reachable from its entry block.
fn reachable_statements(source: &str) -> FxHashSet<String> {
    let parse = parsed(source);
    assert!(!parse.has_errors(), "{source}");
    let tree = parse.tree();
    let function = tree
        .syntax()
        .descendants()
        .find_map(JsFunctionDeclaration::cast)
        .unwrap();
    let root = AnyJsControlFlowRoot::cast(function.into_syntax()).unwrap();
    let graph = control_flow_model(&tree)
        .graph(&root)
        .unwrap_or_else(|| panic!("graph should be built for {source}"));

    let mut reachable = FxHashSet::default();
    let mut visited = FxHashSet::default();
    let mut stack = vec![0];
    while let Some(index) = stack.pop() {
        if !visited.insert(index) {
            continue;
        }
        let block = &graph.blocks[index as usize];
        stack.extend(
            block
                .exception_handlers
                .iter()
                .chain(&block.cleanup_handlers)
                .map(|handler| handler.target.index()),
        );
        for instruction in &block.instructions {
            match instruction.kind {
                biome_control_flow::InstructionKind::Statement => {
                    if let Some(node) = instruction.node.as_ref().and_then(|node| node.as_node()) {
                        reachable.insert(node.text_trimmed().to_string());
                    }
                }
                biome_control_flow::InstructionKind::Jump {
                    conditional, block, ..
                } => {
                    stack.push(block.index());
                    if !conditional {
                        break;
                    }
                }
                biome_control_flow::InstructionKind::Return => break,
            }
        }
    }
    reachable
}

#[test]
fn unlabeled_jump_in_labeled_statement() {
    for source in [
        // `break` leaves the labeled loop
        "function f() { outer: for (;;) { break; skipped(); } reached(); }",
        "function f(xs) { outer: for (const x of xs) { if (x) break; reached(); } reached2(); }",
        "function f(xs) { outer: for (const x of xs) { break; skipped(); } reached(); }",
        "function f(o) { outer: for (const k in o) { break; skipped(); } reached(); }",
        "function f() { outer: while (true) { break; skipped(); } reached(); }",
        "function f() { outer: do { break; skipped(); } while (true); reached(); }",
        "function f(x) { outer: switch (x) { case 1: break; skipped(); default: reached(); } reached2(); }",
        // `break` targets the innermost loop, not an outer labeled one
        "function f() { outer: for (;;) { inner: for (;;) { break; } reached(); break; } reached2(); }",
        // `break` targets the loop, not a closer labeled block
        "function f() { outer: for (;;) { block: { break; skipped(); } skipped2(); } reached(); }",
        "function f() { for (;;) { block: { break; skipped(); } skipped2(); } reached(); }",
        // `continue` stays in the labeled loop
        "function f() { outer: for (let i = 0; i < 1; i++) { continue; skipped(); } reached(); }",
        "function f(xs) { outer: for (const x of xs) { if (x) continue; reached(); } reached2(); }",
        "function f(xs) { outer: for (const x of xs) { continue; skipped(); } reached(); }",
        "function f(o) { outer: for (const k in o) { continue; skipped(); } reached(); }",
        "function f(x) { outer: while (x) { continue; skipped(); } reached(); }",
        "function f(x) { outer: do { continue; skipped(); } while (x); reached(); }",
        "function f() { for (;;) { inner: while (true) { continue; } skipped(); } }",
    ] {
        let reachable = reachable_statements(source);
        let parse = parsed(source);
        for statement in parse
            .syntax()
            .descendants()
            .filter_map(JsExpressionStatement::cast)
        {
            let text = statement.syntax().text_trimmed().to_string();
            assert_eq!(
                reachable.contains(&text),
                text.starts_with("reached"),
                "{text} in {source}"
            );
        }
    }
}
