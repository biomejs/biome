//! A minimal Salsa database and parsing helpers shared by the crate's unit tests.

use biome_db::testing::Events;
use biome_db::{ParsedSnippet, ParsedSource};
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_js_parser::JsParserOptions;
use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource, LanguageDb};
use biome_rowan::{RawSyntaxKind, TextRange, TextSize, TokenText};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;

#[salsa::db]
#[derive(Default)]
pub(crate) struct TestDb {
    files: HashMap<Utf8PathBuf, ParsedSource>,
    events: Events,
    storage: Storage<Self>,
}

impl TestDb {
    pub(crate) fn new() -> Self {
        let events = Events::default();
        Self {
            files: HashMap::new(),
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

    pub(crate) fn insert_file(&self, path: Utf8PathBuf, file: ParsedSource) {
        self.files.pin().insert(path, file);
    }
}

#[salsa::db]
impl salsa::Database for TestDb {}

#[salsa::db]
impl biome_db::Db for TestDb {
    fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<ParsedSource> {
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
    let parsed = parse_html(source, HtmlParserOptions::default().with_vue()).into();
    let file = ParsedSource::new(db, path.clone(), parsed, 0, vec![]);
    db.insert_file(path.clone(), file);
    path
}

pub(crate) fn parse_html_source_with_js_snippet(db: &TestDb, html: &str, js: &str) -> Utf8PathBuf {
    let path = Utf8PathBuf::from("src/file.html");
    let parsed = parse_html(html, HtmlParserOptions::default()).into();
    let snippet_parse =
        biome_js_parser::parse(js, JsFileSource::js_module(), JsParserOptions::default()).into();
    let content_start = TextSize::from(html.find(js).expect("snippet should exist") as u32);
    let content_end = content_start + TextSize::from(js.len() as u32);
    let snippet = ParsedSnippet::new(
        db,
        snippet_parse,
        TextRange::new(TextSize::from(0), TextSize::from(html.len() as u32)),
        TextRange::new(content_start, content_end),
        content_start,
        3,
    );
    let file = ParsedSource::new(db, path.clone(), parsed, 2, vec![snippet]);
    db.insert_file(path.clone(), file);
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
    let parsed = parse_html(html, HtmlParserOptions::default().with_vue()).into();
    let snippets = snippets
        .iter()
        .map(|(js, source_index)| {
            let file_source = db
                .source_from_index(*source_index)
                .and_then(|source| source.to_js_file_source())
                .expect("the source index should resolve to a JavaScript file source");
            let snippet_parse =
                biome_js_parser::parse(js, file_source, JsParserOptions::default()).into();
            // Snippets whose text is not literally present in the host document keep
            // empty ranges; they are still visited because they are embedded sources.
            let content_range = html.find(js).map_or_else(TextRange::default, |offset| {
                let start = TextSize::from(offset as u32);
                TextRange::new(start, start + TextSize::from(js.len() as u32))
            });
            ParsedSnippet::new(
                db,
                snippet_parse,
                content_range,
                content_range,
                content_range.start(),
                *source_index,
            )
        })
        .collect();
    let file = ParsedSource::new(db, path.clone(), parsed, 0, snippets);
    db.insert_file(path.clone(), file);
    path
}

pub(crate) fn token_text(text: &str) -> TokenText {
    TokenText::new_raw(RawSyntaxKind(0), text)
}
