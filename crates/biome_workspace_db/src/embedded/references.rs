use crate::embedded::visitor::{
    embedded_references_from_source, embedded_type_references_from_source,
};
use biome_languages::LanguageDb;
use biome_rowan::{TextRange, TokenText};
use camino::Utf8PathBuf;

#[derive(Debug, PartialEq)]
pub struct EmbeddedValueReference {
    /// Where it's been used
    pub range: TextRange,

    /// The text of the reference
    pub text: TokenText,
}

#[derive(Debug, PartialEq)]
pub struct EmbeddedTypeReference {
    /// Where it's been used
    pub range: TextRange,

    /// The text of the reference
    pub text: TokenText,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedReference {
    #[returns(ref)]
    path: Utf8PathBuf,
    #[returns(ref)]
    name: TokenText,
}

#[salsa::tracked]
pub fn is_value_reference_used(db: &dyn LanguageDb, reference: InternedReference<'_>) -> bool {
    let parsed_source = db.parsed_source_for_path(reference.path(db));
    parsed_source.is_some_and(|parsed_source| {
        embedded_references_from_source(db, parsed_source)
            .iter()
            .any(|refs| {
                refs.iter()
                    .any(|value_reference| value_reference.text.text() == *reference.name(db))
            })
    })
}

#[salsa::tracked]
pub fn is_type_reference_used(db: &dyn LanguageDb, reference: InternedReference<'_>) -> bool {
    let parsed_source = db.parsed_source_for_path(reference.path(db));
    parsed_source.is_some_and(|parsed_source| {
        embedded_type_references_from_source(db, parsed_source)
            .iter()
            .any(|refs| {
                refs.iter()
                    .any(|type_reference| type_reference.text.text() == *reference.name(db))
            })
    })
}

#[salsa::tracked]
pub fn is_reference_used(db: &dyn LanguageDb, reference: InternedReference<'_>) -> bool {
    let parsed_source = db.parsed_source_for_path(reference.path(db));
    parsed_source.is_some_and(|parsed_source| {
        let name = reference.name(db);
        embedded_references_from_source(db, parsed_source)
            .iter()
            .any(|refs| {
                refs.iter()
                    .any(|value_reference| value_reference.text.text() == *name)
            })
            || embedded_type_references_from_source(db, parsed_source)
                .iter()
                .any(|refs| {
                    refs.iter()
                        .any(|type_reference| type_reference.text.text() == *name)
                })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_db::testing::{Events, assert_function_query_was_not_run};
    use biome_db::{Db, ParsedSnippet, ParsedSource};
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::JsParserOptions;
    use biome_languages::javascript::JsEmbeddingKind;
    use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource, LanguageDb};
    use biome_rowan::{RawSyntaxKind, TextRange, TextSize};
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
                _ => DocumentFileSource::Js(JsFileSource::ts().with_embedding_kind(
                    JsEmbeddingKind::Vue {
                        setup: false,
                        is_source: false,
                        event_handler: false,
                        allow_statements: false,
                    },
                )),
            })
        }
    }

    fn token_text(text: &str) -> TokenText {
        TokenText::new_raw(RawSyntaxKind(0), text)
    }

    fn parse_vue_source(db: &TestDb) -> Utf8PathBuf {
        let path = Utf8PathBuf::from("src/App.vue");
        let html_source = r#"<template>{{ Component }}<AvatarPrimitive.Fallback /></template>"#;
        let parsed = parse_html(html_source, HtmlParserOptions::default().with_vue()).into();
        let snippet_parse = biome_js_parser::parse(
            "Component",
            JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Vue {
                setup: false,
                is_source: false,
                event_handler: false,
                allow_statements: false,
            }),
            JsParserOptions::default(),
        )
        .into();
        let snippet = ParsedSnippet::new(
            db,
            snippet_parse,
            TextRange::new(TextSize::from(12), TextSize::from(23)),
            TextRange::new(TextSize::from(12), TextSize::from(21)),
            TextSize::from(12),
            1,
        );
        let parsed = ParsedSource::new(db, path.clone(), parsed, 0, vec![snippet]);
        db.insert_file(path.clone(), parsed);
        path
    }

    fn parse_vue_source_with_js_snippet(db: &TestDb, js_source: &str) -> Utf8PathBuf {
        let path = Utf8PathBuf::from("src/App.vue");
        let parsed = parse_html(
            "<template></template>",
            HtmlParserOptions::default().with_vue(),
        )
        .into();
        let snippet_parse = biome_js_parser::parse(
            js_source,
            JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Vue {
                setup: false,
                is_source: false,
                event_handler: false,
                allow_statements: false,
            }),
            JsParserOptions::default(),
        )
        .into();
        let snippet = ParsedSnippet::new(
            db,
            snippet_parse,
            TextRange::default(),
            TextRange::default(),
            TextSize::default(),
            1,
        );
        let parsed = ParsedSource::new(db, path.clone(), parsed, 0, vec![snippet]);
        db.insert_file(path.clone(), parsed);
        path
    }

    #[test]
    fn is_value_reference_used_finds_references_across_groups() {
        let db = TestDb::new();
        let path = parse_vue_source(&db);

        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("Component"))
        ));
        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("AvatarPrimitive"))
        ));
        assert!(!is_value_reference_used(
            &db,
            InternedReference::new(&db, path, token_text("Missing"))
        ));
    }

    #[test]
    fn is_reference_used_classifies_type_references() {
        let db = TestDb::new();
        let path = parse_vue_source_with_js_snippet(&db, "foo as IconType");

        assert!(is_type_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("IconType"))
        ));
        assert!(!is_value_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("IconType"))
        ));
        assert!(is_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("IconType"))
        ));
        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path, token_text("foo"))
        ));
    }

    #[test]
    fn is_value_reference_used_is_memoized() {
        let db = TestDb::new();
        let path = parse_vue_source(&db);
        let file = db
            .parsed_source_for_path(&path)
            .expect("parsed source should be stored");

        let _ = embedded_references_from_source(&db, file);

        db.clear_salsa_events();
        let _ = embedded_references_from_source(&db, file);
        let events = db.take_salsa_events();

        assert_function_query_was_not_run(&db, embedded_references_from_source, file, &events);
    }
}
