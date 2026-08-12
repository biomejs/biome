//! The equality check is important for salsa.
//! The semantic model tracks some information inside a parsed file, but not everything needs to change
//! when part of the source code changes. With salsa, and the correct implementation of [PartialEq],
//! we care re-use the same semantic model, if the information that belong to the semantic model didn't change

use crate::db::{SemanticInput, semantic_model_from_source};
use crate::{SemanticModel, semantic_model};
use biome_db::FileSource;
use biome_db::testing::{Events, assert_function_query_was_not_run, assert_function_query_was_run};
use biome_js_parser::parse;
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_parser::AnyParse;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Storage;

fn build_model(source: &str) -> SemanticModel {
    let parsed = parse(source, JsFileSource::js_module(), Default::default());
    let parsed: AnyJsRoot = parsed.tree();
    semantic_model(&parsed, Default::default())
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
    fn file_source_for_path(&self, _path: &Utf8Path) -> Option<FileSource> {
        None
    }

    fn for_each_file_source(&self, _f: &mut dyn FnMut(FileSource)) {}
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
        Some(DocumentFileSource::Js(JsFileSource::tsx()))
    }
}

fn make_file(db: &TestDb, source: &str) -> FileSource {
    FileSource::new(
        db,
        Utf8PathBuf::from("test.tsx"),
        source.to_string(),
        0,
        None,
    )
}

#[salsa::tracked(returns(ref))]
fn test_semantic_model(db: &dyn LanguageDb, file: FileSource) -> SemanticModel {
    let parsed: AnyParse = parse(file.content(db), JsFileSource::tsx(), Default::default()).into();
    let input = SemanticInput::new(db, file, parsed);
    semantic_model_from_source(db, input).clone()
}

#[test]
fn semantic_model_is_memoized() {
    let mut db = TestDb::new();
    let file = make_file(&db, "let x = 1;");

    // First query — builds the model
    let _model = test_semantic_model(&db, file);

    // Second query — should be memoized
    db.clear_salsa_events();
    let _model = test_semantic_model(&db, file);
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, test_semantic_model, file, &events);
}

// Test-only downstream tracked function that reads from js_semantic_model
#[salsa::tracked]
fn binding_count(db: &dyn LanguageDb, file: FileSource) -> usize {
    let model = test_semantic_model(db, file);
    model.data.bindings.len()
}
#[test]
fn rename_does_recompute_downstream() {
    let mut db = TestDb::new();
    let file = make_file(&db, "let x = 1;");
    let _ = binding_count(&db, file);

    // Rename variable — semantic structure changes
    salsa::Setter::to(file.set_content(&mut db), "let y = 1;".to_string());

    db.clear_salsa_events();
    let _ = binding_count(&db, file);
    let events = db.take_salsa_events();

    // Both should have run — model changed, downstream must recompute
    assert_function_query_was_run(&db, test_semantic_model, file, &events);
    assert_function_query_was_run(&db, binding_count, file, &events);
}

#[test]
fn new_export_does_recompute_downstream() {
    let mut db = TestDb::new();
    let file = make_file(&db, "function f() {}");
    let _ = binding_count(&db, file);

    salsa::Setter::to(
        file.set_content(&mut db),
        "export function f() {}".to_string(),
    );

    db.clear_salsa_events();
    let _ = binding_count(&db, file);
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, test_semantic_model, file, &events);
    assert_function_query_was_run(&db, binding_count, file, &events);
}

#[test]
fn type_change_does_not_recompute_binding_count() {
    let mut db = TestDb::new();
    let file = make_file(&db, "export let x: string = 'hello';");

    let count = binding_count(&db, file);
    assert_eq!(count, 1);

    // Change type annotation — CST changes, but semantic structure doesn't
    salsa::Setter::to(
        file.set_content(&mut db),
        "export let x: number = 'hello';".to_string(),
    );

    db.clear_salsa_events();
    let count = binding_count(&db, file);
    assert_eq!(count, 1);
    let events = db.take_salsa_events();

    // Semantic model recomputed (CST input changed)...
    assert_function_query_was_run(&db, test_semantic_model, file, &events);
    // ...but binding_count did NOT (model Eq → early termination)
    assert_function_query_was_not_run(&db, binding_count, file, &events);
}
