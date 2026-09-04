use biome_rowan::{TextRange, TokenText};

#[derive(Debug, Clone, PartialEq)]
pub struct EmbeddedBinding {
    /// The range of the binding
    pub range: TextRange,
    /// The text of the binding
    pub text: TokenText,
    /// Optionally, the source of the binding. It represents the path of the import/dynamic import.
    pub source: Option<TokenText>,
}

#[cfg(test)]
mod tests {
    use crate::{EmbeddedSnippet, collect_embedded_data};
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::JsParserOptions;
    use biome_languages::{DocumentFileSource, HtmlFileSource, JsFileSource};
    use biome_parser::AnyParse;
    use biome_rowan::{TextRange, TextSize, TokenText};

    #[test]
    fn finds_binding_directly() {
        let parsed: AnyParse = parse_html(
            r#"<template><div v-for="Local in items" /></template>"#,
            HtmlParserOptions::default().with_vue(),
        )
        .into();
        let data = collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::vue()),
            &parsed,
            Vec::new(),
        );

        let found = data.binding("Local").expect("binding should exist");
        assert_eq!(found.text.text(), "Local");
        assert!(data.binding_with_source("Local").is_none());
    }

    #[test]
    fn finds_imported_binding_with_source() {
        let host: AnyParse = parse_html(
            "<script setup></script>",
            HtmlParserOptions::default().with_vue(),
        )
        .into();
        let snippet: AnyParse = biome_js_parser::parse(
            r#"import Component from "./Component.vue";"#,
            JsFileSource::vue_setup(),
            JsParserOptions::default(),
        )
        .into();
        let data = collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::vue()),
            &host,
            vec![EmbeddedSnippet::new(
                &snippet,
                TextRange::default(),
                DocumentFileSource::Js(JsFileSource::vue_setup()),
            )],
        );

        let found = data
            .binding_with_source("Component")
            .expect("imported binding should exist");
        assert_eq!(found.text.text(), "Component");
        assert_eq!(
            found.source.as_ref().map(TokenText::text),
            Some("./Component.vue")
        );
    }

    #[test]
    fn collects_bindings_across_html_and_javascript() {
        let js = r#"import _ from "lodash"; const schema = {};"#;
        let html = format!(r#"<script type="module">{js}</script>"#);
        let host: AnyParse = parse_html(&html, HtmlParserOptions::default()).into();
        let snippet: AnyParse =
            biome_js_parser::parse(js, JsFileSource::js_module(), JsParserOptions::default())
                .into();
        let content_start = TextSize::from(html.find(js).expect("snippet should exist") as u32);
        let content_range = TextRange::at(content_start, TextSize::from(js.len() as u32));
        let data = collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::html()),
            &host,
            vec![EmbeddedSnippet::new(
                &snippet,
                content_range,
                DocumentFileSource::Js(JsFileSource::js_module()),
            )],
        );

        assert!(data.binding("_").is_some());
        assert!(data.binding("schema").is_some());
    }
}
