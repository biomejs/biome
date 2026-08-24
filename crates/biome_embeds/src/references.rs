use crate::visitor::{embedded_references_from_source, embedded_type_references_from_source};
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

/// Svelte stores are a special case. The `$` prefix is used to "dereference" the store and get its value.
///
/// See also: https://svelte.dev/docs/svelte/stores
#[salsa::tracked]
pub fn is_svelte_store_reference_used(
    db: &dyn LanguageDb,
    reference: InternedReference<'_>,
) -> bool {
    let Some(parsed_source) = db.parsed_source_for_path(reference.path(db)) else {
        return false;
    };

    embedded_references_from_source(db, parsed_source)
        .iter()
        .any(|refs| {
            refs.iter().any(|value_reference| {
                svelte_store_reference_name(value_reference.text.text()).is_some_and(
                    |reference_store_name| reference_store_name == reference.name(db).text(),
                )
            })
        })
}

fn svelte_store_reference_name(reference_name: &str) -> Option<&str> {
    // These are special Svelte runes that are not valid store names, so we should ignore them.
    const SVELTE_RUNES: [&str; 7] = [
        "$bindable",
        "$derived",
        "$effect",
        "$host",
        "$inspect",
        "$props",
        "$state",
    ];

    if SVELTE_RUNES.contains(&reference_name) {
        return None;
    }
    let store_name = reference_name.strip_prefix('$')?;
    if store_name.is_empty() || store_name.starts_with('$') {
        return None;
    }
    Some(store_name)
}

/// Vue custom directives are a special case. The template spells them in
/// kebab-case (e.g. `v-highlight`), while the JS binding they refer to is
/// spelled in camelCase (e.g. `vHighlight`).
///
/// See also: https://vuejs.org/guide/reusability/custom-directives.html
#[salsa::tracked]
pub fn is_vue_directive_reference_used(
    db: &dyn LanguageDb,
    reference: InternedReference<'_>,
) -> bool {
    let reference_name = reference.name(db).text();
    if !is_potential_vue_directive_reference(reference_name) {
        return false;
    }

    let Some(parsed_source) = db.parsed_source_for_path(reference.path(db)) else {
        return false;
    };

    embedded_references_from_source(db, parsed_source)
        .iter()
        .any(|refs| {
            refs.iter().any(|value_reference| {
                vue_directive_name_matches_reference_name(
                    value_reference.text.text(),
                    reference_name,
                )
            })
        })
}

/// Returns `true` if `reference_name` starts with `v` followed by an uppercase letter,
/// which is the naming convention for Vue custom directives (e.g. `vHighlight`).
fn is_potential_vue_directive_reference(reference_name: &str) -> bool {
    matches!(reference_name.as_bytes(), [b'v', b'A'..=b'Z', ..])
}

