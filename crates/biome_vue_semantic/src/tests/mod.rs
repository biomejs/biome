mod component;
mod db;
mod sfc;

use crate::{SemanticModel, SfcSnippet, component_model, sfc_model};
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnyVueDirective, HtmlDoubleTextExpression, HtmlElement,
    HtmlEmbeddedContent, HtmlRoot, VueVForValue,
};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::javascript::JsEmbeddingKind;
use biome_languages::{HtmlFileSource, JsFileSource};
use biome_rowan::{AstNode, TextRange, TextSize};

/// Builds the component model of a JavaScript tree parsed as `source`.
pub(crate) fn js_model(code: &str, source: JsFileSource) -> SemanticModel {
    let parsed = biome_js_parser::parse(code, source, JsParserOptions::default());
    let root = parsed.tree();
    let js = semantic_model(&root, SemanticModelOptions::from(&source));
    component_model(&root, &js, source)
}

/// The kinds of embedded snippet the tests parse.
#[derive(Clone, Copy)]
enum SnippetKind {
    Script { setup: bool },
    Expression,
    EventHandler,
    SlotProps,
}

fn vue_kind(kind: SnippetKind) -> JsEmbeddingKind {
    let is_source = matches!(kind, SnippetKind::Script { .. });
    JsEmbeddingKind::Vue {
        setup: matches!(kind, SnippetKind::Script { setup: true }),
        is_source,
        event_handler: matches!(kind, SnippetKind::EventHandler),
        allow_statements: is_source,
        slot_props: matches!(kind, SnippetKind::SlotProps),
        is_class_attribute: false,
    }
}

/// Builds the model of a single-file component.
///
/// The workspace finds and parses the embedded snippets of a `.vue` file in
/// production. This helper does the same for the constructs the tests use.
pub(crate) fn sfc(code: &str) -> SemanticModel {
    let options = HtmlParserOptions::from(&HtmlFileSource::vue());
    let host: HtmlRoot = parse_html(code, options).tree();
    // (text, content range, content offset, embedding kind)
    let mut found: Vec<(String, TextRange, TextSize, JsEmbeddingKind)> = Vec::new();
    let mut is_ts = false;

    for node in host.syntax().descendants() {
        if let Some(element) = HtmlElement::cast_ref(&node) {
            if !element.is_script_tag() {
                continue;
            }
            is_ts |= element.is_typescript_lang();
            let Some(content) = element
                .children()
                .syntax()
                .descendants()
                .find_map(HtmlEmbeddedContent::cast)
                .and_then(|content| content.value_token().ok())
            else {
                continue;
            };
            found.push((
                content.text().to_string(),
                content.text_trimmed_range(),
                content.text_range().start(),
                vue_kind(SnippetKind::Script {
                    setup: element.is_script_with_setup_attribute(),
                }),
            ));
        } else if let Some(expression) = HtmlDoubleTextExpression::cast_ref(&node) {
            if let Some(token) = expression
                .expression()
                .ok()
                .and_then(|expression| expression.html_literal_token().ok())
            {
                found.push((
                    token.text().to_string(),
                    token.text_trimmed_range(),
                    token.text_range().start(),
                    vue_kind(SnippetKind::Expression),
                ));
            }
        } else if let Some(v_for) = VueVForValue::cast_ref(&node) {
            if let Some(token) = v_for
                .expression()
                .ok()
                .and_then(|expression| expression.html_literal_token().ok())
            {
                found.push((
                    token.text().to_string(),
                    token.text_trimmed_range(),
                    token.text_range().start(),
                    vue_kind(SnippetKind::Expression),
                ));
            }
        } else if let Some(directive) = AnyVueDirective::cast_ref(&node) {
            let (initializer, kind) = match &directive {
                AnyVueDirective::VueDirective(directive) => {
                    let name = directive
                        .name_token()
                        .map(|token| token.text_trimmed().to_string())
                        .unwrap_or_default();
                    (
                        directive.initializer(),
                        if name == "v-on" && directive.arg().is_some() {
                            SnippetKind::EventHandler
                        } else if name == "v-slot" {
                            SnippetKind::SlotProps
                        } else {
                            SnippetKind::Expression
                        },
                    )
                }
                AnyVueDirective::VueVBindShorthandDirective(directive) => {
                    (directive.initializer(), SnippetKind::Expression)
                }
                AnyVueDirective::VueVOnShorthandDirective(directive) => {
                    (directive.initializer(), SnippetKind::EventHandler)
                }
                AnyVueDirective::VueVSlotShorthandDirective(directive) => {
                    (directive.initializer(), SnippetKind::SlotProps)
                }
                AnyVueDirective::VueBogusDirective(_) => continue,
            };
            let Some(AnyHtmlAttributeInitializer::HtmlString(string)) =
                initializer.and_then(|initializer| initializer.value().ok())
            else {
                continue;
            };
            let (Ok(text), Ok(range)) = (string.inner_string_text(), string.inner_string_range())
            else {
                continue;
            };
            found.push((
                text.text().to_string(),
                range,
                range.start(),
                vue_kind(kind),
            ));
        }
    }

    let base = if is_ts {
        JsFileSource::ts()
    } else {
        JsFileSource::js_module()
    };
    let parsed: Vec<_> = found
        .iter()
        .map(|(text, _, _, kind)| {
            let source = base.with_embedding_kind(*kind);
            let root: AnyJsRoot =
                biome_js_parser::parse(text, source, JsParserOptions::default()).tree();
            let js = semantic_model(&root, SemanticModelOptions::from(&source));
            (root, js, source)
        })
        .collect();
    let snippets: Vec<_> = parsed
        .iter()
        .zip(&found)
        .map(|((root, js, source), (_, range, offset, _))| SfcSnippet {
            root,
            js,
            source: *source,
            content_range: *range,
            content_offset: *offset,
        })
        .collect();
    sfc_model(&host, &snippets)
}
