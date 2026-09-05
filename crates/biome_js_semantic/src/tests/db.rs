use crate::{SemanticModel, SemanticModelOptions, js_semantic_model, semantic_model};
use biome_db::FileSource;
use biome_js_parser::parse;
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_parser::{AnyParsedSource, ParsedSnippet};
use biome_rowan::{TextRange, TextSize};
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
    storage: Storage<Self>,
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
        Some(DocumentFileSource::Js(JsFileSource::svelte()))
    }
}

fn make_file(db: &TestDb, source: &str) -> FileSource {
    FileSource::new(
        db,
        Utf8PathBuf::from("test.js"),
        source.to_string(),
        0,
        None,
    )
}

#[test]
fn parsed_source_helper_builds_model() {
    let db = TestDb::default();
    let file = make_file(&db, "$store;");
    let parse = parse("$store;", JsFileSource::svelte(), Default::default());
    let expected = semantic_model(
        &parse.tree(),
        SemanticModelOptions::from(&JsFileSource::svelte()),
    );
    let parsed = AnyParsedSource::ParsedSource(parse.into());

    assert_eq!(js_semantic_model(&db, file, &parsed), expected);
}

#[test]
fn parsed_snippet_helper_builds_model() {
    let db = TestDb::default();
    let file = make_file(&db, "$store;");
    let parse = parse("$store;", JsFileSource::svelte(), Default::default());
    let expected = semantic_model(
        &parse.tree(),
        SemanticModelOptions::from(&JsFileSource::svelte()),
    );
    let empty_range = TextRange::new(TextSize::default(), TextSize::default());
    let parsed = AnyParsedSource::ParsedSnippet(ParsedSnippet {
        parsed: parse.into(),
        element_range: empty_range,
        content_range: empty_range,
        content_offset: TextSize::default(),
        document_source_index: Some(0),
    });

    assert_eq!(js_semantic_model(&db, file, &parsed), expected);
}