/// Returns `true` if `directive_name` starts with `v-` and its camelCase
/// form matches `reference_name` (e.g. `v-highlight` matches `vHighlight`),
/// without allocating.
fn vue_directive_name_matches_reference_name(directive_name: &str, reference_name: &str) -> bool {
    if !directive_name.starts_with("v-") {
        return false;
    }

    let mut directive_chars = directive_name.chars();
    let mut reference_chars = reference_name.chars();
    let mut capitalize_next = false;
    loop {
        match directive_chars.next() {
            // A dangling `-` never resolves into a match.
            None => return !capitalize_next && reference_chars.next().is_none(),
            // The first `-` in a run starts a new word. A second consecutive
            // `-` means `directive_name` contains a literal hyphen, which a
            // JS identifier (`reference_name`) can never contain so no match
            // is possible.
            Some('-') => {
                if capitalize_next {
                    return false;
                }
                capitalize_next = true;
            }
            Some(c) => {
                let Some(expected) = reference_chars.next() else {
                    return false;
                };
                let ok = if capitalize_next && c.is_ascii_alphabetic() {
                    expected.is_ascii_uppercase() && expected.eq_ignore_ascii_case(&c)
                } else {
                    expected == c
                };
                if !ok {
                    return false;
                }
                capitalize_next = false;
            }
        }
    }
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

    fn parse_vue_template_source(db: &TestDb, html_source: &str) -> Utf8PathBuf {
        let path = Utf8PathBuf::from("src/App.vue");
        let parsed = parse_html(html_source, HtmlParserOptions::default().with_vue()).into();
        let parsed = ParsedSource::new(db, path.clone(), parsed, 0, vec![]);
        db.insert_file(path.clone(), parsed);
        path
    }

    #[test]
    fn is_value_reference_used_finds_vue_same_name_binding_shorthand() {
        let db = TestDb::new();
        let path = parse_vue_template_source(&db, r#"<template><button :disabled /></template>"#);

        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path, token_text("disabled")),
        ));
    }

    #[test]
    fn is_value_reference_used_finds_vue_v_bind_same_name_binding_shorthand() {
        let db = TestDb::new();
        let path =
            parse_vue_template_source(&db, r#"<template><button v-bind:disabled /></template>"#);

        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path, token_text("disabled")),
        ));
    }

    #[test]
    fn is_value_reference_used_ignores_vue_binding_attribute_name() {
        let db = TestDb::new();
        let path = parse_vue_template_source(
            &db,
            r#"<template><button :disabled="isDisabled" /><button v-bind:disabled="isDisabled" /></template>"#,
        );

        assert!(!is_value_reference_used(
            &db,
            InternedReference::new(&db, path, token_text("disabled")),
        ));
    }

    #[test]
    fn is_vue_directive_reference_used_finds_custom_directive() {
        let db = TestDb::new();
        let path = parse_vue_template_source(
            &db,
            r#"<template><div v-highlight /><div v-click-outside /><div v-require-2fa /><div v-weird-_but-valid /></template>"#,
        );

        assert!(is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vHighlight")),
        ));
        assert!(is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vClickOutside")),
        ));
        assert!(is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vRequire2fa")),
        ));
        assert!(is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vWeird_butValid")),
        ));
    }

    #[test]
    fn is_vue_directive_reference_used_ignores_builtin_directive() {
        let db = TestDb::new();
        let path = parse_vue_template_source(&db, r#"<template><div v-cloak /></template>"#);

        assert!(!is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vCloak")),
        ));
    }

    #[test]
    fn is_vue_directive_reference_used_ignores_mismatched_name() {
        let db = TestDb::new();
        let path = parse_vue_template_source(&db, r#"<template><div v-highlight /></template>"#);

        assert!(!is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vSomethingElse")),
        ));
    }

    #[test]
    fn is_vue_directive_reference_used_ignores_trailing_hyphen() {
        let db = TestDb::new();
        let path = parse_vue_template_source(&db, r#"<template><div v-foo- /></template>"#);

        assert!(!is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vFoo")),
        ));
    }

    #[test]
    fn is_vue_directive_reference_used_ignores_consecutive_hyphens() {
        let db = TestDb::new();
        let path = parse_vue_template_source(&db, r#"<template><div v-foo--bar /></template>"#);

        assert!(!is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("vFooBar")),
        ));
    }

    #[test]
    fn is_vue_directive_reference_used_ignores_non_directive_hyphenated_reference() {
        // `:aria-label` (same-name v-bind shorthand) registers "aria-label" as a
        // plain reference, unrelated to custom directives. It must not be
        // mistaken for a `v-`-prefixed directive name that happens to also
        // camelCase-match "ariaLabel".
        let db = TestDb::new();
        let path = parse_vue_template_source(&db, r#"<template><div :aria-label /></template>"#);

        assert!(!is_vue_directive_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("ariaLabel")),
        ));
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
    fn is_value_reference_used_finds_vue_assignment_targets() {
        let db = TestDb::new();
        let path = parse_vue_source_with_js_snippet(&db, "() => isNewSheetOpen = true");

        assert!(is_value_reference_used(
            &db,
            InternedReference::new(&db, path.clone(), token_text("isNewSheetOpen"))
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
