//! The equality check is important for salsa.
//! The semantic model tracks some information inside a parsed file, but not everything needs to change
//! when part of the source code changes. With salsa, and the correct implementation of [PartialEq],
//! we care re-use the same semantic model, if the information that belong to the semantic model didn't change

use crate::db::semantic_model_from_source;
use crate::{
    SemanticEventExtractor, SemanticModel, SemanticModelBuilder, SemanticModelOptions,
    semantic_model,
};
use biome_db::ParsedSource;
use biome_db::testing::{Events, assert_function_query_was_not_run, assert_function_query_was_run};
use biome_js_parser::parse;
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_rowan::AstNode;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Storage;

fn build_model(source: &str) -> SemanticModel {
    let parsed = parse(source, JsFileSource::js_module(), Default::default());
    let parsed: AnyJsRoot = parsed.tree();
    semantic_model(&parsed, Default::default())
}

fn assert_pointers_resolve(source: &str, source_type: JsFileSource) {
    let parsed = parse(source, source_type, Default::default());
    let root: AnyJsRoot = parsed.tree();
    let model = semantic_model(&root, SemanticModelOptions::from(&source_type));
    let syntax = root.syntax();

    for pointer in model.data.binding_node_by_start.values() {
        assert!(
            pointer.try_to_node(syntax).is_some(),
            "unresolved binding pointer for {source:?}: {pointer:?}"
        );
    }
    for pointer in model.data.scope_node_by_range.values() {
        assert!(
            pointer.try_to_node(syntax).is_some(),
            "unresolved scope pointer for {source:?}: {pointer:?}"
        );
    }
    for scope in model.scopes() {
        assert!(scope.syntax().is_some(), "unresolved scope for {source:?}");
    }
    for binding in model.all_bindings() {
        assert!(
            binding.syntax().is_some(),
            "unresolved binding for {source:?}"
        );
        assert!(
            binding.tree().is_some(),
            "unresolved binding tree for {source:?}"
        );
        for reference in binding.all_references() {
            assert!(
                reference.syntax().is_some(),
                "unresolved reference for {source:?}"
            );
        }
    }
    for reference in model.all_global_references() {
        assert!(
            reference.syntax().is_some(),
            "unresolved global reference for {source:?}"
        );
    }
    for reference in model.all_unresolved_references() {
        assert!(
            reference.syntax().is_some(),
            "unresolved reference for {source:?}"
        );
    }
}

#[test]
fn pointers_resolve_for_partial_sources() {
    let cases = [
        (
            "export function example(value) { const result = value + external; return result; }",
            JsFileSource::js_module(),
        ),
        (
            "interface Example<T> { value: T } const element = <Component value={external} />;",
            JsFileSource::tsx(),
        ),
    ];

    for (source, source_type) in cases {
        for end in source
            .char_indices()
            .map(|(index, _)| index)
            .chain(std::iter::once(source.len()))
        {
            assert_pointers_resolve(&source[..end], source_type);
        }
    }
}

#[test]
fn accessors_return_none_for_foreign_nodes() {
    let model_root: AnyJsRoot = parse("", JsFileSource::js_module(), Default::default()).tree();
    let foreign_root: AnyJsRoot = parse(
        "let value = external;",
        JsFileSource::js_module(),
        Default::default(),
    )
    .tree();
    let mut extractor = SemanticEventExtractor::default();
    let mut builder = SemanticModelBuilder::new(model_root);

    for event in foreign_root.syntax().preorder() {
        match event {
            biome_js_syntax::WalkEvent::Enter(node) => {
                builder.push_node(&node);
                extractor.enter(&node);
            }
            biome_js_syntax::WalkEvent::Leave(node) => extractor.leave(&node),
        }
    }
    while let Some(event) = extractor.pop() {
        builder.push_event(event);
    }

    let model = builder.build();
    let binding = model.all_bindings().next().unwrap();
    assert!(binding.syntax().is_none());
    assert!(binding.tree().is_none());
    assert!(
        binding
            .all_references()
            .all(|reference| reference.syntax().is_none())
    );
    assert!(model.scopes().all(|scope| scope.syntax().is_none()));
    assert!(
        model
            .all_unresolved_references()
            .all(|reference| reference.syntax().is_none())
    );
}

#[test]
fn type_annotation_change_is_eq() {
    let source_a = "export function f(x: string): void { return; }";
    let source_b = "export function f(x: number): void { return; }";
    let model_a = build_model(source_a);
    let model_b = build_model(source_b);
    assert_eq!(
        model_a, model_b,
        "type-only change should produce equal models"
    );
}

#[test]
fn variable_rename_is_not_eq() {
    let model_a = build_model("let x = 1;");
    let model_b = build_model("let y = 1;");
    assert_ne!(model_a, model_b, "rename should produce different models");
}

#[test]
fn added_export_is_not_eq() {
    let model_a = build_model("function f() {}");
    let model_b = build_model("export function f() {}");
    assert_ne!(
        model_a, model_b,
        "export change should produce different models"
    );
}

#[test]
fn whitespace_change_is_eq() {
    let model_a = build_model("let x = 1;");
    let model_b = build_model("let   x   =   1 ;");
    assert_eq!(model_a, model_b, "whitespace should not affect semantic eq");
}

