use crate::embedded::visitor::embedded_bindings_from_source;
use biome_languages::LanguageDb;
use biome_rowan::{TextRange, TokenText};
use camino::Utf8PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct EmbeddedBinding {
    /// The range of the binding
    pub range: TextRange,
    /// The text of the binding
    pub text: TokenText,
    /// Optionally, the source of the binding. It represents the path of the import/dynamic import.
    pub source: Option<TokenText>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedBindingTokenText {
    #[returns(ref)]
    path: Utf8PathBuf,

    #[returns(ref)]
    name: TokenText,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedBindingText {
    #[returns(ref)]
    path: Utf8PathBuf,

    #[returns(ref)]
    name: String,
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_by_name<'db>(
    db: &'db dyn LanguageDb,
    binding_name: InternedBindingTokenText<'db>,
) -> Option<EmbeddedBinding> {
    let parsed_source = db.parsed_source_for_path(binding_name.path(db))?;

    for bindings in embedded_bindings_from_source(db, parsed_source) {
        for binding in bindings {
            if binding.text.text() == *binding_name.name(db) {
                return Some(binding.clone());
            }
        }
    }
    None
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_with_source<'db>(
    db: &'db dyn LanguageDb,
    binding_name: InternedBindingTokenText<'db>,
) -> Option<EmbeddedBinding> {
    let parsed_source = db.parsed_source_for_path(binding_name.path(db))?;
    for bindings in embedded_bindings_from_source(db, parsed_source) {
        for binding in bindings {
            if binding.text.text() == *binding_name.name(db) && binding.source.is_some() {
                return Some(binding.clone());
            }
        }
    }
    None
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_by_token_text<'db>(
    db: &'db dyn LanguageDb,
    binding_name: InternedBindingTokenText<'db>,
) -> Option<EmbeddedBinding> {
    let parsed_source = db.parsed_source_for_path(binding_name.path(db))?;

    for bindings in embedded_bindings_from_source(db, parsed_source) {
        for binding in bindings {
            if binding.text.text() == binding_name.name(db).text() {
                return Some(binding.clone());
            }
        }
    }
    None
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_by_text<'db>(
    db: &'db dyn LanguageDb,
    binding_name: InternedBindingText<'db>,
) -> Option<EmbeddedBinding> {
    let parsed_source = db.parsed_source_for_path(binding_name.path(db))?;

    for bindings in embedded_bindings_from_source(db, parsed_source) {
        for binding in bindings {
            if binding.text.text() == binding_name.name(db).as_str() {
                return Some(binding.clone());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_db::testing::{Events, assert_function_query_was_not_run};
    use biome_db::{Db, ParsedSnippet, ParsedSource};
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::JsParserOptions;
    use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource, LanguageDb};
    use biome_rowan::{RawSyntaxKind, TextSize};
    use camino::{Utf8Path, Utf8PathBuf};
    use papaya::HashMap;
    use salsa::Storage;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        files: HashMap<Utf8PathBuf, ParsedSource>,
        events: Events,
        storage: Storage<Self>,
    }

    impl TestDb {
        fn new() -> Self {
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

        fn take_salsa_events(&self) -> Vec<salsa::Event> {
            std::mem::take(&mut *self.events.0.lock().unwrap())
        }

        fn clear_salsa_events(&self) {
            self.take_salsa_events();
        }

        fn insert_file(&self, path: Utf8PathBuf, file: ParsedSource) {
            self.files.pin().insert(path, file);
        }
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<biome_db::ParsedSource> {
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
                _ => DocumentFileSource::Js(JsFileSource::vue()),
            })
        }
    }

    fn parse_vue_source(db: &TestDb, source: &str) -> Utf8PathBuf {
        let path = Utf8PathBuf::from("src/App.vue");
        let parsed = parse_html(source, HtmlParserOptions::default().with_vue()).into();
        let file = ParsedSource::new(db, path.clone(), parsed, 0, vec![]);
        db.insert_file(path.clone(), file);
        path
    }

    fn parse_html_source_with_js_snippet(db: &TestDb, html: &str, js: &str) -> Utf8PathBuf {
        let path = Utf8PathBuf::from("src/file.html");
        let parsed = parse_html(html, HtmlParserOptions::default()).into();
        let snippet_parse =
            biome_js_parser::parse(js, JsFileSource::js_module(), JsParserOptions::default())
                .into();
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

    fn parse_vue_source_with_js_snippet(db: &TestDb, html: &str, js: &str) -> Utf8PathBuf {
        let path = Utf8PathBuf::from("src/App.vue");
        let parsed = parse_html(html, HtmlParserOptions::default().with_vue()).into();
        let snippet_parse =
            biome_js_parser::parse(js, JsFileSource::vue(), JsParserOptions::default()).into();
        let snippet = ParsedSnippet::new(
            db,
            snippet_parse,
            TextRange::default(),
            TextRange::default(),
            TextSize::default(),
            1,
        );
        let file = ParsedSource::new(db, path.clone(), parsed, 0, vec![snippet]);
        db.insert_file(path.clone(), file);
        path
    }

    fn token_text(text: &str) -> TokenText {
        TokenText::new_raw(RawSyntaxKind(0), text)
    }

    #[test]
    fn get_binding_by_name_finds_matching_binding() {
        let db = TestDb::new();
        let path = parse_vue_source(
            &db,
            r#"<template><div v-for="Local in items" /></template>"#,
        );

        let found = get_binding_by_name(
            &db,
            InternedBindingTokenText::new(&db, path, token_text("Local")),
        )
        .as_ref()
        .expect("binding should exist");

        assert_eq!(found.text.text(), "Local");
    }

    #[test]
    fn get_binding_with_source_ignores_local_bindings() {
        let db = TestDb::new();
        let path = parse_vue_source(
            &db,
            r#"<template><div v-for="Local in items" /></template>"#,
        );

        assert!(
            get_binding_with_source(
                &db,
                InternedBindingTokenText::new(&db, path, token_text("Local"))
            )
            .is_none()
        );
    }

    #[test]
    fn get_binding_with_source_finds_imported_binding_from_source_snippet() {
        let db = TestDb::new();
        let path = parse_vue_source_with_js_snippet(
            &db,
            "<script setup></script>",
            r#"import Component from "./Component.vue";"#,
        );

        let found = get_binding_with_source(
            &db,
            InternedBindingTokenText::new(&db, path, token_text("Component")),
        )
        .as_ref()
        .expect("imported binding should exist");

        assert_eq!(found.text.text(), "Component");
        assert_eq!(
            found.source.as_ref().map(TokenText::text),
            Some("./Component.vue")
        );
    }

    #[test]
    fn collects_bindings_from_plain_html_script_source_snippet() {
        let db = TestDb::new();
        let js = r#"import _ from "lodash"; const schema = {};"#;
        let path = parse_html_source_with_js_snippet(
            &db,
            &format!(r#"<script type="module">{js}</script>"#),
            js,
        );

        assert!(
            get_binding_by_name(
                &db,
                InternedBindingTokenText::new(&db, path.clone(), token_text("_"))
            )
            .is_some()
        );
        assert!(
            get_binding_by_name(
                &db,
                InternedBindingTokenText::new(&db, path, token_text("schema"))
            )
            .is_some()
        );
    }

    #[test]
    fn get_binding_by_name_is_memoized() {
        let db = TestDb::new();
        let path = parse_vue_source(
            &db,
            r#"<template><div v-for="Local in items" /></template>"#,
        );
        let file = db
            .parsed_source_for_path(&path)
            .expect("parsed source should be stored");

        let _ = embedded_bindings_from_source(&db, file);

        db.clear_salsa_events();
        let _ = embedded_bindings_from_source(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(&db, embedded_bindings_from_source, file, &events);
    }
}
