//! A minimal Salsa database and parsing helpers shared by the crate's unit tests.

use biome_db::Source;
use biome_db::testing::Events;
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_js_parser::JsParserOptions;
use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource, LanguageDb};
use biome_parser::AnyParse;
use biome_parser::db::{ParseDb, SnippetParse, SourceParse};
use biome_rowan::{RawSyntaxKind, TextRange, TextSize, TokenText};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;

/// A snippet embedded in a test document, which the test database parses
/// along with the document.
#[derive(Clone, Debug)]
pub(crate) struct TestSnippet {
    /// The text of the snippet. It isn't necessarily the text of the document
    /// at `content_range`.
    pub(crate) text: String,
    pub(crate) element_range: TextRange,
    pub(crate) content_range: TextRange,
    pub(crate) content_offset: TextSize,
    /// The index of the file source of the snippet.
    pub(crate) source_index: usize,
}

/// Parses `text` as the file source at `source_index` in `db`.
fn parse_text(db: &dyn LanguageDb, text: &str, source_index: usize) -> AnyParse {
    match db.source_from_index(source_index) {
        Some(DocumentFileSource::Html(source)) if source.is_vue() => {
            parse_html(text, HtmlParserOptions::default().with_vue()).into()
        }
        Some(DocumentFileSource::Html(_)) => parse_html(text, HtmlParserOptions::default()).into(),
        source => {
            let source = source
                .and_then(|source| source.to_js_file_source())
                .unwrap_or_default();
            biome_js_parser::parse(text, source, JsParserOptions::default()).into()
        }
    }
}

/// Parses the text of `source` and its `snippets`, as the test databases do.
pub(crate) fn parse_test_source(
    db: &dyn LanguageDb,
    source: Source,
    snippets: &[TestSnippet],
) -> SourceParse {
    let source_index = source.file_source_index(db);
    let snippets = snippets
        .iter()
        .map(|snippet| {
            SnippetParse::new(
                parse_text(db, &snippet.text, snippet.source_index),
                snippet.element_range,
                snippet.content_range,
                snippet.content_offset,
                snippet.source_index,
            )
        })
        .collect();
    SourceParse::new(parse_text(db, source.text(db), source_index), source_index)
        .with_snippets(snippets)
}

#[salsa::db]
#[derive(Default)]
pub(crate) struct TestDb {
    files: HashMap<Utf8PathBuf, Source>,
    snippets: HashMap<Utf8PathBuf, Vec<TestSnippet>>,
    events: Events,
    storage: Storage<Self>,
}

impl TestDb {
    pub(crate) fn new() -> Self {
        let events = Events::default();
        Self {
            files: HashMap::new(),
            snippets: HashMap::new(),
            storage: salsa::Storage::new(Some(Box::new({
                let events = events.clone();
                move |event| {
                    events.0.lock().unwrap().push(event);
                }
            }))),
            events,
        }
    }

    pub(crate) fn take_salsa_events(&self) -> Vec<salsa::Event> {
        std::mem::take(&mut *self.events.0.lock().unwrap())
    }

    pub(crate) fn clear_salsa_events(&self) {
        self.take_salsa_events();
    }

    /// Stores the document `text` at `path`, parsed as the file source at
    /// `source_index`, with the given embedded snippets.
    pub(crate) fn insert_document(
        &self,
        path: &Utf8Path,
        text: &str,
        source_index: usize,
        snippets: Vec<TestSnippet>,
    ) {
        self.snippets.pin().insert(path.to_path_buf(), snippets);
        let file = Source::new(self, path.to_path_buf(), text.into(), source_index);
        self.files.pin().insert(path.to_path_buf(), file);
    }
}

#[salsa::db]
impl salsa::Database for TestDb {}

#[salsa::db]
impl ParseDb for TestDb {
    fn parse_source(&self, source: Source) -> SourceParse {
        let snippets = self.snippets.pin().get(source.path(self)).cloned();
        parse_test_source(self, source, &snippets.unwrap_or_default())
    }
}

