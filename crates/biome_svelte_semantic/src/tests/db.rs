//! Tests for the database query, with a minimal database holding one file.

use crate::{FileKind, Rune, SymbolKind, svelte_model_from_source};
use biome_db::Source;
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_js_parser::{JsParserOptions, parse_js_with_offset};
use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind, SvelteFileKind};
use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource, LanguageDb};
use biome_parser::db::{ParseDb, SnippetOutput, SourceOutput};
use biome_rowan::{TextRange, TextSize};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;

const SVELTE_SOURCE: usize = 0;
const SCRIPT_SOURCE: usize = 1;
const EXPRESSION_SOURCE: usize = 2;
const MODULE_SOURCE: usize = 3;
const JS_SOURCE: usize = 4;

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
    fn parse_source(&self, source: Source) -> SourceOutput {
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
                Some(SnippetOutput::new(
                    parsed.into(),
                    range,
                    range,
                    start,
                    *index,
                ))
            })
            .collect();
        SourceOutput::new(parse, source_index).with_snippets(snippets)
    }
}

fn svelte_source(embedding_kind: SvelteEmbeddingKind) -> JsFileSource {
    JsFileSource::js_module().with_embedding_kind(JsEmbeddingKind::Svelte {
        file_kind: SvelteFileKind::Component,
        embedding_kind,
        is_class_attribute: false,
        is_module_script: false,
    })
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource> {
        Some(match index {
            SVELTE_SOURCE => DocumentFileSource::Html(HtmlFileSource::svelte()),
            SCRIPT_SOURCE => DocumentFileSource::Js(svelte_source(SvelteEmbeddingKind::Source)),
            EXPRESSION_SOURCE => {
                DocumentFileSource::Js(svelte_source(SvelteEmbeddingKind::TextInterpolation))
            }
            MODULE_SOURCE => DocumentFileSource::Js(
                JsFileSource::try_from(Utf8Path::new("state.svelte.js")).ok()?,
            ),
            _ => DocumentFileSource::Js(JsFileSource::js_module()),
        })
    }
}

#[test]
fn component() {
    let db = TestDb::default();
    let host = "<script>\nlet { title } = $props();\n</script>\n<p>{ title }</p>\n";
    let file = db.insert_document(
        "src/routes/+page.svelte",
        host,
        SVELTE_SOURCE,
        vec![
            ("\nlet { title } = $props();\n", SCRIPT_SOURCE),
            (" title ", EXPRESSION_SOURCE),
        ],
    );

    let model = svelte_model_from_source(&db, file);
    assert!(model.is_runes_mode());
    assert_eq!(model.route_file(), Some(crate::RouteFile::PageComponent));
    let prop = model.props().next().expect("a prop");
    assert_eq!(&host[prop.host_range()], "title");
    let local = prop.local().expect("a variable");
    assert_eq!(local.rune(), Some(Rune::Props));
    // The prop is declared in the script snippet, so its range is relative
    // to that snippet's tree.
    assert_eq!(
        local.range() + local.snippet().unwrap().offset(),
        local.host_range()
    );
    let reference = local.uses().next().expect("a use in the template");
    assert_eq!(&host[reference.host_range()], "title");
    assert_eq!(reference.symbol().unwrap().kind(), SymbolKind::Local);
}

#[test]
fn module() {
    let db = TestDb::default();
    let code = "export const count = $state(0);\n";
    let file = db.insert_document("src/lib/state.svelte.js", code, MODULE_SOURCE, Vec::new());

    let model = svelte_model_from_source(&db, file);
    assert_eq!(model.file_kind(), FileKind::Module);
    let symbol = model.symbols().next().expect("a symbol");
    assert_eq!(symbol.rune(), Some(Rune::State));
    assert!(symbol.is_exported());
}

#[test]
fn other_files_have_an_empty_model() {
    let db = TestDb::default();
    let file = db.insert_document("src/lib/plain.js", "let a = 1;\n", JS_SOURCE, Vec::new());

    let model = svelte_model_from_source(&db, file);
    assert!(model.symbols().next().is_none());
}
