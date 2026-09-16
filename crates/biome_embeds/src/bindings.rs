use crate::visitor::embedded_bindings_from_source;
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
    use crate::testing::{
        TestDb, parse_html_source_with_js_snippet, parse_vue_source,
        parse_vue_source_with_js_snippet, token_text,
    };
    use biome_db::Db;
    use biome_db::testing::assert_function_query_was_not_run;

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