#[salsa::db]
impl biome_db::Db for TestDb {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<Source> {
        self.files.pin().get(path).copied()
    }
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource> {
        Some(match index {
            0 => DocumentFileSource::Html(HtmlFileSource::vue()),
            2 => DocumentFileSource::Html(HtmlFileSource::html()),
            3 => DocumentFileSource::Js(JsFileSource::js_module()),
            4 => DocumentFileSource::Js(JsFileSource::vue_setup()),
            5 => DocumentFileSource::Js(
                JsFileSource::ts()
                    .with_embedding_kind(*JsFileSource::vue_setup().as_embedding_kind()),
            ),
            _ => DocumentFileSource::Js(JsFileSource::vue()),
        })
    }
}

pub(crate) fn parse_vue_source(db: &TestDb, source: &str) -> Utf8PathBuf {
    let path = Utf8PathBuf::from("src/App.vue");
    db.insert_document(&path, source, 0, Vec::new());
    path
}

/// Stores a JavaScript module that is its own host document, with the given
/// embedded snippets.
pub(crate) fn parse_js_source(
    db: &TestDb,
    source: &str,
    snippets: Vec<TestSnippet>,
) -> Utf8PathBuf {
    let path = Utf8PathBuf::from("src/file.js");
    db.insert_document(&path, source, 3, snippets);
    path
}

pub(crate) fn parse_html_source_with_js_snippet(db: &TestDb, html: &str, js: &str) -> Utf8PathBuf {
    let path = Utf8PathBuf::from("src/file.html");
    let content_start = TextSize::from(html.find(js).expect("snippet should exist") as u32);
    let content_end = content_start + TextSize::from(js.len() as u32);
    let snippet = TestSnippet {
        text: js.to_string(),
        element_range: TextRange::new(TextSize::from(0), TextSize::from(html.len() as u32)),
        content_range: TextRange::new(content_start, content_end),
        content_offset: content_start,
        source_index: 3,
    };
    db.insert_document(&path, html, 2, vec![snippet]);
    path
}

pub(crate) fn parse_vue_source_with_js_snippet(db: &TestDb, html: &str, js: &str) -> Utf8PathBuf {
    parse_vue_source_with_js_snippets(db, html, &[(js, VUE_SCRIPT_SOURCE_INDEX)])
}

/// The source index [`TestDb::source_from_index`] resolves to a plain `<script>` block.
pub(crate) const VUE_SCRIPT_SOURCE_INDEX: usize = 1;

/// The source index [`TestDb::source_from_index`] resolves to a `<script setup>` block.
pub(crate) const VUE_SCRIPT_SETUP_SOURCE_INDEX: usize = 4;

/// The source index [`TestDb::source_from_index`] resolves to a `<script setup lang="ts">`
/// block, which is the only place type-only declarations can appear.
pub(crate) const VUE_SCRIPT_SETUP_TS_SOURCE_INDEX: usize = 5;

/// Builds a Vue document from the given `<script>` snippets, each identified by the
/// source index that decides whether it is a plain `<script>` or a `<script setup>`.
pub(crate) fn parse_vue_source_with_js_snippets(
    db: &TestDb,
    html: &str,
    snippets: &[(&str, usize)],
) -> Utf8PathBuf {
    let path = Utf8PathBuf::from("src/App.vue");
    let snippets = snippets
        .iter()
        .map(|(js, source_index)| {
            // Snippets whose text is not literally present in the host document keep
            // empty ranges; they are still visited because they are embedded sources.
            let content_range = html.find(js).map_or_else(TextRange::default, |offset| {
                let start = TextSize::from(offset as u32);
                TextRange::new(start, start + TextSize::from(js.len() as u32))
            });
            TestSnippet {
                text: js.to_string(),
                element_range: content_range,
                content_range,
                content_offset: content_range.start(),
                source_index: *source_index,
            }
        })
        .collect();
    db.insert_document(&path, html, 0, snippets);
    path
}

/// Returns the document stored for `path`.
pub(crate) fn document(db: &TestDb, path: &Utf8Path) -> Source {
    biome_db::Db::parsed_source_for_path(db, path).expect("the document should be stored")
}

pub(crate) fn token_text(text: &str) -> TokenText {
    TokenText::new_raw(RawSyntaxKind(0), text)
}