#[test]
fn comment_change_is_eq() {
    assert_eq!(
        build_model("let x = 1; // old comment"),
        build_model("let x = 1; // new comment"),
    );
}

#[test]
fn body_change_same_bindings_is_eq() {
    assert_eq!(
        build_model("function f() { return 1; }"),
        build_model("function f() { return 2; }"),
    );
}

#[test]
fn new_binding_is_not_eq() {
    assert_ne!(
        build_model("let x = 1;"),
        build_model("let x = 1; let y = 2;"),
    );
}

#[test]
fn declaration_kind_change_is_not_eq() {
    assert_ne!(build_model("let x = 1;"), build_model("function x() {}"),);
}

#[test]
fn let_vs_const_is_eq() {
    assert_eq!(build_model("let x = 1;"), build_model("const x = 1;"),);
}

#[test]
fn same_export_different_position() {
    // Different because of different hoisting kind
    assert_ne!(
        build_model("let x = 1; export {x};"),
        build_model("export {x}; let x = 1;"),
    );
}

#[salsa::db]
#[derive(Default)]
pub struct TestDb {
    events: Events,

    storage: Storage<Self>,
}

impl TestDb {
    pub fn new() -> Self {
        let events = Events::default();
        Self {
            storage: salsa::Storage::new(Some(Box::new({
                let events = events.clone();
                move |event| {
                    events.0.lock().unwrap().push(event);
                }
            }))),
            events,
        }
    }
    pub fn take_salsa_events(&mut self) -> Vec<salsa::Event> {
        std::mem::take(&mut *self.events.0.lock().unwrap())
    }

    pub fn clear_salsa_events(&mut self) {
        self.take_salsa_events();
    }
}

#[salsa::db]
impl salsa::Database for TestDb {}

#[salsa::db]
impl biome_db::Db for TestDb {
    fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<ParsedSource> {
        unreachable!("Not currently touched by the test")
    }
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
        Some(DocumentFileSource::Js(JsFileSource::tsx()))
    }
}

#[test]
fn semantic_model_is_memoized() {
    let mut db = TestDb::new();
    let parsed = parse("let x = 1;", JsFileSource::tsx(), Default::default()).into();
    let file = ParsedSource::new(&db, Utf8PathBuf::from("test.tsx"), parsed, 0, vec![]);

    // First query — builds the model
    let _model = semantic_model_from_source(&db, file);

    // Second query — should be memoized
    db.clear_salsa_events();
    let _model = semantic_model_from_source(&db, file);
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, semantic_model_from_source, file, &events);
}

// Test-only downstream tracked function that reads from js_semantic_model
#[salsa::tracked]
fn binding_count(db: &dyn LanguageDb, file: ParsedSource) -> usize {
    let model = semantic_model_from_source(db, file);
    model.data.bindings.len()
}
#[test]
fn rename_does_recompute_downstream() {
    let mut db = TestDb::new();
    let parsed = parse("let x = 1;", JsFileSource::tsx(), Default::default()).into();
    let file = ParsedSource::new(&db, Utf8PathBuf::from("test.tsx"), parsed, 0, vec![]);
    let _ = binding_count(&db, file);

    // Rename variable — semantic structure changes
    let new_parsed = parse("let y = 1;", JsFileSource::tsx(), Default::default()).into();
    salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

    db.clear_salsa_events();
    let _ = binding_count(&db, file);
    let events = db.take_salsa_events();

    // Both should have run — model changed, downstream must recompute
    assert_function_query_was_run(&db, semantic_model_from_source, file, &events);
    // assert_function_query_was_run(&db, binding_count, file, &events);
}

#[test]
fn new_export_does_recompute_downstream() {
    let mut db = TestDb::new();
    let parsed = parse("function f() {}", JsFileSource::tsx(), Default::default()).into();
    let file = ParsedSource::new(&db, Utf8PathBuf::from("test.tsx"), parsed, 0, vec![]);
    let _ = binding_count(&db, file);

    let new_parsed = parse(
        "export function f() {}",
        JsFileSource::tsx(),
        Default::default(),
    )
    .into();
    salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

    db.clear_salsa_events();
    let _ = binding_count(&db, file);
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, semantic_model_from_source, file, &events);
    assert_function_query_was_run(&db, binding_count, file, &events);
}

#[test]
fn type_change_does_not_recompute_binding_count() {
    let mut db = TestDb::new();
    let parsed = parse(
        "export let x: string = 'hello';",
        JsFileSource::tsx(),
        Default::default(),
    )
    .into();
    let file = ParsedSource::new(&db, Utf8PathBuf::from("test.tsx"), parsed, 0, vec![]);

    let count = binding_count(&db, file);
    assert_eq!(count, 1);

    // Change type annotation — CST changes, but semantic structure doesn't
    let new_parsed = parse(
        "export let x: number = 'hello';",
        JsFileSource::tsx(),
        Default::default(),
    )
    .into();
    salsa::Setter::to(file.set_parsed(&mut db), new_parsed);

    db.clear_salsa_events();
    let count = binding_count(&db, file);
    assert_eq!(count, 1);
    let events = db.take_salsa_events();

    // Semantic model recomputed (CST input changed)...
    assert_function_query_was_run(&db, semantic_model_from_source, file, &events);
    // ...but binding_count did NOT (model Eq → early termination)
    assert_function_query_was_not_run(&db, binding_count, file, &events);
}
