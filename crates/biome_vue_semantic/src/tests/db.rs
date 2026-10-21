//! Tests for the database query, with a minimal database holding one file.

use crate::{ComponentKind, SymbolKind, vue_model_from_source};
use biome_db::Source;
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_js_parser::{JsParserOptions, parse_js_with_offset};
use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource, LanguageDb};
use biome_parser::db::{ParseDb, SnippetParse, SourceParse};
use biome_rowan::{TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;

const VUE_SOURCE: usize = 0;
const SETUP_SOURCE: usize = 1;
const EXPRESSION_SOURCE: usize = 2;
const JS_SOURCE: usize = 3;

#[salsa::db]
#[derive(Default)]
struct TestDb {
    files: HashMap<Utf8PathBuf, Source>,
    /// The snippets of each document: their text, which is in the document,
    /// and the index of their file source.
    snippets: HashMap<Utf8PathBuf, Vec<(&'static str, usize)>>,
    storage: Storage<Self>,
}

impl TestDb {
    /// Stores the document `text` at `path`, parsed as the file source at
    /// `source_index`, with the given snippets.
    fn insert_document(
        &self,
        path: &str,
        text: &str,
        source_index: usize,
        snippets: Vec<(&'static str, usize)>,
    ) -> Source {
        let path = Utf8PathBuf::from(path);
        self.snippets.pin().insert(path.clone(), snippets);
        let file = Source::new(self, path.clone(), text.into(), source_index);
        self.files.pin().insert(path, file);
        file
    }
}

#[salsa::db]
impl salsa::Database for TestDb {}

#[salsa::db]
impl biome_db::Db for TestDb {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<Source> {
        self.files.pin().get(path).copied()
    }
}

/// Parses a document, and its snippets the way the workspace does: with their
/// offset in the document.
#[salsa::db]
impl ParseDb for TestDb {
    fn parse_source(&self, source: Source) -> SourceParse {
        let text = source.text(self);
        let source_index = source.file_source_index(self);
        let js_source = |index| {
            self.source_from_index(index)
                .and_then(|source| source.to_js_file_source())
                .unwrap_or_default()
        };
        let parse = match self.source_from_index(source_index) {
            Some(DocumentFileSource::Html(html)) => {
                parse_html(text, HtmlParserOptions::from(&html)).into()
            }
            _ => biome_js_parser::parse(text, js_source(source_index), JsParserOptions::default())
                .into(),
        };
        let snippets = self
            .snippets
            .pin()
            .get(source.path(self))
            .into_iter()
            .flatten()
            .filter_map(|(snippet, index)| {
                let start = TextSize::from(text.find(snippet)? as u32);
                let range = TextRange::at(start, TextSize::of(*snippet));
                let parsed = parse_js_with_offset(
                    snippet,
                    start,
                    js_source(*index),
                    JsParserOptions::default(),
                );
                Some(SnippetParse::new(
                    parsed.into(),
                    range,
                    range,
                    start,
                    *index,
                ))
            })
            .collect();
        SourceParse::new(parse, source_index).with_snippets(snippets)
    }
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource> {
        Some(match index {
            VUE_SOURCE => DocumentFileSource::Html(HtmlFileSource::vue()),
            SETUP_SOURCE => DocumentFileSource::Js(JsFileSource::vue_setup()),
            EXPRESSION_SOURCE => DocumentFileSource::Js(expression_source()),
            _ => DocumentFileSource::Js(JsFileSource::js_module()),
        })
    }
}

/// The file source the workspace gives a template expression.
fn expression_source() -> JsFileSource {
    use biome_languages::javascript::JsEmbeddingKind;
    JsFileSource::js_module().with_embedding_kind(JsEmbeddingKind::Vue {
        setup: false,
        is_source: false,
        event_handler: false,
        allow_statements: false,
        slot_props: false,
        generics_list: false,
        is_class_attribute: false,
    })
}

#[test]
fn single_file_component() {
    let db = TestDb::default();
    let host = "<template><p>{{ title }}</p></template>\n<script setup>\ndefineProps(['title'])\n</script>\n";
    let file = db.insert_document(
        "App.vue",
        host,
        VUE_SOURCE,
        vec![
            ("\ndefineProps(['title'])\n", SETUP_SOURCE),
            (" title ", EXPRESSION_SOURCE),
        ],
    );

    let model = vue_model_from_source(&db, file);
    let component = model.template_component().expect("a component");
    assert_eq!(component.kind(), ComponentKind::Setup);
    let prop = component.props().next().expect("a prop");
    assert_eq!(&host[prop.host_range()], "'title'");
    // The prop is declared in the script snippet, so its range is relative
    // to that snippet's tree.
    assert_eq!(
        prop.range() + prop.snippet().unwrap().offset(),
        prop.host_range()
    );
    let reference = prop.uses().next().expect("a use in the template");
    assert_eq!(&host[reference.host_range()], "title");
    assert_eq!(reference.symbol().unwrap().kind(), SymbolKind::Prop);
}

#[test]
fn javascript_file() {
    let db = TestDb::default();
    let code = "import { defineComponent } from 'vue';\ndefineComponent({ props: ['a'] });\n";
    let file = db.insert_document("component.js", code, JS_SOURCE, Vec::new());

    let model = vue_model_from_source(&db, file);
    let component = model.components().next().expect("a component");
    assert_eq!(component.kind(), ComponentKind::DefineComponent);
    assert_eq!(component.props().next().unwrap().name(), "a");
    assert!(model.blocks().next().is_none());
}
