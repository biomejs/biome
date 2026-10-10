//! Tests for the database query, with a minimal database holding one file.

use crate::{ComponentKind, SymbolKind, vue_model_from_source};
use biome_db::{ParsedSnippet, ParsedSource};
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_js_parser::{JsParserOptions, parse_js_with_offset};
use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource, LanguageDb};
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
    files: HashMap<Utf8PathBuf, ParsedSource>,
    storage: Storage<Self>,
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
        is_class_attribute: false,
    })
}

/// Stores a snippet the way the workspace does: parsed with its offset in the
/// host document.
fn snippet(
    db: &TestDb,
    host: &str,
    text: &str,
    source: JsFileSource,
    index: usize,
) -> ParsedSnippet {
    let start = TextSize::from(host.find(text).expect("the snippet is in the host") as u32);
    let range = TextRange::at(start, TextSize::from(text.len() as u32));
    let parsed = parse_js_with_offset(text, start, source, JsParserOptions::default());
    ParsedSnippet::new(db, parsed.into(), range, range, start, index)
}

#[test]
fn single_file_component() {
    let db = TestDb::default();
    let host = "<template><p>{{ title }}</p></template>\n<script setup>\ndefineProps(['title'])\n</script>\n";
    let snippets = vec![
        snippet(
            &db,
            host,
            "\ndefineProps(['title'])\n",
            JsFileSource::vue_setup(),
            SETUP_SOURCE,
        ),
        snippet(&db, host, " title ", expression_source(), EXPRESSION_SOURCE),
    ];
    let options = HtmlParserOptions::from(&HtmlFileSource::vue());
    let file = ParsedSource::new(
        &db,
        Utf8PathBuf::from("App.vue"),
        parse_html(host, options).into(),
        VUE_SOURCE,
        snippets,
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
    let parsed =
        biome_js_parser::parse(code, JsFileSource::js_module(), JsParserOptions::default());
    let file = ParsedSource::new(
        &db,
        Utf8PathBuf::from("component.js"),
        parsed.into(),
        JS_SOURCE,
        vec![],
    );

    let model = vue_model_from_source(&db, file);
    let component = model.components().next().expect("a component");
    assert_eq!(component.kind(), ComponentKind::DefineComponent);
    assert_eq!(component.props().next().unwrap().name(), "a");
    assert!(model.blocks().next().is_none());
}
