use biome_rowan::{TextRange, TokenText};

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

pub(crate) fn svelte_store_reference_name(reference_name: &str) -> Option<&str> {
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

/// Returns `true` if `reference_name` starts with `v` followed by an uppercase letter
/// or digit, which is the naming convention for Vue custom directives (e.g. `vHighlight`).
pub(crate) fn is_potential_vue_directive_reference(reference_name: &str) -> bool {
    matches!(
        reference_name.as_bytes(),
        [b'v', b'A'..=b'Z' | b'0'..=b'9', ..]
    )
}

/// Returns `true` if `directive_name` starts with `v-` and its camelCase
/// form matches `reference_name` (e.g. `v-highlight` matches `vHighlight`),
/// without allocating.
pub(crate) fn vue_directive_name_matches_reference_name(
    directive_name: &str,
    reference_name: &str,
) -> bool {
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
    use crate::{EmbeddedData, EmbeddedSnippet, collect_embedded_data};
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::JsParserOptions;
    use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind, SvelteFileKind};
    use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource};
    use biome_parser::AnyParse;
    use biome_rowan::{TextRange, TextSize};

    fn collect_vue_template(source: &str) -> EmbeddedData {
        let parsed: AnyParse = parse_html(source, HtmlParserOptions::default().with_vue()).into();
        collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::vue()),
            &parsed,
            Vec::new(),
        )
    }

    fn collect_vue_snippet(source: &str) -> EmbeddedData {
        let host: AnyParse = parse_html(
            "<template></template>",
            HtmlParserOptions::default().with_vue(),
        )
        .into();
        let file_source = JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Vue {
            setup: false,
            is_source: false,
            event_handler: false,
            allow_statements: false,
        });
        let snippet: AnyParse =
            biome_js_parser::parse(source, file_source, JsParserOptions::default()).into();
        collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::vue()),
            &host,
            vec![EmbeddedSnippet::new(
                &snippet,
                TextRange::default(),
                DocumentFileSource::Js(file_source),
            )],
        )
    }

    #[test]
    fn finds_vue_same_name_binding_shorthand() {
        let data = collect_vue_template(
            r#"<template><button :disabled /><button v-bind:checked /></template>"#,
        );
        assert!(data.is_used_as_value("disabled"));
        assert!(data.is_used_as_value("checked"));
    }

    #[test]
    fn ignores_vue_binding_attribute_name_with_initializer() {
        let data = collect_vue_template(
            r#"<template><button :disabled="isDisabled" /><button v-bind:checked="isChecked" /></template>"#,
        );
        assert!(!data.is_used_as_value("disabled"));
        assert!(!data.is_used_as_value("checked"));
    }

    #[test]
    fn finds_vue_custom_directives() {
        let data = collect_vue_template(
            r#"<template><div v-highlight /><div v-click-outside /><div v-require-2fa /><div v-2fa-forbidden /><div v-weird-_but-valid /></template>"#,
        );
        for name in [
            "vHighlight",
            "vClickOutside",
            "vRequire2fa",
            "v2faForbidden",
            "vWeird_butValid",
        ] {
            assert!(data.is_vue_directive_used(name), "missing {name}");
        }
        assert!(!data.is_vue_directive_used("vSomethingElse"));
    }

    #[test]
    fn ignores_invalid_vue_directive_matches() {
        let data = collect_vue_template(
            r#"<template><div v-cloak /><div v-foo- /><div v-foo--bar /><div :aria-label /></template>"#,
        );
        assert!(!data.is_vue_directive_used("vCloak"));
        assert!(!data.is_vue_directive_used("vFoo"));
        assert!(!data.is_vue_directive_used("vFooBar"));
        assert!(!data.is_vue_directive_used("ariaLabel"));
    }

    #[test]
    fn finds_svelte_store_reference() {
        let host: AnyParse = parse_html(
            "<p>{$count}</p>",
            HtmlParserOptions::default().with_svelte(),
        )
        .into();
        let file_source = JsFileSource::js_module().with_embedding_kind(JsEmbeddingKind::Svelte {
            file_kind: SvelteFileKind::Component,
            embedding_kind: SvelteEmbeddingKind::Expression,
        });
        let snippet: AnyParse =
            biome_js_parser::parse("[$count, $state]", file_source, JsParserOptions::default())
                .into();
        let data = collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::svelte()),
            &host,
            vec![EmbeddedSnippet::new(
                &snippet,
                TextRange::new(TextSize::from(4), TextSize::from(10)),
                DocumentFileSource::Js(file_source),
            )],
        );

        assert!(data.is_svelte_store_used("count"));
        assert!(!data.is_svelte_store_used("missing"));
        assert!(!data.is_svelte_store_used("state"));
    }

    #[test]
    fn finds_references_across_host_and_snippet() {
        let host: AnyParse = parse_html(
            "<template>{{ Component }}<AvatarPrimitive.Fallback /></template>",
            HtmlParserOptions::default().with_vue(),
        )
        .into();
        let file_source = JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Vue {
            setup: false,
            is_source: false,
            event_handler: false,
            allow_statements: false,
        });
        let snippet: AnyParse =
            biome_js_parser::parse("Component", file_source, JsParserOptions::default()).into();
        let data = collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::vue()),
            &host,
            vec![EmbeddedSnippet::new(
                &snippet,
                TextRange::new(TextSize::from(12), TextSize::from(21)),
                DocumentFileSource::Js(file_source),
            )],
        );

        assert!(data.is_used_as_value("Component"));
        assert!(data.is_used_as_value("AvatarPrimitive"));
        assert!(!data.is_used_as_value("Missing"));
    }

    #[test]
    fn classifies_vue_assignment_and_type_references() {
        let assignment = collect_vue_snippet("() => isNewSheetOpen = true");
        assert!(assignment.is_used_as_value("isNewSheetOpen"));

        let typed = collect_vue_snippet("foo as IconType");
        assert!(typed.is_used_as_type("IconType"));
        assert!(!typed.is_used_as_value("IconType"));
        assert!(typed.is_used("IconType"));
        assert!(typed.is_used_as_value("foo"));
    }
}
